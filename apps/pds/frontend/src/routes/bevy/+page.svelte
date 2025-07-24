<script>
    import BevyView from '$lib/BevyView.svelte';
    import { onMount } from 'svelte';
    
    let showBevy = true;
    
    onMount(() => {
        // Set page title
        document.title = 'Bevy Experience - CPC';
    });
</script>

<style>
    .bevy-page {
        width: 100vw;
        height: 100vh;
        display: flex;
        flex-direction: column;
    }
    
    .bevy-header {
        background: #1a1a1a;
        color: white;
        padding: 1rem;
        border-bottom: 1px solid #333;
    }
    
    .bevy-content {
        flex: 1;
        position: relative;
    }
    
    .bevy-info {
        position: absolute;
        bottom: 20px;
        left: 20px;
        background: rgba(0, 0, 0, 0.7);
        color: white;
        padding: 10px;
        border-radius: 5px;
        font-family: monospace;
        font-size: 12px;
    }
</style>

<div class="bevy-page">
    <div class="bevy-header">
        <h1>Bevy 3D Experience</h1>
        <p>Interactive 3D scene powered by Bevy engine</p>
    </div>
    
    <div class="bevy-content">
        {#if showBevy}
            <BevyView width="100%" height="100%" show={true} />
        {/if}
        
        <div class="bevy-info">
            <strong>Controls:</strong><br>
            • Use mouse to look around<br>
            • WASD to move (when implemented)<br>
            • Space to pause/resume
        </div>
    </div>
</div>