<script>
    import { onMount, onDestroy } from 'svelte';
    import { createEventDispatcher } from 'svelte';
    import { graphqlClient } from '$lib/services/graphql-client.js';
    import { mediaService } from '$lib/services/desktop-media-service.js';
    import { gql } from '@apollo/client/core';
    
    export let postId = null;
    export let allowMultiple = true;
    export let maxFileSize = 50 * 1024 * 1024; // 50MB
    export let allowedTypes = ['image/*', 'video/*', 'audio/*'];
    
    const dispatch = createEventDispatcher();
    
    let files = [];
    let uploadProgress = {};
    let processingStatus = {};
    let subscriptions = [];
    let isUploading = false;
    let error = null;
    
    const MEDIA_STATUS_SUBSCRIPTION = gql`
        subscription MediaStatusUpdated($mediaId: ID!) {
            mediaStatusUpdated(mediaId: $mediaId) {
                mediaId
                status
                progress
                message
                processedUrl
                thumbnailUrl
                error
            }
        }
    `;
    
    const POST_MEDIA_STATUS_SUBSCRIPTION = gql`
        subscription PostMediaStatusUpdated($postId: ID!) {
            postMediaStatusUpdated(postId: $postId) {
                mediaId
                status
                progress
                message
                processedUrl
                thumbnailUrl
                error
            }
        }
    `;
    
    function handleFileSelect(event) {
        const selectedFiles = Array.from(event.target.files);
        
        if (!allowMultiple && selectedFiles.length > 1) {
            error = 'Only one file is allowed';
            return;
        }
        
        if (!allowMultiple) {
            files = [];
            uploadProgress = {};
            processingStatus = {};
        }
        
        const validFiles = selectedFiles.filter(file => {
            if (file.size > maxFileSize) {
                error = `File ${file.name} exceeds maximum size of ${maxFileSize / (1024 * 1024)}MB`;
                return false;
            }
            
            const isValidType = allowedTypes.some(type => {
                if (type.endsWith('/*')) {
                    return file.type.startsWith(type.slice(0, -2));
                }
                return file.type === type;
            });
            
            if (!isValidType) {
                error = `File type ${file.type} is not allowed`;
                return false;
            }
            
            return true;
        });
        
        files = [...files, ...validFiles];
        error = null;
        
        if (postId) {
            uploadFiles();
        }
    }
    
    async function uploadFiles() {
        if (files.length === 0) return;
        
        isUploading = true;
        error = null;
        
        try {
            const uploadPromises = files.map(async (file, index) => {
                const fileId = `${Date.now()}-${index}`;
                uploadProgress[fileId] = 0;
                
                try {
                    const result = await mediaService.uploadMedia(file, postId, (progress) => {
                        uploadProgress[fileId] = progress;
                        uploadProgress = { ...uploadProgress };
                    });
                    
                    subscribeToProcessing(result.mediaId);
                    return result;
                } catch (err) {
                    console.error('Upload failed:', err);
                    throw err;
                }
            });
            
            const results = await Promise.all(uploadPromises);
            dispatch('uploadComplete', { files: results });
            files = [];
            uploadProgress = {};
            
        } catch (err) {
            error = err.message || 'Upload failed';
            dispatch('uploadError', { error: err });
        } finally {
            isUploading = false;
        }
    }
    
    function subscribeToProcessing(mediaId) {
        const subscription = graphqlClient.subscribe({
            query: MEDIA_STATUS_SUBSCRIPTION,
            variables: { mediaId }
        }).subscribe({
            next: (result) => {
                if (result.data?.mediaStatusUpdated) {
                    const update = result.data.mediaStatusUpdated;
                    processingStatus[mediaId] = update;
                    processingStatus = { ...processingStatus };
                    
                    dispatch('processingUpdate', { update });
                    
                    if (update.status === 'completed' || update.status === 'failed') {
                        setTimeout(() => {
                            delete processingStatus[mediaId];
                            processingStatus = { ...processingStatus };
                        }, 3000);
                    }
                }
            },
            error: (err) => {
                console.error('Subscription error:', err);
                processingStatus[mediaId] = {
                    status: 'error',
                    error: 'Failed to receive processing updates'
                };
                processingStatus = { ...processingStatus };
            }
        });
        
        subscriptions.push(subscription);
    }
    
    function subscribeToPostProcessing() {
        if (!postId) return;
        
        const subscription = graphqlClient.subscribe({
            query: POST_MEDIA_STATUS_SUBSCRIPTION,
            variables: { postId }
        }).subscribe({
            next: (result) => {
                if (result.data?.postMediaStatusUpdated) {
                    const update = result.data.postMediaStatusUpdated;
                    processingStatus[update.mediaId] = update;
                    processingStatus = { ...processingStatus };
                    
                    dispatch('processingUpdate', { update });
                }
            },
            error: (err) => {
                console.error('Post subscription error:', err);
            }
        });
        
        subscriptions.push(subscription);
    }
    
    function removeFile(index) {
        files = files.filter((_, i) => i !== index);
        const fileId = `${Date.now()}-${index}`;
        delete uploadProgress[fileId];
        uploadProgress = { ...uploadProgress };
    }
    
    function clearError() {
        error = null;
    }
    
    onMount(() => {
        if (postId) {
            subscribeToPostProcessing();
        }
    });
    
    onDestroy(() => {
        subscriptions.forEach(sub => sub.unsubscribe());
    });
    
    function getStatusIcon(status) {
        switch (status) {
            case 'pending': return '⏳';
            case 'processing': return '🔄';
            case 'completed': return '✅';
            case 'failed': return '❌';
            case 'retrying': return '🔄';
            default: return '📁';
        }
    }
    
    function getProgressColor(status) {
        switch (status) {
            case 'completed': return 'bg-green-500';
            case 'failed': return 'bg-red-500';
            case 'processing': return 'bg-blue-500';
            case 'retrying': return 'bg-yellow-500';
            default: return 'bg-gray-500';
        }
    }
