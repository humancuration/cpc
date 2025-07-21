# Backend Command System: Comprehensive Test Plan

## 1. Introduction

This document outlines the test plan for the backend scene command system in CPC Studio. The primary goal is to ensure the reliability, correctness, and robustness of all scene operations, including entity lifecycle, component manipulation, and hierarchy changes.

A key observation during the creation of this plan is a **discrepancy between the intended architecture and the current implementation**. The `SceneManager` correctly holds a `HierarchyGraph`, but several commands (`DeleteEntitiesCommand`, `ReparentEntitiesCommand`) attempt to interact with fields that do not exist on `HierarchyGraph`.

**This test plan is written against the intended architecture.** The tests specified herein assume that commands will correctly use the methods provided by `SceneManager` and `HierarchyGraph` (`add_entity`, `remove_entity`, `reparent_entity`, etc.). Executing these tests will validate the correct implementation once the code is brought in line with the architectural plan.

## 2. Test Utilities & Setup

To facilitate testing, a set of test utilities will be required. These are not part of the production code but are essential for creating a consistent test environment.

-   `setup_test_scene()`: Creates a new `SceneManager` instance for isolated testing.
-   `create_entity_with_parent(scene, parent_id)`: A helper to quickly add entities to the scene for testing.
-   `add_component_to_entity(scene, entity_id, component)`: A helper to set up component data.
-   `get_undo_stack_size(scene)`: Returns the number of commands in the undo stack.
-   `get_redo_stack_size(scene)`: Returns the number of commands in the redo stack.

---

## 3. Entity Lifecycle Tests

### 3.1. `CreateEntityCommand`

-   **Test Case 3.1.1: Create a root entity**
    -   **Preconditions:** An empty scene.
    -   **Steps:**
        1.  Execute a `CreateEntityCommand` with `parent_id: None`.
    -   **Expected Results:**
        1.  A new entity exists in `scene.entities`.
        2.  `scene.hierarchy` correctly registers the new entity with a `None` parent.
        3.  The undo stack contains one command.
        4.  Executing `undo` removes the entity from `scene.entities` and `scene.hierarchy`.

-   **Test Case 3.1.2: Create a child entity**
    -   **Preconditions:** A scene with an existing entity (`parent_entity`).
    -   **Steps:**
        1.  Execute `CreateEntityCommand` with `parent_id: Some(parent_entity.id)`.
    -   **Expected Results:**
        1.  A new entity exists in `scene.entities`.
        2.  `scene.hierarchy` shows the new entity as a child of `parent_entity`.
        3.  Executing `undo` removes the new entity and its relationship to `parent_entity`.

### 3.2. `DeleteEntitiesCommand`

-   **Test Case 3.2.1: Delete a single leaf entity**
    -   **Preconditions:** A scene with a parent and a child entity.
    -   **Steps:**
        1.  Execute `DeleteEntitiesCommand` targeting the child entity.
    -   **Expected Results:**
        1.  The child entity is removed from `scene.entities`.
        2.  The child entity is removed from `scene.hierarchy`.
        3.  The parent entity's list of children in the hierarchy no longer contains the child's ID.
        4.  Executing `undo` restores the entity and its hierarchical relationships.

-   **Test Case 3.2.2: Delete an entity with children**
    -   **Preconditions:** A scene with a grandparent, parent, and child entity.
    -   **Steps:**
        1.  Execute `DeleteEntitiesCommand` targeting the parent entity.
    -   **Expected Results:**
        1.  The parent entity and its descendants (the child) are removed from `scene.entities`.
        2.  The parent and child are removed from `scene.hierarchy`.
        3.  The grandparent's list of children no longer contains the parent's ID.
        4.  Executing `undo` restores the parent, the child, and all hierarchical relationships.

-   **Test Case 3.2.3: Delete multiple entities at once**
    -   **Preconditions:** A scene with several unrelated entities.
    -   **Steps:**
        1.  Execute `DeleteEntitiesCommand` with a list of multiple entity IDs.
    -   **Expected Results:**
        1.  All specified entities are removed.
        2.  Executing `undo` restores all removed entities.

---

## 4. Component Operation Tests

### 4.1. `AddComponentCommand`

-   **Test Case 4.1.1: Add a new component**
    -   **Preconditions:** An entity exists with no components.
    -   **Steps:**
        1.  Execute `AddComponentCommand` to add a `Transform` component with specific data.
    -   **Expected Results:**
        1.  The entity's `components` map now contains the new component type and data.
        2.  Executing `undo` removes the component from the entity.

### 4.2. `UpdateComponentCommand`

-   **Test Case 4.2.1: Update an existing component**
    -   **Preconditions:** An entity exists with a `Transform` component.
    -   **Steps:**
        1.  Execute `UpdateComponentCommand` to change the `Transform` data.
    -   **Expected Results:**
        1.  The component's value is updated.
        2.  Executing `undo` reverts the component's data to its original value.

### 4.3. `RemoveComponentCommand`

-   **Test Case 4.3.1: Remove an existing component**
    -   **Preconditions:** An entity exists with a `Transform` component.
    -   **Steps:**
        1.  Execute `RemoveComponentCommand` for the `Transform` component.
    -   **Expected Results:**
        1.  The component is removed from the entity's `components` map.
        2.  Executing `undo` restores the component with its original data.

