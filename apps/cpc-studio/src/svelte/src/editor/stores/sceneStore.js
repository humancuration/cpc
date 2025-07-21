import { writable, derived } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

// Create writable stores for scene data
export const scene = writable({
    entities: {},
    parent_child_pairs: [],
    locks: {}
});

export const selectedEntity = writable(null);

// Handle entity lock events
listen('entity-locked', (event) => {
    const { entity_id, user_id, user_name } = event.payload;
    scene.update(currentScene => ({
        ...currentScene,
        locks: {
            ...currentScene.locks,
            [entity_id]: { user_id, user_name }
        }
    }));
});

// Handle entity unlock events
listen('entity-unlocked', (event) => {
    const { entity_id } = event.payload;
    scene.update(currentScene => {
        const locks = { ...currentScene.locks };
        delete locks[entity_id];
        return {
            ...currentScene,
            locks
        };
    });
});

// Handle scene updates from commands
listen('command-executed', (event) => {
    if (event.payload.command_type === 'SceneCommand') {
        scene.set(event.payload.result.scene);
    }
});

// Function to select an entity
export function selectEntity(entityId) {
    selectedEntity.set(entityId);
}

// Get the currently selected entity
export const selectedEntityData = derived(
    [scene, selectedEntity],
    ([$scene, $selectedEntity]) => {
        if (!$selectedEntity || !$scene.entities[$selectedEntity]) {
            return null;
        }
        return $scene.entities[$selectedEntity];
    }
);