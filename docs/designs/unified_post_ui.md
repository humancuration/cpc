# Unified Post Creation Interface (PostComposer)
author: Elias Petrova  
date: 2025-07-25  

## 0. TL;DR
`PostComposer` is a vertical slice (Hexagonal + Screaming Architecture) that lets a user craft either  
* a **Social Post** (timeline-style) or  
* a **Forum Thread** (topic-style)  
from the same screen.  
The slice contains:
1. A **pure-Rust editor core** reused by both the Desktop (Tauri), Web (Yew/WASM), and Android/iOS wrappers.  
2. Yew UI components that wrap the core into an ergonomic UX.  
3. GraphQL mutations + subscriptions that bridge UI ↔ backend.  
4. A live preview pane that can toggle between “Social” and “Forum” render modes.

```
crate  apps/cpc-platform/src/post_composer/  ← Yew front-end
crate  packages/cpc-core/src/post_composer/ ← Editor + domain logic
crate  apps/backend/src/graphql/post_composer.rs
```

---

## 1. Component hierarchy (Yew)

```mermaid
flowchart TD
  %% UI Layer
  subgraph Yew UI Components
    PC[PostComposer]
    TS[TypeSelector]
    FS[ForumSelector]
    ED[RichTextEditor]
    MU[MediaUploader]
    PV[PreviewPane]
    VS[VisibilitySelector]
  end

  %% Domain Logic
  subgraph Core (Rust no_std friendly)
    EC[EditorCore]
    PD[PostDraft]
    PM[PostManager]
  end

  %% Ports / Adapters
  subgraph Ports
    GQL[GraphQLPort]
    MEDIA[MediaPort]
    STORAGE[DraftStoragePort]
  end

  %% wire-up
  PC --> TS
  PC --> VS
  PC --> ED
  PC --> MU
  PC --> FS
  PC --> PV

  ED ---> EC
  PC -.save draft .-> STORAGE
  MU --> MEDIA
  PC -->|submit| GQL
  GQL --> PM
  MEDIA --> PM
  STORAGE --> PM
  PM --> EC
```

### Folder Layout (vertical slice)

```
post_composer/
├── mod.rs              # Yew root component + router entry
├── components/
│   ├── type_selector.rs
│   ├── forum_selector.rs
│   ├── rich_text_editor.rs
│   ├── media_uploader.rs
│   └── preview_pane.rs
├── state.rs            # Reducer / Context objects
└── services/
    ├── gql.rs          # GraphQLPort impl
    ├── media.rs        # MediaPort impl
    └── draft_store.rs  # localStorage / sqlite-kv adapter
```

---

## 2. State management flow

We keep **all UI state inside a single `use_reducer` store** shared through context `PostComposerCtx`.

```rust
pub enum ComposerMode { Social, Forum }

pub struct ComposerState {
    mode: ComposerMode,
    community_id: Option<Uuid>,        // forum only
    visibility: Visibility,            // social only
    content: String,
    media: Vec<MediaItem>,
    draft_id: Option<String>,          // localStorage key
    is_posting: bool,
}

pub enum ComposerAction {
    SetMode(ComposerMode),
    SetCommunity(Uuid),
    UpdateContent(String),
    AttachMedia(Vec<MediaItem>),
    RemoveMedia(usize),
    SetVisibility(Visibility),
    PostStarted,
    PostSuccess(PostId),
    PostError(String),
    LoadDraft(Draft),
    Clear,
}
```

* Reducer lives in `state.rs`.  
* Side-effects (GraphQL mutation, media upload, saving draft) are handled by service functions that dispatch follow-up actions.

---

## 3. GraphQL interface

### 3.1 Shared input object

```graphql
"""Common post payload"""
input PostContentInput {
  content: String!
  mediaIds: [UUID!]!    # completed media only
}
```

### 3.2 Social post mutation

```graphql
mutation CreateSocialPost($input: CreateSocialPostInput!) {
  createSocialPost(input: $input) {
    id
    ... PostFragments
  }
}

input CreateSocialPostInput {
  cooperativeId: UUID      # optional coop
  visibility: Visibility!
  data: PostContentInput!
}
```

### 3.3 Forum thread mutation

```graphql
mutation CreateForumThread($input: CreateForumThreadInput!) {
  createForumThread(input: $input) {
    threadId
    slug
    firstPost { id }
  }
}

input CreateForumThreadInput {
  communityId: UUID!       # selected forum
  title: String!
  data: PostContentInput!
}
```

### 3.4 Subscriptions

A single generic stream is added so the UI updates while media finishes processing:

```graphql
subscription PostStatus($postIds: [UUID!]!) {
  postStatus(ids: $postIds) {
    id
    processingState   # queued | processing | ready | failed
  }
}
```

---

## 4. Editor core & rich-text requirements

| Feature                      | Technique / crate                                  |
|------------------------------|----------------------------------------------------|
| Markdown → HTML             | `pulldown-cmark` inside `EditorCore::to_html()`    |
| Syntax extensions           | GFM tables / strikethrough enabled via `cmark` opts|
| Embed images / video / audio| Insert placeholder tag: `![alt](cid:media_uuid)`   |
| Link preview                | On paste: fetch `OpenGraph` in WebWorker, dispatch |
| Keyboard shortcuts          | **Ctrl+B**, **Ctrl+I**, **Ctrl+K** for link        |
| Max length                  | enforced in core + UI                              |
| Merkle-hash draft id        | `blake3` hash of content for offline caching       |

`RichTextEditor` Yew component wraps a `<textarea>` + toolbar (mirrors current Svelte) but delegates all formatting transforms to `EditorCore`.

---

## 5. Media support

* Uploader uses existing `MediaUploader` service (QUIC upload → backend).
* After successful upload backend returns `mediaId` and presigned CDN URL.  
* PostComposer stores provisional media list and waits for `processingState=='ready'`.  
* Only READY items are sent in `mediaIds`.

---

## 6. Preview rendering

The right pane (`PreviewPane`) simply:

```rust
let html = EditorCore::to_html(&state.content, &state.media);
match state.mode {
    ComposerMode::Social => html_into_social_card(html),
    ComposerMode::Forum  => html_into_forum_post(html),
}
```

Where `*_card/post()` are small Yew components that apply the respective layout & CSS.

A tab bar above the preview toggles between Social-view and Forum-view on the fly.

---

## 7. Accessibility & mobile

* All buttons have ARIA labels.  
* Editor supports `prefers-color-scheme`.  
* Mobile layout collapses sidebar; toolbar becomes bottom-sheet.

---

## 8. Migration plan

1. Copy current Svelte `PostComposer` logic into new Rust _spec_ tests to ensure parity.  
2. Implement `EditorCore` in `cpc-core`.  
3. Build Yew components under feature flag `post_composer_v2`.  
4. Expose GraphQL mutations in backend (`post_composer.rs`) and wire into root schema.  
5. Transition routes: `routes/social/+page.svelte` loads WASM component via `<PostComposerWasm />`.  
6. Remove Svelte code after parity & perf checks.

---

## 9. Open questions

* Forum category hierarchy: do we need sub-forums selector?  
* Attachment ordering & captions?  
* Server-side sanitisation policy for Markdown/HTML.