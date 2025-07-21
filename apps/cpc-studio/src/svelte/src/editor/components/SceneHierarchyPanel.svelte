<script>
import { onMount } from 'svelte';
import { toast } from '@zerodevx/svelte-toast';
import { scene, selectedEntity, selectEntity } from '../stores/sceneStore';
import SceneHierarchyEntity from './SceneHierarchyEntity.svelte';
import { dispatchCommand } from '../stores/commandDispatcher';

let contextMenu = {
    show: false,
    x: 0,
    y: 0,
    entity: null
};

// Load scene hierarchy
onMount(async () => {
    try {
        const hierarchy = await dispatchCommand('GetSceneHierarchy');
        scene.set(hierarchy);
    } catch (error) {
        toast.push(`Error loading scene: ${error.message}`, { theme: { '--toastColor': 'red' } });
    }
});

// Context menu handlers
function showContextMenu(event, entityId) {
    event.preventDefault();
    contextMenu = {
        show: true,
        x: event.clientX,
        y: event.clientY,
        entity: entityId
    };
    selectEntity(entityId);
}

function closeContextMenu() {
    contextMenu.show = false;
}

async function createEntity(parentId = null) {
    try {
        await dispatchCommand('CreateEntity', { parent: parentId });
    } catch (error) {
        toast.push(`Create entity failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
    }
}

async function deleteEntity(entityId) {
    try {
        await dispatchCommand('DeleteEntity', { entityId });
    } catch (error) {
        toast.push(`Delete failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
    }
}

async function duplicateEntity(entityId) {
    // TODO: Implement duplication
    toast.push('Duplication not implemented yet', { theme: { '--toastColor': 'orange' } });
}

// Handle drag and drop for reparenting
function handleDragStart(event, entityId) {
    event.dataTransfer.setData('text/plain', entityId);
}

async function handleDrop(event, parentId) {
    event.preventDefault();
    const entityId = event.dataTransfer.getData('text/plain');
    
    try {
        await dispatchCommand('ReparentEntity', {
            entityId,
            newParentId: parentId
        });
    } catch (error) {
        toast.push(`Reparent failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
    }
}

function handleDragOver(event) {
    event.preventDefault();
}
</script>

<div class="scene-hierarchy">
    <div class="toolbar">
        <button on:click={() => createEntity()}>Create Entity</button>
    </div>
    
    <div class="tree-view">
        {#if $scene}
            {#each Object.values($scene.entities) as entity}
                {#if !entity.parent}
                    <SceneHierarchyEntity
                        entity={entity}
                        depth={0}
                        on:select={e => selectEntity(e.detail)}
                        on:contextmenu={e => showContextMenu(e, entity.id)}
                        on:dragstart={e => handleDragStart(e, entity.id)}
                        on:drop={e => handleDrop(e, entity.id)}
                        on:dragover={handleDragOver}
                    />
                {/if}
            {/each}
        {/if}
    </div>
    
    {#if contextMenu.show}
        <div class="context-menu" style="left: {contextMenu.x}px; top: {contextMenu.y}px" on:mouseleave={closeContextMenu}>
            <div class="menu-item" on:click={() => createEntity(contextMenu.entity)}>Create Child</div>
            <div class="menu-item" on:click={() => duplicateEntity(contextMenu.entity)}>Duplicate</div>
            <div class="menu-divider"></div>
            <div class="menu-item" on:click={() => deleteEntity(contextMenu.entity)}>Delete</div>
        </div>
    {/if}
</div>

<style>
    .scene-hierarchy {
        height: 100%;
        border: 1px solid #ccc;
        padding: 10px;
        display: flex;
        flex-direction: column;
    }
    
    .toolbar {
        padding-bottom: 10px;
        border-bottom: 1px solid #eee;
    }
    
    .tree-view {
        flex: 1;
        overflow-y: auto;
        padding-top: 10px;
    }
    
    .context-menu {
        position: fixed;
        background: white;
        border: 1px solid #ccc;
        box-shadow: 2px 2px 5px rgba(0,0,0,0.2);
        z-index: 1000;
    }
    
    .menu-item {
        padding: 8px 16px;
        cursor: pointer;
    }
    
    .menu-item:hover {
        background-color: #f0f0f0;
    }
    
    .menu-divider {
        height: 1px;
        background-color: #eee;
        margin: 4px 0;
    }
</style>