# Community Feature – State-Management Design  
_Revision: 2025-07-25_

This document specifies how we manage UI state for Community-related features in the CPC desktop / web app.  
It links the GraphQL operations layer (`community_operations.rs`), Tauri commands, Yew / yewdux stores, and (optionally) Bevy ECS resources.

---

## 1. Architectural Principles

* Hexagonal & Screaming Architecture — state slices are named after the business concepts (`CommunityList`, `CommunityDetail`, …).
* Vertical Slices — each feature owns its state, mutations, subscriptions, and UI components.
* Rust-first, no hidden JavaScript logic.  
* Stores use only permissive crates: `yew`, `yewdux`, `tokio`, `tracing`.
* Asynchronous boundary is **only** at the GraphQL operations or Tauri command layer.

---

## 2. Core State Types

```rust
/// Generic resource wrapper for async UI data
pub enum Resource<T> {
    Idle,
    Loading,
    Ready(T),
    Error(String),          // GraphQL or transport error
}

/// Normalised cache keyed by IDs
pub type CommunityCache = HashMap<CommunityId, Arc<Community>>;
```

### 2.1 CommunityListState

```rust
pub struct CommunityListState {
    pub communities: Vec<CommunityId>,        // ordered list of IDs
    pub cursor: Option<String>,               // pagination cursor
    pub is_end: bool,
    pub status: Resource<()>,                 // tracks loading/error
}
```

### 2.2 CommunityDetailState

```rust
pub struct CommunityDetailState {
    pub id: CommunityId,
    pub community: Resource<Arc<Community>>,  // pulls from global cache
    pub threads_cursor: Option<String>,
    pub is_end: bool,
    pub status: Resource<()>,
}
```

### 2.3 CommunityCreationState

```rust
pub struct CommunityCreationState {
    pub draft: CommunityDraft,          // name, description, avatar…
    pub status: Resource<CommunityId>,  // Ready holds new community id
}
```

### 2.4 MembershipState

```rust
pub struct MembershipState {
    pub membership: HashMap<CommunityId, MembershipRole>,
    pub status: Resource<()>,
}
```

### 2.5 ForumCache (Global)

```rust
pub struct ForumCache {
    pub communities: CommunityCache,
    /// LRU head for paged lists (configurable capacity)
    pub lru: lru::LruCache<CommunityId, ()>,
}
```

All slice stores hold an `Rc<RefCell<ForumCache>>` to dereference entities quickly and keep them unified.

---

## 3. yewdux Store Layout

```
forum/
├─ store/
│  ├─ cache.rs              ForumCacheStore  (global)
│  ├─ community_list.rs     CommunityListStore
│  ├─ community_detail.rs   CommunityDetailStore
│  ├─ community_create.rs   CommunityCreateStore
│  └─ membership.rs         MembershipStore
```

Each store implements:

```rust
impl Store for CommunityListStore {
    type Model = CommunityListState;
    fn new() -> Self { … }
}
```

Reducers are expressed as:

```rust
pub enum CommunityListMsg {
    FetchFirstPage,
    FetchNextPage,
    ReceivePage { items: Vec<Community>, cursor: Option<String> },
    Error(String),
}
```

---

## 4. Loading / Error Handling

* `Resource::Loading` triggers a spinner overlay.
* `Resource::Error(msg)` shows a toast + retry button that dispatches the same fetch message.
* Stores reset `status` to `Idle` after a successful mutation or on navigation away.

---

## 5. Pagination

Lists keep a `cursor` and `is_end` flag.  
When the scroll-observer hits 90 % of viewport, the component dispatches `FetchNextPage` if:

```
!state.is_end && matches!(state.status, Resource::Idle | Resource::Ready(_))
```

The GraphQL operations layer provides:

```rust
communities_page(cursor: Option<String>, limit: u32 = 25)
```

Responses update the global cache then push IDs into `communities`.

---

## 6. Real-time Updates

`community_updates` GraphQL subscription delivers:

```rust
type CommunityUpdate =
  | CommunityCreated(Community)
  | CommunityUpdated(Community)
  | MemberJoined { community_id, user_id, role }
  | MemberLeft   { community_id, user_id }
```

Flow:

1. `community_operations.rs` holds a Tokio `broadcast::Sender<CommunityUpdate>`.
2. The Tauri backend spawns the subscription task on app start.
3. Each slice store listens on the broadcast receiver, diffs entities against the cache, and applies updates (using yewdux `link.send_message`).

Consistency strategy = “last-write wins w/ version field”.

---

## 7. Caching Strategy

* Normalised `HashMap<CommunityId, Arc<Community>>`
* LRU keeps hottest 1 000 records in memory (configurable).
* Entities get overwritten on every update; no field-level merge yet.
* Optimistic updates:

```rust
// On mutation request:
cache.insert(temp_id, draft.into());
dispatch PendingCreated(temp_id);

// On server success:
cache.remove(temp_id);
cache.insert(real_id, entity);
reconcile list IDs
```

Rollback on error simply drops the `temp_id` and shows toast.

---

## 8. UI ↔ Operations Interaction

```
┌──────────┐   Msg      ┌───────────┐  invoke   ┌─────────────┐
│ Yew View │ ───────▶  │ yewdux    │ ─────────▶ │ Tauri Cmds  │
└──────────┘           │ Store     │            │ (Rust)      │
      ▲                └───────────┘◀───────────┘
      │       broadcast updates          ▲
      │                                   │  gRPC (internal)
      └───────────── subscriptions  ◀────┘
```

Commands live in `tauri_commands/forum.rs` and delegate to `community_operations::*`.

---

## 9. Bevy ECS Integration

When Bevy-based editors are active (= feature `with_bevy`):

* A Bevy system starts with:

```rust
#[derive(Resource)]
pub struct BevyCommunityCache(pub Arc<ForumCache>);
```

* Another system listens for `CommunityUpdateEvent` and mutates Bevy resources.
* UI slices send `CommunityJoined` etc. through a Bevy `EventWriter`.

This keeps gameplay/editor layers aware of the same data without tight coupling.

---

## 10. Component Responsibilities

| Component                              | Reads Store(s)                 | Dispatches Msgs                                 |
|----------------------------------------|--------------------------------|-------------------------------------------------|
| `CommunityBrowserPage`                 | `CommunityListStore`           | `FetchFirstPage`, `FetchNextPage`               |
| `CommunityCard`                        | `ForumCacheStore`              | —                                               |
| `CommunityDetailPage`                  | `CommunityDetailStore`         | `LoadDetail`, `Join`, `Leave`                   |
| `CreateCommunityModal`                 | `CommunityCreateStore`         | `UpdateDraft`, `Submit`                         |
| `MembershipBadge`                      | `MembershipStore`              | `ChangeRole`, `Leave`                           |

---

## 11. Error Surfaces

* GraphQL errors → user-visible toast.
* Transport errors → auto-retry exponential backoff (max 3).
* Permission errors → redirect to login or show “forbidden” banner.

---

## 12. Future Work

* Offline support with persistence to `indexed_db` (web) or `sled` (desktop).
* Fine-grained cache eviction (by community ACL).
* Subscription resumability on reconnect.

---

## 13. Glossary

* **Slice** – A vertical feature boundary owning its state/reducers.
* **Resource<T>** – Enum capturing async lifecycle.
* **Subscription** – Long-lived channel for server-pushed updates.

---

### End of File