---

## 5. Hierarchy Operations Tests

### 5.1. `ReparentEntitiesCommand`

-   **Test Case 5.1.1: Reparent an entity to a new parent**
    -   **Preconditions:** Scene with `entity_A`, `parent_1`, and `parent_2`. `entity_A` is a child of `parent_1`.
    -   **Steps:**
        1.  Execute `ReparentEntitiesCommand` to move `entity_A` to be a child of `parent_2`.
    -   **Expected Results:**
        1.  `scene.hierarchy` reflects that `parent_2` is now the parent of `entity_A`.
        2.  The old parent, `parent_1`, no longer lists `entity_A` as a child.
        3.  Executing `undo` restores `entity_A` as a child of `parent_1`.

-   **Test Case 5.1.2: Reparent an entity to the root**
    -   **Preconditions:** Scene with `entity_A` as a child of `parent_1`.
    -   **Steps:**
        1.  Execute `ReparentEntitiesCommand` to move `entity_A`, setting `new_parent_id` to `None`.
    -   **Expected Results:**
        1.  `scene.hierarchy` shows `entity_A` has no parent.
        2.  Executing `undo` restores `entity_A` as a child of `parent_1`.

-   **Test Case 5.1.3: Reparent multiple entities**
    -   **Preconditions:** `entity_A` and `entity_B` are children of `parent_1`.
    -   **Steps:**
        1.  Execute `ReparentEntitiesCommand` to move both `entity_A` and `entity_B` to `parent_2`.
    -   **Expected Results:**
        1.  Both entities are now children of `parent_2`.
        2.  Executing `undo` moves both entities back to `parent_1`.

-   **Test Case 5.1.4: Attempt to create a cyclical dependency (Error Case)**
    -   **Preconditions:** `entity_A` is a parent of `entity_B`.
    -   **Steps:**
        1.  Execute `ReparentEntitiesCommand` to make `entity_A` a child of `entity_B`.
    -   **Expected Results:**
        1.  The command should fail gracefully.
        2.  The hierarchy remains unchanged.
        3.  No command is added to the undo stack.

---

## 6. Undo/Redo System Tests

-   **Test Case 6.1.1: Basic Undo/Redo**
    -   **Preconditions:** An empty scene.
    -   **Steps:**
        1.  Execute a command (e.g., `CreateEntityCommand`).
        2.  Call `undo`.
        3.  Call `redo`.
    -   **Expected Results:**
        1.  After `undo`, the scene state is reverted. The undo stack is empty, redo stack has 1 item.
        2.  After `redo`, the command is re-applied. The undo stack has 1 item, redo stack is empty.

-   **Test Case 6.1.2: Multiple Undo/Redo**
    -   **Preconditions:** An empty scene.
    -   **Steps:**
        1.  Execute 3 commands.
        2.  Call `undo` twice.
        3.  Call `redo` once.
    -   **Expected Results:**
        1.  The undo stack should have 2 items, and the redo stack should have 1 item. The scene state should reflect that the first command is applied, but the second and third are not.

-   **Test Case 6.1.3: New command clears redo stack**
    -   **Preconditions:** A scene where a command has been executed and then undone.
    -   **Steps:**
        1.  Execute command A.
        2.  Call `undo`. (Redo stack now has command A).
        3.  Execute command B.
    -   **Expected Results:**
        1.  The redo stack is cleared. `get_redo_stack_size` returns 0.
        2.  Calling `redo` has no effect.

---

## 7. Error Handling Tests

-   **Test Case 7.1.1: Command with invalid Entity ID**
    -   **Preconditions:** An empty scene.
    -   **Steps:**
        1.  Execute `UpdateComponentCommand` with a random, non-existent UUID.
    -   **Expected Results:**
        1.  The command executes but has no effect on the scene state.
        2.  The command is still added to the undo stack (as it represents a "no-op"). *Alternatively, the command could return an error and not be pushed, this should be standardized.*

-   **Test Case 7.1.2: Reparenting to an invalid parent ID**
    -   **Preconditions:** A scene with one entity.
    -   **Steps:**
        1.  Execute `ReparentEntitiesCommand` with a random, non-existent parent UUID.
    -   **Expected Results:**
        1.  The command should fail gracefully, leaving the hierarchy unchanged.

---

## 8. Event Emission Tests

**Note:** This functionality is not yet implemented but is a requirement.

-   **Test Case 8.1.1: Verify event on command execution**
    -   **Preconditions:** A system is listening for a "command-executed" event from the backend.
    -   **Steps:**
        1.  Execute any command (e.g., `CreateEntityCommand`).
    -   **Expected Results:**
        1.  A "command-executed" event is emitted.
        2.  The event payload contains the command type (e.g., "CreateEntityCommand").
        3.  The payload contains the affected entity IDs.
        4.  The payload contains a snapshot of the updated scene hierarchy (or relevant parts).

-   **Test Case 8.1.2: Verify event on Undo/Redo**
    -   **Preconditions:** A system is listening for events.
    -   **Steps:**
        1.  Execute a command.
        2.  Call `undo`.
        3.  Call `redo`.
    -   **Expected Results:**
        1.  An event should be emitted after the initial execution, the undo, and the redo, each time reflecting the new state of the scene.