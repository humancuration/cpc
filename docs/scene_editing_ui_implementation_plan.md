# Scene Editing UI Implementation Plan

This document outlines the technical plan for implementing the Scene Hierarchy and editing features within CPC Studio.

## 1. Overview

The goal is to create a scene hierarchy panel that displays a tree view of entities in the current scene. Users will be able to select, create, delete, duplicate, and reparent entities. These actions will be synchronized in real-time for all collaborators.

## 2. Frontend Implementation (Yew)

### 2.1. New Component: `SceneHierarchyPanel.svelte`

**(Location: `apps/cpc-studio/src/svelte/src/editor/components/SceneHierarchyPanel.svelte`)**

This component will be the primary interface for scene manipulation.

**Features:**

*   **Tree View:** Display a recursive, collapsible tree of scene entities.
*   **Selection:**
    *   Single-click to select an entity.
    *   `Ctrl+Click` for multi-selection.
    *   `Shift+Click` for range selection.
*   **Drag-and-Drop:**
    *   Allow re-parenting entities by dragging them onto other entities.
    *   Dragging to the root level should also be possible.
*   **Context Menu (Right-click):**
    *   Create Entity (prompts for type, e.g., Empty, Mesh, Light)
    *   Delete Selected Entity/Entities
    *   Duplicate Selected Entity/Entities
*   **Keyboard Shortcuts:**
    *   `Shift+A`: Open "Create Entity" context menu at the selected entity's level.
    *   `X` or `Delete`: Delete selected entities.
    *   `Shift+D`: Duplicate selected entities.
*   **Entity Locking:** Visually indicate when an entity is locked by another collaborator (e.g., a lock icon).

### 2.2. New Store: `sceneStore.js`

**(Location: `apps/cpc-studio/src/svelte/src/editor/stores/sceneStore.js`)**

This store will manage the state of the scene on the frontend. It will be similar in function to `assetStore.js`.

**State:**

*   `sceneHierarchy`: A `writable` store holding the nested tree structure of entities.
    *   Example structure: `[{ id: 'uuid-1', name: 'Player', children: [...] }, ...]`
*   `selectedEntities`: A `writable` store containing a `Set` of selected entity IDs.

**Functions:**

*   `loadScene()`: Fetches the initial scene hierarchy from the Rust backend using `invoke('get_scene_hierarchy')`.
*   `createEntity(parentId)`: Calls the `create_entity` Tauri command.
*   `deleteEntities(entityIds)`: Calls the `delete_entities` Tauri command.
*   `reparentEntities(draggedEntityIds, newParentId)`: Calls the `reparent_entities` command.
*   Event listeners for real-time updates from the backend (`scene-updated`, `entity-locked`, `entity-unlocked`).

### 2.3. Layout Modification: `+page.svelte`

**(Location: `apps/cpc-studio/src/svelte/src/editor/+page.svelte`)**

The main editor layout needs to be updated to include the `SceneHierarchyPanel`.

*   Modify the `.editor-layout` grid to have a split left column.
*   The top half will be the `AssetBrowser`.
*   The bottom half will be the new `SceneHierarchyPanel`.
*   A draggable splitter should be implemented between them for resizing.

```html
<!-- Suggested structure in +page.svelte -->
<div class="left-panel">
    <div class="asset-browser-container">
        <AssetBrowser />
    </div>
    <div class="scene-hierarchy-container">
        <SceneHierarchyPanel />
    </div>
</div>
```

## 3. Backend Implementation (Rust/Tauri)

### 3.1. New Tauri Commands

The following commands need to be added to the Tauri command handler in `src-tauri/src/commands.rs` (or a relevant module).

*   `#[tauri::command]
  async fn get_scene_hierarchy(state: tauri::State<'_, AppState>) -> Result<Vec<EntityNode>, ()>`
    *   Returns the entire scene graph as a nested structure.

*   `#[tauri::command]
  async fn create_entity(parent: Option<Uuid>, state: tauri::State<'_, AppState>) -> Result<Uuid, ()>`
    *   Creates a new entity, optionally parented to another.
    *   Returns the new entity's ID.
    *   Broadcasts a `scene-updated` event to all clients.

*   `#[tauri::command]
  async fn delete_entities(ids: Vec<Uuid>, state: tauri::State<'_, AppState>) -> Result<(), ()>`
    *   Deletes the specified entities and their children.
    *   Broadcasts a `scene-updated` event.

*   `#[tauri::command]
  async fn reparent_entities(entity_map: HashMap<Uuid, Option<Uuid>>, state: tauri::State<'_, AppState>) -> Result<(), ()>`
    *   Takes a map of entity IDs to their new parent ID (`None` for root).
    *   Updates the scene graph.
    *   Broadcasts a `scene-updated` event.

### 3.2. Entity State Management & Locking

This will mirror the existing asset management system found in `asset_sync.rs`.

*   **`SceneSync` struct:** A new struct, `SceneSync`, should be created, analogous to `AssetSync`. It will manage the scene state, entity locks, and network synchronization.
*   **Entity Locking:**
    *   Implement `acquire_entity_lock(entity_id, user_id)` and `release_entity_lock(entity_id, user_id)` functions.
    *   When a user selects an entity, a lock should be acquired.
    *   The lock prevents other users from modifying the entity (e.g., moving, deleting, changing components).
    *   The backend will broadcast `entity-locked` and `entity-unlocked` events to all clients, containing the `entity_id` and `user_info`.
*   **Bevy Integration:** The `SceneSync` system will need to be added to the Bevy app to interact with the ECS (Entity Component System) and respond to changes.

## 4. Real-Time Collaboration

*   **WebSockets:** The existing WebSocket infrastructure will be used to pass events.
*   **New Events:**
    *   `scene-updated`: Sent when the hierarchy changes (create, delete, reparent). The payload should contain the updated scene hierarchy or a delta. A full resync is acceptable for the initial implementation.
    *   `entity-locked`: Sent when an entity is locked. Payload: `{ entity_id: Uuid, user: CollaboratorInfo }`.
    *   `entity-unlocked`: Sent when a lock is released. Payload: `{ entity_id: Uuid }`.

## 5. Task Breakdown for `ougcode`

1.  **Backend First:**
    *   Create the `SceneSync` struct and its core logic for managing the scene graph in memory.
    *   Implement the four new Tauri commands (`get_scene_hierarchy`, `create_entity`, `delete_entities`, `reparent_entities`).
    *   Implement the entity locking mechanism (`acquire_entity_lock`, `release_entity_lock`).
    *   Define and broadcast the new WebSocket events.
2.  **Frontend Integration:**
    *   Create the `sceneStore.js` file to manage state.
    *   Implement the `SceneHierarchyPanel.svelte` component with basic tree display and selection.
    *   Connect the store to the backend commands to fetch and display the scene.
    *   Implement the create, delete, and duplicate context menu actions.
    *   Implement drag-and-drop for reparenting.
    *   Add visual feedback for locked entities.
3.  **Layout:**
    *   Update `+page.svelte` to include the new panel in the layout.