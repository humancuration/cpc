<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
    import { listen } from '@tauri-apps/api/event';
    import type { UnlistenFn } from '@tauri-apps/api/event';

    interface ARMarker {
        id: string;
        content_hash: string;
        position: [number, number];
        size: [number, number];
        confidence: number;
    }

    interface AREvent {
        type: 'marker_detected' | 'marker_lost' | 'tracking_update';
        marker?: ARMarker;
        data?: any;
    }

    interface ARPreview {
        content_hash: string;
        thumbnail_url: string;
        width: number;
        height: number;
    }

    export let experienceId: string | null = null;
    export let enableSharing: boolean = true;
    export let debugMode: boolean = false;

    let canvas: HTMLCanvasElement;
    let video: HTMLVideoElement;
    let isTracking: boolean = false;
    let markers: ARMarker[] = [];
    let preview: ARPreview | null = null;
    let error: string | null = null;
    let unlisten: UnlistenFn | null = null;

    // AR tracking state
    let trackingState = {
        enabled: false,
        cameraActive: false,
        fps: 0,
        lastFrameTime: 0
    };

    onMount(async () => {
        try {
            // Initialize AR tracking
            await invoke('initialize_ar_tracking');
            
            // Listen for AR events
            unlisten = await listen<AREvent>('ar-event', (event) => {
                handleAREvent(event.payload);
            });

            // Start camera
            await startCamera();
            
            // Load preview if experience ID provided
            if (experienceId) {
                await loadPreview();
            }

            // Start AR tracking
            await startTracking();
            
        } catch (err) {
            error = `Failed to initialize AR: ${err}`;
            console.error('AR initialization error:', err);
        }
    });

    onDestroy(async () => {
        if (unlisten) {
            unlisten();
        }
        
        try {
            await stopTracking();
            await stopCamera();
        } catch (err) {
            console.error('AR cleanup error:', err);
        }
    });

    async function startCamera() {
        try {
            const stream = await navigator.mediaDevices.getUserMedia({
                video: {
                    facingMode: 'environment',
                    width: { ideal: 1920 },
                    height: { ideal: 1080 }
                }
            });
            
            if (video) {
                video.srcObject = stream;
                await video.play();
                trackingState.cameraActive = true;
            }
        } catch (err) {
            error = `Camera access denied: ${err}`;
            console.error('Camera error:', err);
        }
    }

    async function stopCamera() {
        if (video && video.srcObject) {
            const tracks = (video.srcObject as MediaStream).getTracks();
            tracks.forEach(track => track.stop());
            video.srcObject = null;
            trackingState.cameraActive = false;
        }
    }

    async function startTracking() {
        try {
            await invoke('start_ar_tracking');
            isTracking = true;
            trackingState.enabled = true;
        } catch (err) {
            error = `Failed to start AR tracking: ${err}`;
            console.error('Tracking error:', err);
        }
    }

    async function stopTracking() {
        try {
            await invoke('stop_ar_tracking');
            isTracking = false;
            trackingState.enabled = false;
        } catch (err) {
            console.error('Stop tracking error:', err);
        }
    }

    function handleAREvent(event: AREvent) {
        switch (event.type) {
            case 'marker_detected':
                if (event.marker) {
                    markers = [...markers, event.marker];
                }
                break;
            case 'marker_lost':
                if (event.marker) {
                    markers = markers.filter(m => m.id !== event.marker!.id);
                }
                break;
            case 'tracking_update':
                if (event.data) {
                    trackingState.fps = event.data.fps || 0;
                    trackingState.lastFrameTime = event.data.timestamp || 0;
                }
                break;
        }
    }

    async function loadPreview() {
        if (!experienceId) return;
        
        try {
            const previewData = await invoke('get_ar_preview', { experienceId });
            preview = previewData;
        } catch (err) {
            console.error('Failed to load preview:', err);
        }
    }

    async function shareExperience() {
        if (!experienceId) return;
        
        try {
            const result = await invoke('share_ar_experience', {
                experienceId,
                visibility: 'public'
            });
            
            // Emit share event
            dispatch('share', result);
        } catch (err) {
            error = `Failed to share experience: ${err}`;
            console.error('Share error:', err);
        }
    }

    async function generatePreview() {
        if (!experienceId) return;
        
        try {
            const result = await invoke('generate_ar_preview', { experienceId });
            preview = result;
        } catch (err) {
            error = `Failed to generate preview: ${err}`;
            console.error('Preview error:', err);
        }
    }

    function drawDebugInfo(ctx: CanvasRenderingContext2D) {
        if (!debugMode) return;
        
        ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
        ctx.fillRect(10, 10, 200, 100);
        
        ctx.fillStyle = 'white';
        ctx.font = '12px monospace';
        ctx.fillText(`Tracking: ${isTracking}`, 20, 30);
        ctx.fillText(`Markers: ${markers.length}`, 20, 45);
        ctx.fillText(`Camera: ${trackingState.cameraActive}`, 20, 60);
        ctx.fillText(`FPS: ${trackingState.fps}`, 20, 75);
        ctx.fillText(`Last Frame: ${trackingState.lastFrameTime}`, 20, 90);
    }

    function drawMarkers(ctx: CanvasRenderingContext2D) {
        markers.forEach(marker => {
            const [x, y] = marker.position;
            const [w, h] = marker.size;
            
            // Draw marker bounding box
            ctx.strokeStyle = `rgba(0, 255, 0, ${marker.confidence})`;
            ctx.lineWidth = 2;
            ctx.strokeRect(x * ctx.canvas.width, y * ctx.canvas.height, w * ctx.canvas.width, h * ctx.canvas.height);
            
            // Draw marker ID
            ctx.fillStyle = 'white';
            ctx.font = '12px sans-serif';
            ctx.fillText(marker.id, x * ctx.canvas.width, y * ctx.canvas.height - 5);
        });
    }

    function onCanvasReady(node: HTMLCanvasElement) {
        canvas = node;
        const ctx = canvas.getContext('2d');
        
        if (ctx) {
            function render() {
                if (video && canvas) {
                    // Draw video frame
                    ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
                    
                    // Draw AR overlays
                    drawMarkers(ctx);
                    drawDebugInfo(ctx);
                }
                
                if (isTracking) {
                    requestAnimationFrame(render);
                }
            }
            
            render();
        }
    }
