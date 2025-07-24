import { graphqlClient } from './graphql-client.js';
import { gql } from '@apollo/client/core';

// GraphQL mutations and subscriptions for media processing
const UPLOAD_MEDIA_MUTATION = gql`
  mutation UploadMedia($file: Upload!, $postId: ID, $metadata: MediaMetadataInput) {
    uploadMedia(file: $file, postId: $postId, metadata: $metadata) {
      id
      filename
      originalFilename
      mimeType
      size
      status
      uploadProgress
      processingStatus
      createdAt
      updatedAt
      postId
      processedUrl
      thumbnailUrl
      error
    }
  }
`;

const GET_MEDIA_STATUS_QUERY = gql`
  query GetMediaStatus($mediaId: ID!) {
    mediaStatus(mediaId: $mediaId) {
      id
      status
      processingStatus
      uploadProgress
      processedUrl
      thumbnailUrl
      error
      createdAt
      updatedAt
    }
  }
`;

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

class DesktopMediaService {
  constructor() {
    this.activeSubscriptions = new Map();
  }

  /**
   * Upload media file with progress tracking
   * @param {File} file - The file to upload
   * @param {string} postId - Optional post ID to associate with the media
   * @param {Function} onProgress - Progress callback
   * @returns {Promise} Upload result
   */
  async uploadMedia(file, postId = null, onProgress = null) {
    try {
      const result = await graphqlClient.mutate({
        mutation: UPLOAD_MEDIA_MUTATION,
        variables: {
          file,
          postId,
          metadata: {
            filename: file.name,
            mimeType: file.type,
            size: file.size
          }
        },
        context: {
          fetchOptions: {
            onProgress: (progressEvent) => {
              if (onProgress && progressEvent.lengthComputable) {
                const progress = Math.round((progressEvent.loaded * 100) / progressEvent.total);
                onProgress(progress);
              }
            }
          }
        }
      });

      return result.data.uploadMedia;
    } catch (error) {
      console.error('Upload failed:', error);
      throw new Error(error.message || 'Failed to upload media');
    }
  }

  /**
   * Get current media processing status
   * @param {string} mediaId - The media ID
   * @returns {Promise} Media status
   */
  async getMediaStatus(mediaId) {
    try {
      const result = await graphqlClient.query({
        query: GET_MEDIA_STATUS_QUERY,
        variables: { mediaId },
        fetchPolicy: 'network-only'
      });

      return result.data.mediaStatus;
    } catch (error) {
      console.error('Failed to get media status:', error);
      throw new Error(error.message || 'Failed to get media status');
    }
  }

  /**
   * Subscribe to media processing updates
   * @param {string} mediaId - The media ID to subscribe to
   * @param {Function} onUpdate - Callback for status updates
   * @returns {Function} Unsubscribe function
   */
  subscribeToMediaStatus(mediaId, onUpdate) {
    if (this.activeSubscriptions.has(mediaId)) {
      this.activeSubscriptions.get(mediaId).unsubscribe();
    }

    const subscription = graphqlClient.subscribe({
      query: MEDIA_STATUS_SUBSCRIPTION,
      variables: { mediaId }
    }).subscribe({
      next: (result) => {
        if (result.data?.mediaStatusUpdated && onUpdate) {
          onUpdate(result.data.mediaStatusUpdated);
        }
      },
      error: (error) => {
        console.error('Media subscription error:', error);
        onUpdate?.({ error: error.message });
      }
    });

    this.activeSubscriptions.set(mediaId, subscription);
    
    return () => {
      if (this.activeSubscriptions.has(mediaId)) {
        this.activeSubscriptions.get(mediaId).unsubscribe();
        this.activeSubscriptions.delete(mediaId);
      }
    };
  }

  /**
   * Subscribe to all media processing updates for a post
   * @param {string} postId - The post ID
   * @param {Function} onUpdate - Callback for status updates
   * @returns {Function} Unsubscribe function
   */
  subscribeToPostMediaStatus(postId, onUpdate) {
    const subscriptionKey = `post_${postId}`;
    
    if (this.activeSubscriptions.has(subscriptionKey)) {
      this.activeSubscriptions.get(subscriptionKey).unsubscribe();
    }

    const subscription = graphqlClient.subscribe({
      query: POST_MEDIA_STATUS_SUBSCRIPTION,
      variables: { postId }
    }).subscribe({
      next: (result) => {
        if (result.data?.postMediaStatusUpdated && onUpdate) {
          onUpdate(result.data.postMediaStatusUpdated);
        }
      },
      error: (error) => {
        console.error('Post media subscription error:', error);
        onUpdate?.({ error: error.message });
      }
    });

    this.activeSubscriptions.set(subscriptionKey, subscription);
    
    return () => {
      if (this.activeSubscriptions.has(subscriptionKey)) {
        this.activeSubscriptions.get(subscriptionKey).unsubscribe();
        this.activeSubscriptions.delete(subscriptionKey);
      }
    };
  }

  /**
   * Cancel all active subscriptions
   */
  unsubscribeAll() {
    this.activeSubscriptions.forEach(subscription => {
      subscription.unsubscribe();
    });
    this.activeSubscriptions.clear();
  }

  /**
   * Get processing status as human-readable text
   * @param {string} status - The processing status
   * @returns {string} Human-readable status
   */
  getStatusText(status) {
    const statusMap = {
      'pending': 'Waiting to process',
      'processing': 'Processing...',
      'completed': 'Processing complete',
      'failed': 'Processing failed',
      'retrying': 'Retrying...',
      'cancelled': 'Cancelled'
    };
    
    return statusMap[status] || status;
  }

  /**
   * Check if media is ready for use
   * @param {object} media - The media object
   * @returns {boolean} True if media is ready
   */
  isMediaReady(media) {
    return media.status === 'completed' && media.processedUrl;
  }

  /**
   * Get thumbnail URL if available, otherwise use processed URL
   * @param {object} media - The media object
   * @returns {string} URL to display
   */
  getDisplayUrl(media) {
    return media.thumbnailUrl || media.processedUrl || media.originalUrl;
  }

  /**
   * Format file size for display
   * @param {number} bytes - File size in bytes
   * @returns {string} Formatted size string
   */
  formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  /**
   * Validate file before upload
   * @param {File} file - File to validate
   * @returns {object} Validation result
   */
  validateFile(file) {
    const errors = [];
    
    if (file.size > maxFileSize) {
      errors.push(`File size exceeds ${this.formatFileSize(maxFileSize)}`);
    }
    
    const isValidType = allowedTypes.some(type => {
      if (type.endsWith('/*')) {
        return file.type.startsWith(type.slice(0, -2));
      }
      return file.type === type;
    });
    
    if (!isValidType) {
      errors.push(`File type ${file.type} is not supported`);
    }
    
    return {
      valid: errors.length === 0,
      errors
    };
  }
}

// Create and export singleton instance
export const desktopMediaService = new DesktopMediaService();

// Export for testing
export { DesktopMediaService };