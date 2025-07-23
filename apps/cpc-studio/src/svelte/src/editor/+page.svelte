<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event';
    import CollaboratorCursor from './components/CollaboratorCursor.svelte';
    import { currentPath, assets, selectedAsset } from './stores/assetStore';
    import VersionHistoryViewer from './components/VersionHistoryViewer.svelte';
    import AssetBrowser from './components/AssetBrowser.svelte';
    import EntityInspector from './components/EntityInspector.svelte';
    import SceneHierarchyPanel from './components/SceneHierarchyPanel.svelte';
    import { selectedEntityData } from './stores/sceneStore';
    
    let editorState = {
        activeScene: '',
        selectedEntities: [],
        resources: [],
        collaborators: [] // Added collaborators array
    };
    
    let canvasCtx;
    
    function textureUpdateHandler(event) {
        const base64 = event.detail;
        const img = new Image();
        img.onload = () => {
            canvasCtx.drawImage(img, 0, 0);
        };
        img.src = `data:image/png;base64,${base64}`;
    }
    
    onMount(async () => {
        const canvas = document.getElementById('editor-canvas');
        canvasCtx = canvas.getContext('2d');
        
        // Setup event listeners
        window.addEventListener('texture-update', textureUpdateHandler);
        
        // Initialize editor state
        editorState = await invoke('get_editor_state');
        
        // Set up collaborator listeners
        invoke('subscribe_to_collaborator_updates');
        
        // Listen for command-executed events
        const unlisten = await listen('command-executed', (event) => {
            const payload = event.payload;
            switch(payload.command_type) {
                case 'CreateEntity':
                    sceneStore.addEntity(payload.entity_id, payload.parent_id);
                    break;
                case 'DeleteEntity':
                    sceneStore.removeEntity(payload.entity_id);
                    break;
                case 'ReparentEntity':
                    sceneStore.reparentEntity(payload.entity_id, payload.parent_id);
                    break;
                // Add cases for other command types
            }
        });
        
        return () => {
            window.removeEventListener('texture-update', textureUpdateHandler);
            unlisten();
        };
    });
    // Function to resolve conflicts
    async function resolveConflict(peerId) {
        await invoke('resolve_conflict', { peerId });
    }
    
    // Handle asset selection from AssetBrowser
    function handleAssetSelect(asset) {
        selectedAsset.set(asset);
    }
</script>

<main>
    <h1>CPC Studio Editor</h1>
    
    <div class="editor-layout">
        <div class="left-panel">
            <div class="panel-section">
                <h3>Scene Hierarchy</h3>
                <SceneHierarchyPanel />
            </div>
            <div class="panel-section">
                <h3>Assets</h3>
                <AssetBrowser on:assetSelect={handleAssetSelect} />
            </div>
        </div>
        
        <div class="viewport">
            <h2>Viewport</h2>
            <!-- Bevy rendering will be displayed here -->
            <canvas id="editor-canvas"></canvas>
            
            <!-- Render collaborator cursors -->
            {#each editorState.collaborators as collaborator (collaborator.peer_id)}
                <CollaboratorCursor {collaborator} />
            {/each}
        </div>
        <div class="inspector-panel">
            {#if $selectedEntityData}
                <EntityInspector />
            {:else if selectedAsset}
                <InspectorPanel {selectedAsset} />
                <VersionHistoryViewer asset={selectedAsset} />
            {:else}
                <div class="empty-inspector">
                    <p>Select an entity or asset to inspect</p>
                </div>
            {/if}
    </div>
</div>
</main>

<style>
    .editor-layout {
        display: grid;
        grid-template-columns: 300px 1fr 300px;
        gap: 20px;
        height: 100vh;
        background: linear-gradient(to bottom, #f8f9fa, #e9ecef);
        padding: 20px;
    }
    
    .left-panel {
        display: flex;
        flex-direction: column;
        gap: 20px;
    }
    
    .panel-section {
        border: none;
        border-radius: 12px;
        padding: 15px;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        flex: 1;
        background: white;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        transition: box-shadow 0.3s ease;
    }
    
    .panel-section:hover {
        box-shadow: 0 6px 12px rgba(0,0,0,0.15);
    }
    
    .panel-section h3 {
        margin-top: 0;
        margin-bottom: 15px;
        padding-bottom: 10px;
        border-bottom: 1px solid #e9ecef;
        color: #343a40;
        font-size: 1.1rem;
    }
    
    .viewport {
        position: relative;
        display: flex;
        flex-direction: column;
        border-radius: 12px;
        overflow: hidden;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        background: white;
    }
    
    .inspector-panel {
        border: none;
        border-radius: 12px;
        padding: 15px;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        background: white;
        box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        transition: box-shadow 0.3s ease;
    }
    
    .inspector-panel:hover {
        box-shadow: 0 6px 12px rgba(0,0,0,0.15);
    }
    
    #editor-canvas {
        width: 100%;
        flex: 1;
        background: #2c3e50;
        border-radius: 0 0 12px 12px;
    }
</style>