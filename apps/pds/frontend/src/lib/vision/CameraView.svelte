<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  const dispatch = createEventDispatcher();
  
  let videoRef;
  let canvasRef;
  let stream = null;
  let isCapturing = false;
  let hasPermission = false;
  let error = '';
  
  onMount(async () => {
    await checkCameraPermission();
    return () => {
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
    };
  });
  
  async function checkCameraPermission() {
    try {
      if (!navigator.mediaDevices || !navigator.mediaDevices.getUserMedia) {
        error = 'Camera not supported on this device';
        return;
      }
      
      const devices = await navigator.mediaDevices.enumerateDevices();
      const hasCamera = devices.some(device => device.kind === 'videoinput');
      
      if (!hasCamera) {
        error = 'No camera found';
        return;
      }
      
      hasPermission = true;
    } catch (err) {
      error = `Camera permission denied: ${err.message}`;
    }
  }
  
  async function startCamera() {
    if (!hasPermission) {
      error = 'Camera permission required';
      return;
    }
    
    try {
      stream = await navigator.mediaDevices.getUserMedia({ 
        video: { 
          width: { ideal: 1280 },
          height: { ideal: 720 },
          facingMode: 'environment'
        } 
      });
      
      videoRef.srcObject = stream;
      isCapturing = true;
      error = '';
    } catch (err) {
      error = `Failed to start camera: ${err.message}`;
    }
  }
  
  async function stopCamera() {
    if (stream) {
      stream.getTracks().forEach(track => track.stop());
      stream = null;
    }
    isCapturing = false;
  }
  
  async function captureImage() {
    if (!videoRef || !stream) {
      error = 'Camera not ready';
      return;
    }
    
    try {
      const canvas = canvasRef;
      const video = videoRef;
      
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      
      const ctx = canvas.getContext('2d');
      ctx.drawImage(video, 0, 0);
      
      // Convert to blob
      const blob = await new Promise(resolve => 
        canvas.toBlob(resolve, 'image/jpeg', 0.9)
      );
      
      // Convert to array buffer
      const buffer = await blob.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      
      // Process image
      const result = await invoke('process_camera_image', { 
        imageData: Array.from(bytes) 
      });
      
      dispatch('recognition', { result });
      return result;
      
    } catch (err) {
      error = `Capture failed: ${err}`;
    }
  }
  
  async function uploadImage(event) {
    const file = event.target.files[0];
    if (!file) return;
    
    try {
      const buffer = await file.arrayBuffer();
      const bytes = new Uint8Array(buffer);
      
      const result = await invoke('process_camera_image', { 
        imageData: Array.from(bytes) 
      });
      
      dispatch('recognition', { result });
      
    } catch (err) {
      error = `Upload failed: ${err}`;
    }
  }
</script>

<div class="camera-container">
  {#if error}
    <div class="error-message">
      <p>{error}</p>
      <button on:click={checkCameraPermission}>Retry</button>
    </div>
  {:else if !hasPermission}
    <div class="permission-request">
      <p>Camera access is required for image recognition.</p>
      <button on:click={checkCameraPermission}>Grant Permission</button>
    </div>
  {:else}
    <div class="camera-view">
      <video 
        bind:this={videoRef}
        autoplay 
        muted 
        playsinline
        class="camera-feed"
      />
      
      <canvas 
        bind:this={canvasRef}
        style="display: none;"
      />
      
      <div class="camera-controls">
        {#if !isCapturing}
          <button on:click={startCamera} class="btn primary">
            Start Camera
          </button>
        {:else}
          <button on:click={captureImage} class="btn capture">
            Capture
          </button>
          <button on:click={stopCamera} class="btn secondary">
            Stop
          </button>
        {/if}
        
        <label class="btn upload">
          Upload Image
          <input 
            type="file" 
            accept="image/*"
            on:change={uploadImage}
            style="display: none;"
          />
        </label>
      </div>
    </div>
  {/if}
</div>

<style>
  .camera-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
  }
  
  .camera-view {
    position: relative;
    width: 100%;
    max-width: 640px;
  }
  
  .camera-feed {
    width: 100%;
    height: auto;
    border-radius: 8px;
    background: #000;
  }
  
  .camera-controls {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
    margin-top: 1rem;
  }
  
  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .btn.primary {
    background: #007bff;
    color: white;
  }
  
  .btn.primary:hover {
    background: #0056b3;
  }
  
  .btn.secondary {
    background: #6c757d;
    color: white;
  }
  
  .btn.secondary:hover {
    background: #545b62;
  }
  
  .btn.capture {
    background: #28a745;
    color: white;
    font-weight: bold;
  }
  
  .btn.capture:hover {
    background: #1e7e34;
  }
  
  .btn.upload {
    background: #17a2b8;
    color: white;
  }
  
  .btn.upload:hover {
    background: #138496;
  }
  
  .error-message, .permission-request {
    text-align: center;
    padding: 2rem;
    background: #f8f9fa;
    border-radius: 8px;
  }
  
  .error-message p, .permission-request p {
    margin-bottom: 1rem;
    color: #495057;
  }
</style>