# Forum UI Components – Architectural Design  
_CPC Platform • July 2025_

## 0. Principles
• Hexagonal & Screaming architecture – the top‐level folder name (`forum`) immediately tells us the feature.  
• Vertical slice per bounded context – every component owns its UI, GraphQL glue, local state hooks and tests.  
• Primary language: **Rust** (Yew) – Svelte code will remain until each slice is ported.  
• Public API: GraphQL Mutations + Subscriptions.  
• Internal service bus: gRPC Server Streaming.  
• No external clouds; p2panda handles sync.

Directory sketch:

```
apps/cpc-platform/src/forum/
├── community_browser/          # Community listings
├── thread_composer/            # Create new thread / reply
├── comment_tree/               # Nested comment view
├── voting_widget/              # Up/Down vote & karma
├── mod_tools/                  # Moderator actions
└── community_dashboard/        # Admin & analytics
```

Every slice contains  
`ui.rs` (Yew view) • `graphql.rs` (queries & mutations) • `hooks.rs` (state helpers) • `mod.rs` (re-exports).

---

## 1  Component Specifications

### 1.1 CommunityBrowser
Purpose: browse/join communities.

UI states  
1. Loading (skeleton)  
2. List of communities (name • avatar • member-count • join btn)  
3. Search/Filter bar  
4. “Create Community” (if perms)

Key interactions  
• Select community → route `/c/{slug}` → Thread list.  
• Join/Leave toggles membership.

GraphQL
```
query Communities($filter: CommunityFilter, $cursor: Cursor) { ... }
mutation JoinCommunity($id: ID!)
mutation LeaveCommunity($id: ID!)
subscription CommunityCreated { ... }
```

### 1.2 ThreadComposer
• Rich-text editor (reuse `lib/components/RichTextEditor`).  
• Attachment picker (media service).  
• Anonymous/alias toggle (future).  
• On submit emits `createThread` or `createComment` depending on context.

GraphQL
```
mutation CreateThread($input: NewThreadInput!)
mutation CreateComment($input: NewCommentInput!)
```

### 1.3 CommentTree
Nested lazy-loaded comments (max depth 7).  Uses tail-recursion + incremental load (`loadMore`).

Props: `thread_id: ID`, `collapsed: bool`.

GraphQL
```
query Comments($thread: ID!, $cursor: Cursor) { ... }
subscription CommentAdded($thread: ID!) { ... }
subscription CommentRemoved($thread: ID!) { ... }
```

### 1.4 VotingWidget
Small inline component reused by Thread cards and Comment nodes.

State: local optimistic diff until server ack.

GraphQL
```
mutation Vote($target: ID!, $value: Int!)      # value ∈ {1, -1, 0}
subscription VoteTally($target: ID!) { ... }   # push updated karma
```

### 1.5 ModTools
Visible to users with `Role::Moderator`.

Actions  
• Remove thread / comment (soft-delete)  
• Pin / Lock thread  
• Ban user (duration)  
• View reports queue

GraphQL
```
mutation RemovePost($id: ID!, $reason: String)
mutation BanUser($user: ID!, $until: DateTime)
mutation PinThread($id: ID!, $pinned: Boolean)
query ReportQueue($cursor: Cursor) { ... }
```

### 1.6 CommunityDashboard
Community owners & mods manage settings + analytics.

Sections  
1. Settings (description, rules, flair palette)  
2. Members & roles  
3. Auto-moderation rules (regex, rate-limits)  
4. Analytics charts (posts/day, active users, retention)

Graphs use existing `lib/bi` widgets via adapter.

---

## 2  UI State Management

We adopt **yewdux** (Redux-like) for cross-slice global state:

```
store/
├── auth.rs
├── forum.rs          # communities, threads, votes cache
└── routing.rs
```

• Components use `use_selector` to minimise re-renders.  
• GraphQL responses are normalised into the store (similar to Apollo).  
• Optimistic updates handled in action creators.  

Per-slice hooks (in `hooks.rs`) encapsulate queries + dispatches, e.g.:

```rust
pub fn use_thread(id: ThreadId) -> UseAsync<Thread> { ... }
pub fn use_vote(target: VoteTarget) -> (i32, Callback<i32>) { ... }
```

---

## 3  GraphQL Schema Additions (Rust pseudo-code)

```rust
#[derive(SimpleObject)]
struct Community { id: ID, name: String, slug: String, about: String,
                   created_at: DateTime, member_count: i64, avatar: Url }

#[derive(InputObject)]
struct NewThreadInput { community_id: ID, title: String, body: String }

extend type Query {
    communities(filter: CommunityFilter, cursor: Cursor): CommunityConnection!
    threadsByCommunity(id: ID!, cursor: Cursor): ThreadConnection!
    commentsByThread(id: ID!, cursor: Cursor): CommentConnection!
}

extend type Mutation {
    createCommunity(name: String!, about: String): Community!
    createThread(input: NewThreadInput!): Thread!
    createComment(input: NewCommentInput!): Comment!
    vote(target: ID!, value: Int!): VoteTally!
    banUser(user: ID!, until: DateTime): Ban!
}

extend type Subscription {
    communityCreated: Community!
    newThreadInCommunity(id: ID!): Thread!
    newCommentInThread(id: ID!): Comment!
    voteUpdated(target: ID!): VoteTally!
}
```

Resolvers will live in `apps/backend/src/graphql/forum/`.

---

## 4  Wireframes (ASCII)

### Community Browser
```
+----------------------------------------------------+
|  🔍 Search [ ________ ]   [+ Create]               |
|----------------------------------------------------|
| • @FarmingCoop         3.2k members  [Join]        |
| • @LocalMakers         1.1k members  [Joined ✓]    |
| • @AltEnergy           812 members   [Join]        |
+----------------------------------------------------+
```

### Thread View (inside /c/slug)
```
[ Back ]   FarmingCoop

( ThreadComposer )

------------------------------------------------------
⬆️12 ⬇️ | Title of thread here
by alice • 2h ago
------------------------------------------------------
  ⬆️3 ⬇️ Comment text …  (Reply)
    ⬆️0 ⬇️ Nested reply …  (Reply)
        … (load more) …
```

### Community Dashboard (owner)
```
[General] [Members] [Rules] [Analytics]

General ▼
Name:  [ FarmingCoop ]          Save
About: [ textarea          ]    Cancel
… etc …
```

---

## 5  Migration & Tasks

1. Backend  
   • Generate `forum` module under `apps/backend/src/graphql/`.  
   • Add objects, connections, resolvers; merge into `schema.rs`.

2. Frontend  
   • Scaffold folders in `apps/cpc-platform/src/forum`.  
   • Implement Yew components progressively; temporary Svelte wrappers map to new GraphQL endpoints.

3. State layer  
   • Introduce `forum.rs` store; adapt global provider in `apps/cpc-platform/src/main.rs`.

4. Replace old social feed voting widget once stable.

---

## 6  Risks / Mitigations
• Deep comment trees → use pagination + virtualised list to avoid DOM bloat.  
• Optimistic voting conflicting with server → server returns authoritative tally; UI reconciles.  
• Permission checks client-side but enforced server-side to prevent spoofing.

---

_Designed by Elias Petrova • ougarchitect_