</script>

<div class="media-uploader">
    {#if error}
        <div class="error-message bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            <span class="block sm:inline">{error}</span>
            <button on:click={clearError} class="ml-2 text-red-500 hover:text-red-700">
                ✕
            </button>
        </div>
    {/if}
    
    <div class="upload-area border-2 border-dashed border-gray-300 rounded-lg p-6 text-center">
        <input
            type="file"
            multiple={allowMultiple}
            accept={allowedTypes.join(',')}
            on:change={handleFileSelect}
            class="hidden"
            id="file-input"
        />
        
        <label for="file-input" class="cursor-pointer">
            <svg class="mx-auto h-12 w-12 text-gray-400" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
            <p class="mt-2 text-sm text-gray-600">
                Click to upload or drag and drop
            </p>
            <p class="text-xs text-gray-500">
                {allowedTypes.join(', ')} up to {maxFileSize / (1024 * 1024)}MB
            </p>
        </label>
    </div>
    
    {#if files.length > 0}
        <div class="mt-4 space-y-2">
            {#each files as file, index}
                <div class="flex items-center justify-between p-3 bg-gray-50 rounded">
                    <div class="flex items-center space-x-3">
                        <span class="text-sm font-medium">{file.name}</span>
                        <span class="text-xs text-gray-500">
                            ({(file.size / 1024 / 1024).toFixed(2)}MB)
                        </span>
                    </div>
                    
                    {#if uploadProgress[`${Date.now()}-${index}`]}
                        <div class="flex items-center space-x-2">
                            <div class="w-24 bg-gray-200 rounded-full h-2">
                                <div 
                                    class="bg-blue-500 h-2 rounded-full transition-all duration-300"
                                    style="width: {uploadProgress[`${Date.now()}-${index}`]}%"
                                ></div>
                            </div>
                            <span class="text-xs text-gray-600">
                                {uploadProgress[`${Date.now()}-${index}`]}%
                            </span>
                        </div>
                    {/if}
                    
                    <button 
                        on:click={() => removeFile(index)}
                        class="text-red-500 hover:text-red-700"
                        disabled={isUploading}
                    >
                        ✕
                    </button>
                </div>
            {/each}
            
            {#if !postId}
                <button
                    on:click={uploadFiles}
                    disabled={isUploading || files.length === 0}
                    class="w-full bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 disabled:opacity-50"
                >
                    {isUploading ? 'Uploading...' : 'Upload Files'}
                </button>
            {/if}
        </div>
    {/if}
    
    {#if Object.keys(processingStatus).length > 0}
        <div class="mt-4 space-y-2">
            <h4 class="font-medium text-gray-700">Processing Status:</h4>
            {#each Object.entries(processingStatus) as [mediaId, status]}
                <div class="p-3 bg-gray-50 rounded">
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            <span>{getStatusIcon(status.status)}</span>
                            <span class="text-sm font-medium">{status.status}</span>
                        </div>
                        {#if status.progress !== null}
                            <span class="text-xs text-gray-500">{status.progress}%</span>
                        {/if}
                    </div>
                    
                    {#if status.message}
                        <p class="text-xs text-gray-600 mt-1">{status.message}</p>
                    {/if}
                    
                    {#if status.error}
                        <p class="text-xs text-red-600 mt-1">{status.error}</p>
                    {/if}
                    
                    {#if status.status === 'processing' && status.progress !== null}
                        <div class="mt-2">
                            <div class="w-full bg-gray-200 rounded-full h-1.5">
                                <div 
                                    class="{getProgressColor(status.status)} h-1.5 rounded-full transition-all duration-300"
                                    style="width: {status.progress}%"
                                ></div>
                            </div>
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .media-uploader {
        @apply w-full;
    }
    
    .upload-area:hover {
        @apply border-blue-400;
    }
    
    .error-message {
        animation: fadeIn 0.3s ease-in;
    }
    
    @keyframes fadeIn {
        from { opacity: 0; transform: translateY(-10px); }
        to { opacity: 1; transform: translateY(0); }
    }
</style>