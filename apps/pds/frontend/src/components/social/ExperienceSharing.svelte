<script>
    import { onMount } from 'svelte';
    import { socialStore } from '../../stores/socialStore.js';
    import { fileStore } from '../../stores/storage.js';
    
    export let experienceId = null;
    export let onSuccess = () => {};
    export let onCancel = () => {};
    
    let title = '';
    let description = '';
    let visibility = 'FRIENDS';
    let isSharing = false;
    let error = null;
    let contentHash = '';
    let fileSize = 0;
    
    $: canShare = title.trim() && contentHash;
    
    onMount(async () => {
        if (experienceId) {
            // Load existing experience data
            const experience = await socialStore.getExperience(experienceId);
            if (experience) {
                title = experience.title;
                description = experience.description || '';
                visibility = experience.visibility;
            }
        }
        
        // Get content hash from file store if available
        const fileData = await fileStore.getCurrentFile();
        if (fileData) {
            contentHash = fileData.hash;
            fileSize = fileData.size;
        }
    });
    
    async function handleShare() {
        if (!canShare || isSharing) return;
        
        isSharing = true;
        error = null;
        
        try {
            const result = await socialStore.shareExperience({
                experienceId: experienceId || crypto.randomUUID(),
                title: title.trim(),
                description: description.trim() || null,
                visibility,
                contentHash,
                fileSize
            });
            
            if (result.success) {
                onSuccess(result);
            } else {
                error = result.error || 'Failed to share experience';
            }
        } catch (err) {
            error = err.message || 'An error occurred';
        } finally {
            isSharing = false;
        }
    }
    
    function handleVisibilityChange(event) {
        visibility = event.target.value;
    }
</script>

<div class="experience-sharing">
    <h3>{experienceId ? 'Update Experience' : 'Share New Experience'}</h3>
    
    {#if error}
        <div class="error-message">{error}</div>
    {/if}
    
    <form on:submit|preventDefault={handleShare}>
        <div class="form-group">
            <label for="title">Title *</label>
            <input
                id="title"
                type="text"
                bind:value={title}
                placeholder="Enter experience title"
                required
            />
        </div>
        
        <div class="form-group">
            <label for="description">Description</label>
            <textarea
                id="description"
                bind:value={description}
                placeholder="Describe your AR experience"
                rows="3"
            ></textarea>
        </div>
        
        <div class="form-group">
            <label for="visibility">Visibility</label>
            <select id="visibility" bind:value={visibility} on:change={handleVisibilityChange}>
                <option value="PUBLIC">Public - Anyone can view</option>
                <option value="FRIENDS">Friends Only</option>
                <option value="PRIVATE">Private - Only you</option>
            </select>
        </div>
        
        <div class="form-actions">
            <button type="button" class="secondary" on:click={onCancel}>
                Cancel
            </button>
            <button type="submit" disabled={!canShare || isSharing}>
                {isSharing ? 'Sharing...' : 'Share Experience'}
            </button>
        </div>
    </form>
</div>

<style>
    .experience-sharing {
        max-width: 500px;
        margin: 0 auto;
        padding: 1rem;
    }
    
    h3 {
        margin-bottom: 1rem;
        color: var(--text-primary);
    }
    
    .error-message {
        background: var(--error-bg);
        color: var(--error-color);
        padding: 0.5rem;
        border-radius: 4px;
        margin-bottom: 1rem;
    }
    
    .form-group {
        margin-bottom: 1rem;
    }
    
    label {
        display: block;
        margin-bottom: 0.25rem;
        font-weight: 500;
        color: var(--text-secondary);
    }
    
    input, textarea, select {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        background: var(--input-bg);
        color: var(--text-primary);
    }
    
    textarea {
        resize: vertical;
        min-height: 80px;
    }
    
    .form-actions {
        display: flex;
        gap: 0.5rem;
        justify-content: flex-end;
        margin-top: 1.5rem;
    }
    
    button {
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-weight: 500;
    }
    
    button[type="submit"] {
        background: var(--primary-color);
        color: white;
    }
    
    button[type="submit"]:disabled {
        background: var(--disabled-bg);
        cursor: not-allowed;
    }
    
    .secondary {
        background: var(--secondary-bg);
        color: var(--text-secondary);
    }
</style>
</script>