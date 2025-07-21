<script>
    import { onMount } from 'svelte';
    import { getContext } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    
    export let collaborator;
    
    const canvasContext = getContext('editorCanvas');
    let canvas;
    let ctx;
    
    onMount(() => {
        canvas = canvasContext.canvas;
        ctx = canvas.getContext('2d');
        drawCursor();
    });
    
    function drawCursor() {
        if (!ctx || !collaborator) return;
        
        const rect = canvas.getBoundingClientRect();
        const x = collaborator.cursor_position.x;
        const y = collaborator.cursor_position.y;
        
        ctx.save();
        ctx.beginPath();
        ctx.moveTo(x, y);
        ctx.lineTo(x + 10, y + 10);
        ctx.strokeStyle = `rgb(${collaborator.color.r * 255}, ${collaborator.color.g * 255}, ${collaborator.color.b * 255})`;
        ctx.lineWidth = 2;
        ctx.stroke();
        
        // Draw collaborator name
        ctx.fillStyle = 'white';
        ctx.font = '12px Arial';
        ctx.fillText(collaborator.peer_id, x + 12, y + 12);
        
        ctx.restore();
    }
    
    $: if (collaborator) {
        drawCursor();
    }
</script>

<div class="collaborator-cursor">
    {#if collaborator.has_conflict}
        <div class="conflict-indicator" on:click={() => invoke('resolve_conflict', { peerId: collaborator.peer_id })}>
            ⚠️
        </div>
    {/if}
</div>

<style>
    .collaborator-cursor {
        position: absolute;
        pointer-events: none;
        z-index: 100;
    }
    
    .conflict-indicator {
        position: absolute;
        background: rgba(200, 0, 0, 0.7);
        border-radius: 50%;
        width: 20px;
        height: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        pointer-events: auto;
    }
</style>