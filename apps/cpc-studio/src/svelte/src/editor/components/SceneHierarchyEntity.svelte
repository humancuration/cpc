<script>
    import { fade } from 'svelte/transition';
    import { get } from 'svelte/store';
    import { scene, selectedEntity } from '../stores/sceneStore';
    
    export let entity;
    export let depth = 0;
    export let expanded = true;
    
    // Toggle entity expansion state
    function toggleExpand() {
        expanded = !expanded;
    }
    
    // Get entity name from components or use default
    $: name = entity.components.Name ? entity.components.Name.value : `Entity ${entity.id.substring(0, 4)}`;
    
    // Get icon based on component types
    $: icon = '📄'; // Default icon
    if (entity.components.Mesh) {
        icon = '📦';
    } else if (entity.components.Camera) {
        icon = '📷';
    } else if (entity.components.Light) {
        icon = '💡';
    }
    
    // Check if this entity is selected
    $: isSelected = get(selectedEntity) === entity.id;
    
    // Handle entity selection
    function selectEntity() {
        selectedEntity.set(entity.id);
    }
</script>

<div class="entity" 
     class:selected={isSelected}
     style={`--depth: ${depth};`}
     on:click={selectEntity}
     on:contextmenu>
    <div class="entity-header">
        {#if entity.children.length > 0}
            <button class="expand-toggle" on:click|stopPropagation={toggleExpand}>
                {#if expanded}▼{:else}►{/if}
            </button>
        {:else}
            <div class="expand-spacer"></div>
        {/if}
        
        <span class="icon">{icon}</span>
        <span class="name">{name}</span>
        
        <div class="drag-handle" draggable="true" on:dragstart on:dragend>
            ⠿
        </div>
    </div>
    
    {#if expanded}
        <div class="children" in:fade>
            {#each entity.children as childId (childId)}
                {#if $scene.entities[childId]}
                    <SceneHierarchyEntity 
                        entity={$scene.entities[childId]} 
                        depth={depth + 1} 
                        bind:expanded
                        on:contextmenu
                        on:dragstart
                        on:dragend
                        on:drop
                        on:dragover />
                {/if}
            {/each}
        </div>
    {/if}
</div>

<style>
    .entity {
        padding-left: calc(var(--depth) * 16px);
        border-left: 1px solid #eee;
        margin: 2px 0;
    }
    
    .entity-header {
        display: flex;
        align-items: center;
        padding: 4px 8px;
        border-radius: 4px;
        cursor: pointer;
        user-select: none;
    }
    
    .entity-header:hover {
        background-color: #f0f0f0;
    }
    
    .entity.selected .entity-header {
        background-color: #e0f0ff;
        font-weight: bold;
    }
    
    .expand-toggle, .expand-spacer {
        width: 20px;
        text-align: center;
        margin-right: 4px;
    }
    
    .icon {
        margin-right: 8px;
    }
    
    .name {
        flex-grow: 1;
    }
    
    .drag-handle {
        cursor: grab;
        opacity: 0.5;
        padding: 0 4px;
    }
    
    .drag-handle:hover {
        opacity: 1;
    }
    
    .children {
        margin-left: 16px;
        border-left: 1px dashed #eee;
    }
</style>