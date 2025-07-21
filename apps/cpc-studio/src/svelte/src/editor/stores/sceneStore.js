import { writable, derived } from 'svelte/store';
import { listen } from '@tauri-apps/api/event';

// Create writable stores for scene data
import { v4 as uuidv4 } from 'uuid';
import { userStore } from './userStore';

export const scene = writable({
    entities: {},
    rootEntities: [],
    locks: {},
    versionVector: {}
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

// Function to add a new entity
export function addEntity(entityId, parentId = null) {
    scene.update(currentScene => {
        // Create new entity
        const { currentUser } = get(userStore);
        const now = new Date().toISOString();
        
        const newEntity = {
            id: entityId,
            name: `Entity ${entityId.substring(0, 4)}`,
            components: {},
            meta: {
                createdBy: currentUser.id,
                createdAt: now,
                lastModified: now,
                version: 1
            },
            children: [],
            parent: parentId
        };
        
        // Add to entities map
        const entities = {
            ...currentScene.entities,
            [entityId]: newEntity
        };
        
        // Update hierarchy
        let rootEntities = [...currentScene.rootEntities];
        if (parentId) {
            // Add as child to parent
            const parent = entities[parentId];
            if (parent) {
                entities[parentId] = {
                    ...parent,
                    children: [...(parent.children || []), entityId]
                };
            }
        } else {
            // Add to root entities
            rootEntities = [...rootEntities, entityId];
        }
        
        // Update version vector
        const versionVector = { ...currentScene.versionVector };
        versionVector[entityId] = 1;
        
        return {
            ...currentScene,
            entities,
            rootEntities,
            versionVector
        };
    });
}

// Function to remove an entity
export function removeEntity(entityId) {
    scene.update(currentScene => {
        const entities = { ...currentScene.entities };
        const rootEntities = [...currentScene.rootEntities];
        
        // Remove from parent's children or root entities
        const entity = entities[entityId];
        if (entity) {
            // Remove from parent if exists
            if (entity.parent) {
                const parent = entities[entity.parent];
                if (parent) {
                    entities[entity.parent] = {
                        ...parent,
                        children: parent.children.filter(id => id !== entityId)
                    };
                }
            } else {
                // Remove from root entities
                const index = rootEntities.indexOf(entityId);
                if (index !== -1) {
                    rootEntities.splice(index, 1);
                }
            }
            
            // Remove the entity itself
            delete entities[entityId];
        }
        
        return {
            ...currentScene,
            entities,
            rootEntities
        };
    });
}

// Function to reparent an entity
export function reparentEntity(entityId, newParentId) {
    scene.update(currentScene => {
        const entities = { ...currentScene.entities };
        const rootEntities = [...currentScene.rootEntities];
        const entity = entities[entityId];
        
        if (!entity) return currentScene;
        
        // Conflict detection
        const remoteVersion = currentScene.versionVector[entityId] || 0;
        if (entity.meta.version !== remoteVersion) {
            console.warn(`Conflict detected for entity ${entityId}. Local version: ${entity.meta.version}, Remote version: ${remoteVersion}`);
            // Resolve conflict by merging changes or prompting user
            // For now, just update to remote version
            entities[entityId] = { ...entity, ...currentScene.entities[entityId] };
            return currentScene;
        }
        
        // Remove from current parent
        if (entity.parent) {
            const oldParent = entities[entity.parent];
            if (oldParent) {
                entities[entity.parent] = {
                    ...oldParent,
                    children: oldParent.children.filter(id => id !== entityId)
                };
            }
        } else {
            // Remove from root entities
            const index = rootEntities.indexOf(entityId);
            if (index !== -1) {
                rootEntities.splice(index, 1);
            }
        }
        
        // Add to new parent
        if (newParentId) {
            const newParent = entities[newParentId];
            if (newParent) {
                entities[newParentId] = {
                    ...newParent,
                    children: [...(newParent.children || []), entityId]
                };
            }
            // Update entity's parent reference and metadata
            entities[entityId] = {
                ...entity,
                parent: newParentId,
                meta: {
                    ...entity.meta,
                    lastModified: new Date().toISOString(),
                    version: entity.meta.version + 1
                }
            };
        } else {
            // Add to root entities
            rootEntities.push(entityId);
            // Update entity's parent reference and metadata
            entities[entityId] = {
                ...entity,
                parent: null,
                meta: {
                    ...entity.meta,
                    lastModified: new Date().toISOString(),
                    version: entity.meta.version + 1
                }
            };
        }
        
        // Update version vector
        const versionVector = { ...currentScene.versionVector };
        versionVector[entityId] = entities[entityId].meta.version;
        
        return {
            ...currentScene,
            entities,
            rootEntities,
            versionVector
        };
    });
}

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