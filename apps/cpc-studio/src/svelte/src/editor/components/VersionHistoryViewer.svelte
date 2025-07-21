<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { toast } from '@zerodevx/svelte-toast';

    export let asset;

    let versions = [];
    let selectedVersion = null;
    let diffResult = null;
    let versionA = null;
    let versionB = null;

    // Load version history when asset changes
    $: if (asset) {
        loadVersionHistory();
    }

    async function loadVersionHistory() {
        try {
            versions = await invoke('get_asset_version_history', { assetId: asset.asset_id });
        } catch (error) {
            toast.push(`Failed to load version history: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }

    async function restoreVersion() {
        if (!selectedVersion) return;
        try {
            await invoke('restore_asset_version', {
                assetId: asset.asset_id,
                version: selectedVersion.version
            });
            toast.push(`Restored version ${selectedVersion.version}`, { theme: { '--toastColor': 'green' } });
        } catch (error) {
            toast.push(`Restore failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }

    async function previewVersion() {
        if (!selectedVersion) return;
        try {
            // This would open a preview window in a real implementation
            toast.push(`Previewing version ${selectedVersion.version}`, { theme: { '--toastColor': 'blue' } });
        } catch (error) {
            toast.push(`Preview failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }

    async function showDiff() {
        if (!versionA || !versionB) return;
        try {
            diffResult = await invoke('get_asset_version_diff', {
                assetId: asset.asset_id,
                versionA: versionA.version,
                versionB: versionB.version
            });
        } catch (error) {
            toast.push(`Diff failed: ${error.message}`, { theme: { '--toastColor': 'red' } });
        }
    }
</script>

<div class="version-history">
    <h3>Version History</h3>
    
    <div class="version-list">
        {#each versions as version (version.version)}
            <div 
                class="version-item {selectedVersion?.version === version.version ? 'selected' : ''}" 
                on:click={() => selectedVersion = version}
            >
                <div class="version-number">v{version.version}</div>
                <div class="version-date">{new Date(version.timestamp * 1000).toLocaleString()}</div>
                {#if version.author}
                    <div class="version-author">By {version.author}</div>
                {/if}
            </div>
        {/each}
    </div>
    
    <div class="actions">
        <button on:click={previewVersion} disabled={!selectedVersion}>Preview</button>
        <button on:click={restoreVersion} disabled={!selectedVersion}>Restore</button>
    </div>
    
    <div class="diff-section">
        <h4>Compare Versions</h4>
        <div class="diff-selectors">
            <select bind:value={versionA}>
                <option value={null} disabled>Select version A</option>
                {#each versions as version (version.version)}
                    <option value={version}>v{version.version}</option>
                {/each}
            </select>
            <select bind:value={versionB}>
                <option value={null} disabled>Select version B</option>
                {#each versions as version (version.version)}
                    <option value={version}>v{version.version}</option>
                {/each}
            </select>
            <button on:click={showDiff} disabled={!versionA || !versionB}>Show Diff</button>
        </div>
        
        {#if diffResult}
            <div class="diff-result">
                {#if diffResult.type === 'Text'}
                    <pre>{diffResult.content}</pre>
                {:else if diffResult.type === 'Image'}
                    <img src={`data:image/png;base64,${diffResult.content}`} alt="Visual diff" />
                {:else}
                    <p>Unsupported diff type</p>
                {/if}
            </div>
        {/if}
    </div>
</div>

<style>
    .version-history {
        padding: 10px;
    }
    
    .version-list {
        max-height: 300px;
        overflow-y: auto;
        margin-bottom: 10px;
    }
    
    .version-item {
        padding: 8px;
        border-bottom: 1px solid #eee;
        cursor: pointer;
    }
    
    .version-item:hover {
        background-color: #f0f0f0;
    }
    
    .version-item.selected {
        background-color: #e0f0ff;
    }
    
    .version-number {
        font-weight: bold;
    }
    
    .version-date {
        font-size: 0.8em;
        color: #666;
    }
    
    .version-author {
        font-size: 0.8em;
        color: #444;
    }
    
    .actions {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    }
    
    .diff-selectors {
        display: flex;
        gap: 10px;
        margin-bottom: 10px;
    }
    
    .diff-result {
        border: 1px solid #ddd;
        padding: 10px;
        max-height: 300px;
        overflow: auto;
    }
    
    pre {
        white-space: pre-wrap;
    }
</style>