import { graphqlClient } from '$lib/graphql/client';
import { mediaProcessingSubscription } from '$lib/graphql/subscriptions';

class MediaSubscriptionService {
  constructor() {
    this.subscriptions = new Map();
  }
  
  subscribeToMediaStatus(mediaId, onUpdate) {
    if (this.subscriptions.has(mediaId)) {
      console.warn(`Already subscribed to media ${mediaId}`);
      return;
    }
    
    const subscription = graphqlClient
      .subscription(mediaProcessingSubscription, { mediaId })
      .subscribe({
        next: ({ data }) => {
          if (data?.mediaProcessingStatus) {
            onUpdate(data.mediaProcessingStatus);
          }
        },
        error: (error) => {
          console.error(`Error in media subscription for ${mediaId}:`, error);
          this.unsubscribeFromMediaStatus(mediaId);
        }
      });
    
    this.subscriptions.set(mediaId, subscription);
  }
  
  unsubscribeFromMediaStatus(mediaId) {
    const subscription = this.subscriptions.get(mediaId);
    if (subscription) {
      subscription.unsubscribe();
      this.subscriptions.delete(mediaId);
    }
  }
  
  unsubscribeAll() {
    this.subscriptions.forEach((subscription) => subscription.unsubscribe());
    this.subscriptions.clear();
  }
}

export const mediaSubscriptionService = new MediaSubscriptionService();