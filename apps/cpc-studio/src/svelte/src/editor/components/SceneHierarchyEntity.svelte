<script>
    import { fade } from 'svelte/transition';
    import { get } from 'svelte/store';
    import { scene, selectedEntity } from '../stores/sceneStore';
    import { userStore } from '../stores/userStore';
    
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
    
    // Get user color and initials
    $: user = get(userStore).users[entity.meta.createdBy] || { id: entity.meta.createdBy, name: 'Unknown' };
    $: userInitials = user.name ? user.name.substring(0, 2) : '??';
    $: userColor = user.color || '#cccccc';
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
        
        <div class="meta-info">
            <span class="user-badge" style="--user-color: {userColor}">
                {userInitials}
            </span>
            <span class="timestamp">
                {new Date(entity.meta.lastModified).toLocaleTimeString()}
            </span>
        </div>
        
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
        padding-left: calc(var(--depth) * 20px);
        border-left: 2px solid #e9ecef;
        margin: 4px 0;
        transition: background 0.2s ease;
    }
    
    .entity-header {
        display: flex;
        align-items: center;
        padding: 8px 12px;
        border-radius: 8px;
        cursor: pointer;
        user-select: none;
        transition: background 0.2s ease;
    }
    
    .entity-header:hover {
        background-color: #f1f3f5;
    }
    
    .entity.selected .entity-header {
        background-color: #d0ebff;
        font-weight: 600;
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
    
    .expand-toggle, .expand-spacer {
        width: 24px;
        text-align: center;
        margin-right: 8px;
        color: #6c757d;
    }
    
    .icon {
        margin-right: 12px;
        font-size: 1.1rem;
    }
    
    .name {
        flex-grow: 1;
        color: #212529;
    }
    
    .meta-info {
        display: flex;
        align-items: center;
        margin: 0 12px;
        font-size: 0.85em;
        opacity: 0.85;
    }
    
    .user-badge {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 24px;
        height: 24px;
        border-radius: 50%;
        background-color: var(--user-color);
        color: #fff;
        font-size: 0.8em;
        margin-right: 6px;
        box-shadow: 0 1px 2px rgba(0,0,0,0.1);
    }
    
    .timestamp {
        white-space: nowrap;
        color: #6c757d;
    }
    
    .drag-handle {
        cursor: grab;
        opacity: 0.6;
        padding: 0 6px;
        transition: opacity 0.2s ease;
    }
    
    .drag-handle:hover {
        opacity: 1;
    }
    
    .children {
        margin-left: 20px;
        border-left: 2px dashed #dee2e6;
    }
</style>