</script>

<div class="ar-view-container">
    {#if error}
        <div class="error-message">
            <p>{error}</p>
            <button on:click={() => error = null}>Dismiss</button>
        </div>
    {/if}

    <div class="ar-canvas-container">
        <video bind:this={video} class="ar-video" style="display: none;" />
        <canvas 
            bind:this={canvas}
            use:onCanvasReady
            width="1920" 
            height="1080"
            class="ar-canvas"
        />
        
        {#if debugMode}
            <div class="debug-overlay">
                <p>AR Debug Mode</p>
                <p>Markers: {markers.length}</p>
                <p>Tracking: {isTracking ? 'Active' : 'Inactive'}</p>
            </div>
        {/if}
    </div>

    <div class="ar-controls">
        <button 
            on:click={startTracking} 
            disabled={isTracking}
            class="control-btn"
        >
            Start Tracking
        </button>
        
        <button 
            on:click={stopTracking} 
            disabled={!isTracking}
            class="control-btn"
        >
            Stop Tracking
        </button>
        
        {#if enableSharing}
            <button 
                on:click={shareExperience}
                disabled={!experienceId}
                class="control-btn"
            >
                Share Experience
            </button>
        {/if}
        
        <button 
            on:click={generatePreview}
            disabled={!experienceId}
            class="control-btn"
        >
            Generate Preview
        </button>
    </div>

    {#if preview}
        <div class="preview-container">
            <h3>Experience Preview</h3>
            <img 
                src={`data:image/webp;base64,${btoa(String.fromCharCode(...new Uint8Array(preview.thumbnail_data)))}`} 
                alt="AR Experience Preview"
                class="preview-image"
            />
            <p>Content Hash: {preview.content_hash}</p>
        </div>
    {/if}

    {#if markers.length > 0}
        <div class="markers-list">
            <h3>Detected Markers ({markers.length})</h3>
            {#each markers as marker}
                <div class="marker-item">
                    <span>{marker.id}</span>
                    <span>Confidence: {(marker.confidence * 100).toFixed(1)}%</span>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .ar-view-container {
        position: relative;
        width: 100%;
        height: 100vh;
        background: #000;
    }

    .ar-canvas-container {
        position: relative;
        width: 100%;
        height: 100%;
    }

    .ar-canvas {
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .ar-video {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        object-fit: cover;
    }

    .debug-overlay {
        position: absolute;
        top: 10px;
        left: 10px;
        background: rgba(0, 0, 0, 0.7);
        color: white;
        padding: 10px;
        border-radius: 5px;
        font-family: monospace;
        font-size: 12px;
    }

    .ar-controls {
        position: absolute;
        bottom: 20px;
        left: 50%;
        transform: translateX(-50%);
        display: flex;
        gap: 10px;
        z-index: 100;
    }

    .control-btn {
        padding: 10px 20px;
        background: rgba(255, 255, 255, 0.2);
        border: 1px solid rgba(255, 255, 255, 0.3);
        color: white;
        border-radius: 5px;
        cursor: pointer;
        transition: background 0.2s;
    }

    .control-btn:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.3);
    }

    .control-btn:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .error-message {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background: rgba(255, 0, 0, 0.8);
        color: white;
        padding: 20px;
        border-radius: 10px;
        z-index: 1000;
    }

    .error-message button {
        margin-top: 10px;
        padding: 5px 10px;
        background: white;
        color: black;
        border: none;
        border-radius: 3px;
        cursor: pointer;
    }

    .preview-container {
        position: absolute;
        top: 20px;
        right: 20px;
        background: rgba(0, 0, 0, 0.8);
        color: white;
        padding: 15px;
        border-radius: 10px;
        max-width: 300px;
    }

    .preview-image {
        width: 100%;
        height: auto;
        border-radius: 5px;
        margin-top: 10px;
    }

    .markers-list {
        position: absolute;
        top: 20px;
        left: 20px;
        background: rgba(0, 0, 0, 0.8);
        color: white;
        padding: 15px;
        border-radius: 10px;
        max-width: 200px;
    }

    .marker-item {
        display: flex;
        justify-content: space-between;
        padding: 5px 0;
        font-size: 12px;
    }
</style>