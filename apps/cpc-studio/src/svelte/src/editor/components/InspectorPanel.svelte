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
        padding: 20px;
        border-radius: 12px;
        height: 100%;
        overflow-y: auto;
        background: #ffffff;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
    }
    
    .entity-info {
        margin-bottom: 25px;
        padding-bottom: 15px;
        border-bottom: 1px solid #e9ecef;
    }
    
    .property {
        display: flex;
        margin: 8px 0;
        color: #495057;
    }
    
    .property label {
        font-weight: 600;
        width: 120px;
        color: #212529;
    }
    
    .components {
        margin-top: 25px;
    }
    
    .component {
        margin-bottom: 20px;
        border: 1px solid #dee2e6;
        border-radius: 8px;
        padding: 15px;
        background: #f8f9fa;
        transition: box-shadow 0.2s ease;
    }
    
    .component:hover {
        box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
    
    .component-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-weight: 600;
        margin-bottom: 10px;
        color: #343a40;
    }
    
    .remove-btn {
        background: #ff4757;
        color: white;
        border: none;
        border-radius: 6px;
        padding: 4px 12px;
        cursor: pointer;
        transition: background 0.2s ease;
    }
    
    .remove-btn:hover {
        background: #ff6b81;
    }
    
    .component-data {
        background: #e9ecef;
        padding: 12px;
        border-radius: 6px;
        max-height: 250px;
        overflow: auto;
        font-size: 0.95em;
        color: #212529;
    }
    
    .component-header-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 15px;
    }
    
    .add-component {
        display: flex;
        gap: 15px;
        margin-top: 20px;
    }
    
    .add-component select {
        flex: 1;
        padding: 8px;
        border-radius: 6px;
        border: 1px solid #ced4da;
    }
    
    .add-component button {
        background: #1e90ff;
        color: white;
        border: none;
        border-radius: 6px;
        padding: 8px 16px;
        cursor: pointer;
        transition: background 0.2s ease;
    }
    
    .add-component button:hover {
        background: #40a9ff;
    }
    
    .add-component button:disabled {
        background: #a5d1ff;
        cursor: not-allowed;
    }
</style>