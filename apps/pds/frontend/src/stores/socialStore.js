import { writable } from 'svelte/store';
import { graphqlClient } from '../lib/graphql/client.js';

// GraphQL queries and mutations
const SHARE_EXPERIENCE_MUTATION = `
    mutation ShareExperience($input: ShareExperienceInput!) {
        shareExperience(input: $input) {
            experience {
                id
                ownerId
                title
                description
                visibility
                contentHash
                fileSize
                createdAt
            }
            shareUrl
        }
    }
`;

const INVITE_FRIEND_MUTATION = `
    mutation InviteFriend($userId: ID!) {
        inviteFriend(userId: $userId) {
            invitationId
            invitationCode
            status
        }
    }
`;

const ADD_COMMENT_MUTATION = `
    mutation AddComment($experienceId: ID!, $content: String!) {
        addComment(experienceId: $experienceId, content: $content) {
            comment {
                id
                experienceId
                authorId
                content
                createdAt
            }
        }
    }
`;

const NEW_COMMENTS_SUBSCRIPTION = `
    subscription NewComments($experienceId: ID!) {
        newComments(experienceId: $experienceId) {
            id
            experienceId
            authorId
            content
            createdAt
        }
    }
`;

// Store for social features
function createSocialStore() {
    const { subscribe, set, update } = writable({
        experiences: [],
        comments: {},
        invitations: [],
        loading: false,
        error: null
    });

    return {
        subscribe,
        
        async shareExperience(input) {
            try {
                const result = await graphqlClient.mutation(SHARE_EXPERIENCE_MUTATION, {
                    input: {
                        experienceId: input.experienceId,
                        title: input.title,
                        description: input.description,
                        visibility: input.visibility,
                        contentHash: input.contentHash,
                        fileSize: input.fileSize
                    }
                });
                
                return {
                    success: true,
                    experience: result.data.shareExperience.experience,
                    shareUrl: result.data.shareExperience.shareUrl
                };
            } catch (error) {
                return {
                    success: false,
                    error: error.message
                };
            }
        },
        
        async inviteFriend(userId) {
            try {
                const result = await graphqlClient.mutation(INVITE_FRIEND_MUTATION, {
                    userId
                });
                
                return {
                    success: true,
                    invitationId: result.data.inviteFriend.invitationId,
                    invitationCode: result.data.inviteFriend.invitationCode
                };
            } catch (error) {
                return {
                    success: false,
                    error: error.message
                };
            }
        },
        
        async addComment(experienceId, content) {
            try {
                const result = await graphqlClient.mutation(ADD_COMMENT_MUTATION, {
                    experienceId,
                    content
                });
                
                return result.data.addComment.comment;
            } catch (error) {
                throw new Error(error.message);
            }
        },
        
        async getComments(experienceId) {
            try {
                // This would be a GraphQL query in a real implementation
                // For now, using a mock implementation
                return [];
            } catch (error) {
                throw new Error(error.message);
            }
        },
        
        async getExperience(experienceId) {
            try {
                // This would be a GraphQL query in a real implementation
                // For now, returning mock data
                return {
                    id: experienceId,
                    title: 'Sample Experience',
                    description: 'A sample AR experience',
                    visibility: 'FRIENDS',
                    contentHash: 'abc123',
                    fileSize: 1024
                };
            } catch (error) {
                throw new Error(error.message);
            }
        },
        
        subscribeToComments(experienceId, callback) {
            // This would set up a WebSocket subscription in a real implementation
            // For now, returning a mock unsubscribe function
            return () => {};
        }
    };
}

export const socialStore = createSocialStore();