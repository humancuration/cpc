<script>
    import { onMount, onDestroy } from 'svelte';
    import * as THREE from 'three';
    import { Waveform } from '@svicons/ionicons-outline';

    export let asset;
    export let thumbnailUrl = null;
    let canvas;
    let renderer;
    let scene;
    let camera;
    let model = null;
    let thumbnailLoading = false;
    let thumbnailError = false;
    
    // Initialize Three.js for 3D models
    onMount(() => {
        if (asset.asset_type === 'model') {
            initThreeJS();
            loadModel();
        }
    });
    
    // Reset thumbnail state when thumbnailUrl changes
    $: if (thumbnailUrl) {
        thumbnailLoading = true;
        thumbnailError = false;
    }
    
    function initThreeJS() {
        // Setup scene
        scene = new THREE.Scene();
        scene.background = new THREE.Color(0xf0f0f0);
        
        // Setup camera
        camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
        camera.position.z = 5;
        
        // Setup renderer
        renderer = new THREE.WebGLRenderer({
            canvas,
            antialias: true,
            alpha: true
        });
        renderer.setSize(300, 300);
        
        // Add lights
        const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
        scene.add(ambientLight);
        
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
        directionalLight.position.set(1, 1, 1);
        scene.add(directionalLight);
        
        // Animation loop
        const animate = () => {
            requestAnimationFrame(animate);
            if (model) {
                model.rotation.y += 0.01;
            }
            renderer.render(scene, camera);
        };
        animate();
    }
    
    async function loadModel() {
        // In a real implementation, we would load the model from asset.path
        // For now we'll just create a placeholder cube
        const geometry = new THREE.BoxGeometry();
        const material = new THREE.MeshPhongMaterial({ color: 0x00ff00 });
        model = new THREE.Mesh(geometry, material);
        scene.add(model);
    }
    
    onDestroy(() => {
        if (renderer) {
            renderer.dispose();
        }
    });
</script>

<div class="asset-preview">
    {#if asset.asset_type === 'Texture'}
        {#if thumbnailUrl && !thumbnailError}
            <img
                src={thumbnailUrl}
                alt={asset.name}
                class="thumbnail-preview"
                on:load={() => thumbnailLoading = false}
                on:error={() => {
                    thumbnailLoading = false;
                    thumbnailError = true;
                }}
            />
        {:else}
            <div class="generic-preview">
                {#if thumbnailLoading}
                    <div class="loading-spinner">Loading...</div>
                {:else}
                    {asset.asset_type.toUpperCase()}
                {/if}
            </div>
        {/if}
    
    {:else if asset.asset_type === 'model'}
        <canvas bind:this={canvas} class="model-preview"></canvas>
    
    {:else if asset.asset_type === 'audio'}
        <div class="audio-preview">
            <Waveform class="waveform-icon" />
            <audio controls>
                <source src={asset.path} type="audio/mpeg">
                Your browser does not support the audio element.
            </audio>
        </div>
    
    {:else}
        <div class="generic-preview">
            {asset.asset_type.toUpperCase()}
        </div>
    {/if}
</div>

<style>
    .asset-preview {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    }
    
    .thumbnail-preview {
        max-width: 128px;
        max-height: 128px;
        object-fit: contain;
    }
    
    .model-preview {
        width: 300px;
        height: 300px;
    }
    
    .audio-preview {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
        width: 100%;
    }
    
    .waveform-icon {
        width: 100px;
        height: 100px;
        color: #666;
    }
    
    .generic-preview {
        font-size: 24px;
        font-weight: bold;
        color: #555;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
    }
    
    .loading-spinner {
        border: 4px solid rgba(0, 0, 0, 0.1);
        border-left-color: #007bff;
        border-radius: 50%;
        width: 40px;
        height: 40px;
        animation: spin 1s linear infinite;
    }
    
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
</style>