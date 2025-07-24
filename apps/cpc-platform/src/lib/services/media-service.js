import { graphqlClient } from '$lib/graphql/client';
import { UPLOAD_MEDIA } from '$lib/graphql/mutations';

export async function uploadMedia(files, postId = null) {
  const results = [];
  
  for (const file of files) {
    try {
      // Create FormData for multipart upload
      const formData = new FormData();
      formData.append('file', file);
      
      // Upload via REST API (as GraphQL file upload is complex)
      const response = await fetch(
        `http://localhost:3000/api/media/upload?post_id=${postId || ''}&media_type=${getMediaType(file.type)}`,
        {
          method: 'POST',
          body: formData,
        }
      );
      
      if (!response.ok) {
        throw new Error(`Upload failed: ${response.statusText}`);
      }
      
      const result = await response.json();
      results.push({
        id: result.media_id,
        url: result.url,
        type: mapMediaType(file.type),
        thumbnailUrl: null,
        description: file.name
      });
    } catch (error) {
      console.error('Failed to upload media:', error);
      results.push({ error: error.message });
    }
  }
  
  return results;
}

export function getMediaType(mimeType) {
  if (mimeType.startsWith('image/')) return 'image';
  if (mimeType.startsWith('video/')) return 'video';
  if (mimeType.startsWith('audio/')) return 'audio';
  return 'unknown';
}

export function mapMediaType(mimeType) {
  if (mimeType.startsWith('image/')) return 'IMAGE';
  if (mimeType.startsWith('video/')) return 'VIDEO';
  if (mimeType.startsWith('audio/')) return 'AUDIO';
  return 'UNKNOWN';
}

export async function getMediaStatus(mediaId) {
  try {
    const response = await fetch(`http://localhost:3000/api/media/${mediaId}/status`);
    if (!response.ok) {
      throw new Error('Failed to get media status');
    }
    return await response.json();
  } catch (error) {
    console.error('Failed to get media status:', error);
    throw error;
  }
}

export async function getMediaForPost(postId) {
  try {
    // This would typically be handled via GraphQL queries
    // For now, return empty array as media is included in post queries
    return [];
  } catch (error) {
    console.error('Failed to get media for post:', error);
    return [];
  }
}

export async function processMedia(mediaId) {
  try {
    // Media processing is handled server-side after upload
    // This function is mainly for compatibility
    return { success: true };
  } catch (error) {
    console.error('Failed to process media:', error);
    throw error;
  }
}