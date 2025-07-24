<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    
    export let width = 1920;
    export let height = 1080;
    export let facingMode: 'user' | 'environment' = 'environment';
    
    const dispatch = createEventDispatcher();
    
    let video: HTMLVideoElement;
    let canvas: HTMLCanvasElement;
    let stream: MediaStream | null = null;
    let isCapturing = false;
    let error: string | null = null;
    
    // WebRTC constraints
    const constraints: MediaStreamConstraints = {
        video: {
            width: { ideal: width },
            height: { ideal: height },
            facingMode: { ideal: facingMode }
        },
        audio: false
    };
    
    onMount(async () => {
        try {
            stream = await navigator.mediaDevices.getUserMedia(constraints);
            if (video) {
                video.srcObject = stream;
            }
        } catch (err) {
            error = `Failed to access camera: ${err.message}`;
            console.error('Camera access error:', err);
        }
    });
    
    onDestroy(() => {
        if (stream) {
            stream.getTracks().forEach(track => track.stop());
        }
    });
    
    async function captureImage() {
        if (!stream || !video || !canvas) {
            error = 'Camera not ready';
            return;
        }
        
        isCapturing = true;
        error = null;
        
        try {
            // Set canvas dimensions to match video
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
            
            const ctx = canvas.getContext('2d');
            if (!ctx) {
                throw new Error('Failed to get canvas context');
            }
            
            // Draw video frame to canvas
            ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
            
            // Convert to WebP blob
            const blob = await new Promise<Blob>((resolve, reject) => {
                canvas.toBlob(
                    (blob) => {
                        if (blob) resolve(blob);
                        else reject(new Error('Failed to create blob'));
                    },
                    'image/webp',
                    0.85
                );
            });
            
            // Convert to ArrayBuffer
            const buffer = await blob.arrayBuffer();
            const bytes = new Uint8Array(buffer);
            
            // Dispatch capture event
            dispatch('capture', {
                data: bytes,
                width: canvas.width,
                height: canvas.height,
                format: 'webp'
            });
            
        } catch (err) {
            error = `Capture failed: ${err.message}`;
            console.error('Capture error:', err);
        } finally {
            isCapturing = false;
        }
    }
    
    function switchCamera() {
        facingMode = facingMode === 'user' ? 'environment' : 'user';
        restartStream();
    }
    
    async function restartStream() {
        if (stream) {
            stream.getTracks().forEach(track => track.stop());
        }
        
        try {
            const newConstraints = {
                ...constraints,
                video: {
                    ...constraints.video,
                    facingMode: { ideal: facingMode }
                }
            };
            
            stream = await navigator.mediaDevices.getUserMedia(newConstraints);
            if (video) {
                video.srcObject = stream;
            }
            error = null;
        } catch (err) {
            error = `Failed to switch camera: ${err.message}`;
        }
    }
</script>

<div class="image-capture">
    {#if error}
        <div class="error-message">
            {error}
        </div>
    {/if}
    
    <div class="video-container">
        <video
            bind:this={video}
            autoplay
            playsinline
            muted
            class="video-feed"
        />
        
        <canvas
            bind:this={canvas}
            style="display: none;"
        />
    </div>
    
    <div class="controls">
        <button
            class="capture-button"
            on:click={captureImage}
            disabled={isCapturing || !stream}
            class:capturing={isCapturing}
        >
            {isCapturing ? 'Processing...' : 'Capture'}
        </button>
        
        <button
            class="switch-button"
            on:click={switchCamera}
            disabled={isCapturing || !stream}
        >
            Switch Camera
        </button>
    </div>
</div>

<style>
    .image-capture {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        max-width: 600px;
        margin: 0 auto;
    }
    
    .error-message {
        background: #fee;
        color: #c53030;
        padding: 1rem;
        border-radius: 0.5rem;
        border: 1px solid #fc8181;
    }
    
    .video-container {
        position: relative;
        width: 100%;
        background: #000;
        border-radius: 0.5rem;
        overflow: hidden;
    }
    
    .video-feed {
        width: 100%;
        height: auto;
        display: block;
    }
    
    .controls {
        display: flex;
        gap: 1rem;
        justify-content: center;
    }
    
    .capture-button,
    .switch-button {
        padding: 0.75rem 1.5rem;
        border: none;
        border-radius: 0.5rem;
        font-weight: 600;
        cursor: pointer;
        transition: all 0.2s;
    }
    
    .capture-button {
        background: #4299e1;
        color: white;
    }
    
    .capture-button:hover:not(:disabled) {
        background: #3182ce;
    }
    
    .capture-button:disabled {
        background: #a0aec0;
        cursor: not-allowed;
    }
    
    .capture-button.capturing {
        background: #ed8936;
    }
    
    .switch-button {
        background: #e2e8f0;
        color: #2d3748;
    }
    
    .switch-button:hover:not(:disabled) {
        background: #cbd5e0;
    }
    
    .switch-button:disabled {
        background: #a0aec0;
        cursor: not-allowed;
    }
</style>
</script>