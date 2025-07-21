<script>
    import { selectedEntityData } from '../stores/sceneStore';
    import { dispatchCommand } from '../stores/commandDispatcher';
    import VectorInput from './VectorInput.svelte';
    import AssetInput from './AssetInput.svelte';
    import ColorPicker from './ColorPicker.svelte';
    import { toast } from '@zerodevx/svelte-toast';
    
    let editingComponent = null;
    let editMode = false;

    // Function to handle component updates
    async function updateComponent(entityId, componentName, newValue) {
        try {
            await dispatchCommand('UpdateComponent', {
                entityId,
                componentName,
                componentData: newValue
            });
        } catch (error) {
            toast.push(`Update failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }

    // Function to remove a component
    async function removeComponent(entityId, componentName) {
        try {
            await dispatchCommand('RemoveComponent', {
                entityId,
                componentName
            });
        } catch (error) {
            toast.push(`Remove failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }

    // Function to add a new component
    async function addComponent(entityId, componentName) {
        try {
            await dispatchCommand('AddComponent', {
                entityId,
                componentName
            });
        } catch (error) {
            toast.push(`Add failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }
    
    // Function to render component-specific editors
    function renderComponentEditor(componentName, componentData) {
        switch(componentName) {
            case 'Transform':
                return {
                    editor: VectorInput,
                    props: {
                        values: componentData.translation,
                        onChange: (newValues) => updateComponent($selectedEntityData.id, 'Transform', { ...componentData, translation: newValues })
                    }
                };
            case 'Mesh':
                return {
                    editor: AssetInput,
                    props: {
                        assetId: componentData.asset_id,
                        onSelect: (assetId) => updateComponent($selectedEntityData.id, 'Mesh', { ...componentData, asset_id: assetId })
                    }
                };
            case 'Material':
                return {
                    editor: ColorPicker,
                    props: {
                        color: componentData.base_color,
                        onChange: (newColor) => updateComponent($selectedEntityData.id, 'Material', { ...componentData, base_color: newColor })
                    }
                };
            default:
                return null;
        }
    }
</script>

<div class="inspector-panel">
    <h2>Inspector</h2>
    
    {#if !$selectedEntityData}
        <p>No entity selected</p>
    {:else}
        <div class="entity-info">
            <h3>{$selectedEntityData.components.Name?.value || `Entity ${$selectedEntityData.id.substring(0,4)}`}</h3>
            <div class="property">
                <label>ID:</label>
                <span>{$selectedEntityData.id}</span>
            </div>
            
            {#if $selectedEntityData.parent}
                <div class="property">
                    <label>Parent:</label>
                    <span>{$selectedEntityData.parent}</span>
                </div>
            {/if}
            
            {#if $selectedEntityData.children.length > 0}
                <div class="property">
                    <label>Children:</label>
                    <span>{$selectedEntityData.children.length}</span>
                </div>
            {/if}
        </div>
        
        <div class="components">
            <div class="component-header-row">
                <h4>Components</h4>
                <button on:click={() => editMode = !editMode}>
                    {editMode ? 'Done' : 'Edit'}
                </button>
            </div>
            
            {#each Object.entries($selectedEntityData.components) as [name, component]}
                <div class="component">
                    <div class="component-header">
                        {name}
                        {#if editMode}
                            <button class="remove-btn" on:click={() => removeComponent($selectedEntityData.id, name)}>Remove</button>
                        {/if}
                    </div>
                    
                    {#if renderComponentEditor(name, component)}
                        <svelte:component
                            this={renderComponentEditor(name, component).editor}
                            {...renderComponentEditor(name, component).props}
                        />
                    {:else}
                        <pre class="component-data">{JSON.stringify(component, null, 2)}</pre>
                    {/if}
                </div>
            {/each}
            
            {#if editMode}
                <div class="add-component">
                    <select bind:value={editingComponent}>
                        <option value="">Add Component</option>
                        <option value="Transform">Transform</option>
                        <option value="Mesh">Mesh</option>
                        <option value="Material">Material</option>
                        <option value="Light">Light</option>
                    </select>
                    <button on:click={() => addComponent($selectedEntityData.id, editingComponent)}
                            disabled={!editingComponent}>
                        Add
                    </button>
                </div>
            {/if}
        </div>
    {/if}
</div>

<style>
    .inspector-panel {
        padding: 15px;
        border: 1px solid #ccc;
        height: 100%;
        overflow-y: auto;
    }
    
    .entity-info {
        margin-bottom: 20px;
        padding-bottom: 10px;
        border-bottom: 1px solid #eee;
    }
    
    .property {
        display: flex;
        margin: 5px 0;
    }
    
    .property label {
        font-weight: bold;
        width: 100px;
    }
    
    .components {
        margin-top: 20px;
    }
    
    .component {
        margin-bottom: 15px;
        border: 1px solid #eee;
        border-radius: 4px;
        padding: 10px;
    }
    
    .component-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-weight: bold;
        margin-bottom: 5px;
    }
    
    .remove-btn {
        background: #ff6b6b;
        color: white;
        border: none;
        border-radius: 4px;
        padding: 2px 8px;
        cursor: pointer;
    }
    
    .component-data {
        background: #f8f8f8;
        padding: 8px;
        border-radius: 4px;
        max-height: 200px;
        overflow: auto;
        font-size: 0.9em;
    }
    
    .component-header-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 10px;
    }
    
    .add-component {
        display: flex;
        gap: 10px;
        margin-top: 15px;
    }
</style>