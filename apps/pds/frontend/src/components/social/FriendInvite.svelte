<script>
    import { socialStore } from '../../stores/socialStore.js';
    
    export let onSuccess = () => {};
    export let onCancel = () => {};
    
    let userId = '';
    let isInviting = false;
    let error = null;
    let success = false;
    let invitationCode = '';
    
    $: canInvite = userId.trim();
    
    async function handleInvite() {
        if (!canInvite || isInviting) return;
        
        isInviting = true;
        error = null;
        
        try {
            const result = await socialStore.inviteFriend(userId.trim());
            
            if (result.success) {
                success = true;
                invitationCode = result.invitationCode;
                setTimeout(() => {
                    onSuccess(result);
                }, 2000);
            } else {
                error = result.error || 'Failed to send invitation';
            }
        } catch (err) {
            error = err.message || 'An error occurred';
        } finally {
            isInviting = false;
        }
    }
    
    function handleInputChange(event) {
        userId = event.target.value;
        error = null;
    }
</script>

<div class="friend-invite">
    <h3>Invite a Friend</h3>
    
    {#if success}
        <div class="success-message">
            <p>Invitation sent successfully!</p>
            <p>Invitation code: <code>{invitationCode}</code></p>
        </div>
    {:else}
        <form on:submit|preventDefault={handleInvite}>
            {#if error}
                <div class="error-message">{error}</div>
            {/if}
            
            <div class="form-group">
                <label for="userId">User ID</label>
                <input
                    id="userId"
                    type="text"
                    bind:value={userId}
                    on:input={handleInputChange}
                    placeholder="Enter friend's user ID"
                    required
                />
            </div>
            
            <div class="form-actions">
                <button type="button" class="secondary" on:click={onCancel}>
                    Cancel
                </button>
                <button type="submit" disabled={!canInvite || isInviting}>
                    {isInviting ? 'Sending...' : 'Send Invitation'}
                </button>
            </div>
        </form>
    {/if}
</div>

<style>
    .friend-invite {
        max-width: 400px;
        margin: 0 auto;
        padding: 1rem;
    }
    
    h3 {
        margin-bottom: 1rem;
        color: var(--text-primary);
    }
    
    .success-message {
        background: var(--success-bg);
        color: var(--success-color);
        padding: 1rem;
        border-radius: 4px;
        text-align: center;
    }
    
    .success-message code {
        background: rgba(0, 0, 0, 0.1);
        padding: 0.2rem 0.4rem;
        border-radius: 2px;
        font-family: monospace;
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
    
    input {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        background: var(--input-bg);
        color: var(--text-primary);
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