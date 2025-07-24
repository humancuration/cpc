<script>
    import BevyView from '$lib/BevyView.svelte';
    import { onMount } from 'svelte';
    
    let showBevy = true;
    
    onMount(() => {
        document.title = 'Bevy Experience - CPC';
    });
</script>

<style>
    .bevy-container {
        width: 100%;
        height: calc(100vh - 100px);
        position: relative;
        background: #000;
    }
    
    .bevy-header {
        padding: 1rem;
        background: #1a1a1a;
        color: white;
        border-bottom: 1px solid #333;
    }
    
    .bevy-content {
        width: 100%;
        height: 100%;
    }
</style>

<div class="bevy-container">
    <div class="bevy-header">
        <h2>Bevy 3D Experience</h2>
        <p>Interactive 3D scene powered by Bevy engine</p>
    </div>
    <div class="bevy-content">
        <BevyView width="100%" height="100%" bind:show={showBevy} />
    </div>
</div>