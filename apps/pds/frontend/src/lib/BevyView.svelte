<script>
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event';

    export let width = '100%';
    export let height = '100%';
    export let show = true;

    let canvas;
    let container;
    let isInitialized = false;
    let isRunning = false;
    let error = null;

    // Bevy event listeners
    let unsubscribeResize;
    let unsubscribeFocus;

    // Initialize Bevy
    async function initializeBevy() {
        try {
            error = null;
            await invoke('initialize_bevy');
            isInitialized = true;
            isRunning = true;
            console.log('Bevy initialized successfully');
        } catch (e) {
            error = `Failed to initialize Bevy: ${e}`;
            console.error('Bevy initialization error:', e);
        }
    }

    // Control Bevy lifecycle
    async function controlBevy(action) {
        if (!isInitialized) return;
        
        try {
            await invoke('control_bevy', { action });
            
            if (action === 'pause') {
                isRunning = false;
            } else if (action === 'resume') {
                isRunning = true;
            } else if (action === 'stop') {
                isInitialized = false;
                isRunning = false;
            }
        } catch (e) {
            console.error('Bevy control error:', e);
        }
    }

    // Send message to Bevy
    async function sendToBevy(message) {
        if (!isInitialized) return;
        
        try {
            await invoke('send_to_bevy', { message });
        } catch (e) {
            console.error('Bevy message error:', e);
        }
    }

    // Handle window resize
    async function handleResize() {
        if (!isInitialized || !container) return;
        
        const rect = container.getBoundingClientRect();
        await sendToBevy(`resize:${rect.width}:${rect.height}`);
    }

    // Handle window focus
    async function handleFocus(focused) {
        if (!isInitialized) return;
        
        if (focused) {
            await controlBevy('resume');
        } else {
            await controlBevy('pause');
        }
    }

    // Lifecycle management
    onMount(async () => {
        if (show && !isInitialized) {
            await initializeBevy();
        }

        // Listen for window events
        unsubscribeResize = listen('tauri://resize', handleResize);
        unsubscribeFocus = listen('tauri://focus', (event) => {
            handleFocus(event.payload);
        });

        // Initial resize
        setTimeout(handleResize, 100);
    });

    onDestroy(async () => {
        if (unsubscribeResize) unsubscribeResize();
        if (unsubscribeFocus) unsubscribeFocus();
        
        if (isInitialized) {
            await controlBevy('stop');
        }
    });

    // React to show prop changes
    $: if (show && !isInitialized && container) {
        initializeBevy();
    } else if (!show && isInitialized) {
        controlBevy('pause');
    }

    // Expose methods for parent components
    export const bevyAPI = {
        initialize: initializeBevy,
        pause: () => controlBevy('pause'),
        resume: () => controlBevy('resume'),
        stop: () => controlBevy('stop'),
        sendMessage: sendToBevy,
        isRunning: () => isRunning,
        isInitialized: () => isInitialized
    };
</script>

<style>
    .bevy-container {
        position: relative;
        width: 100%;
        height: 100%;
        background: #000;
        overflow: hidden;
    }

    .bevy-canvas {
        width: 100%;
        height: 100%;
        display: block;
    }

    .bevy-overlay {
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(0, 0, 0, 0.8);
        color: white;
        font-family: monospace;
    }

    .bevy-controls {
        position: absolute;
        top: 10px;
        right: 10px;
        display: flex;
        gap: 10px;
        z-index: 100;
    }

    .bevy-button {
        padding: 5px 10px;
        background: rgba(255, 255, 255, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.3);
        color: white;
        cursor: pointer;
        border-radius: 3px;
        font-size: 12px;
    }

    .bevy-button:hover {
        background: rgba(255, 255, 255, 0.3);
    }

    .bevy-button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .status-indicator {
        position: absolute;
        top: 10px;
        left: 10px;
        padding: 5px 10px;
        border-radius: 3px;
        font-size: 12px;
        font-family: monospace;
    }

    .status-running {
        background: rgba(0, 255, 0, 0.2);
        border: 1px solid rgba(0, 255, 0, 0.3);
        color: #0f0;
    }

    .status-paused {
        background: rgba(255, 255, 0, 0.2);
        border: 1px solid rgba(255, 255, 0, 0.3);
        color: #ff0;
    }

    .status-stopped {
        background: rgba(255, 0, 0, 0.2);
        border: 1px solid rgba(255, 0, 0, 0.3);
        color: #f00;
    }
</style>

<div class="bevy-container" bind:this={container}>
    <canvas 
        class="bevy-canvas" 
        bind:this={canvas}
        width={width}
        height={height}
    />
    
    {#if error}
        <div class="bevy-overlay">
            <div>{error}</div>
        </div>
    {/if}
    
    <div class="bevy-controls">
        <button 
            class="bevy-button" 
            on:click={() => controlBevy(isRunning ? 'pause' : 'resume')}
            disabled={!isInitialized}
        >
            {isRunning ? 'Pause' : 'Resume'}
        </button>
        <button 
            class="bevy-button" 
            on:click={() => controlBevy('stop')}
            disabled={!isInitialized}
        >
            Stop
        </button>
    </div>
    
    <div class="status-indicator {isRunning ? 'status-running' : isInitialized ? 'status-paused' : 'status-stopped'}">
        {isRunning ? 'Running' : isInitialized ? 'Paused' : 'Stopped'}
    </div>
</div>