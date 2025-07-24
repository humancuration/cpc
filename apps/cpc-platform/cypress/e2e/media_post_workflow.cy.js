describe('Media Post Workflow E2E Tests', () => {
  beforeEach(() => {
    cy.visit('/');
    cy.intercept('POST', '/graphql', (req) => {
      // Mock GraphQL responses
      if (req.body.operationName === 'UploadMedia') {
        req.reply({
          data: {
            uploadMedia: {
              id: 'test-media-id',
              status: 'PROCESSING',
              __typename: 'Media'
            }
          }
        });
      }
      
      if (req.body.operationName === 'CreatePost') {
        req.reply({
          data: {
            createPost: {
              id: 'test-post-id',
              content: req.body.variables.content,
              media: req.body.variables.mediaIds.map(id => ({
                id,
                url: `https://cdn.example.com/processed/${id}`,
                thumbnailUrl: `https://cdn.example.com/thumb/${id}`,
                __typename: 'Media'
              })),
              __typename: 'Post'
            }
          }
        });
      }
      
      if (req.body.operationName === 'GetFeed') {
        req.reply({
          data: {
            feed: {
              posts: [{
                id: 'test-post-id',
                content: 'Test post with media',
                media: [{
                  id: 'test-media-id',
                  url: 'https://cdn.example.com/processed/test-media-id',
                  thumbnailUrl: 'https://cdn.example.com/thumb/test-media-id',
                  __typename: 'Media'
                }],
                __typename: 'Post'
              }],
              __typename: 'Feed'
            }
          }
        });
      }
    });
    
    // Mock WebSocket for subscriptions
    cy.intercept('GET', '/graphql/subscriptions', (req) => {
      req.reply({
        statusCode: 101,
        headers: {
          'upgrade': 'websocket',
          'connection': 'upgrade',
        }
      });
    });
  });

  describe('Happy Path Tests', () => {
    it('should complete full media post workflow', () => {
      // 1. Open post composer
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="post-composer"]').should('be.visible');
      
      // 2. Upload media
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg',
        lastModified: Date.now()
      });
      
      // 3. Verify upload progress
      cy.get('[data-cy="upload-progress"]').should('be.visible');
      cy.get('[data-cy="upload-progress"]').should('contain', 'Uploading...');
      
      // 4. Wait for processing
      cy.get('[data-cy="media-preview"]').should('be.visible');
      cy.get('[data-cy="processing-indicator"]').should('not.exist');
      
      // 5. Add post content
      cy.get('[data-cy="post-content-input"]').type('Check out this amazing photo!');
      
      // 6. Submit post
      cy.get('[data-cy="submit-post-button"]').click();
      
      // 7. Verify post appears in feed
      cy.get('[data-cy="feed-container"]').should('contain', 'Check out this amazing photo!');
      cy.get('[data-cy="media-attachment"]').should('be.visible');
    });

    it('should handle multiple media uploads', () => {
      cy.get('[data-cy="create-post-button"]').click();
      
      // Upload multiple files
      const files = [
        { contents: Cypress.Buffer.from('image1'), fileName: 'image1.jpg', mimeType: 'image/jpeg' },
        { contents: Cypress.Buffer.from('image2'), fileName: 'image2.jpg', mimeType: 'image/jpeg' },
        { contents: Cypress.Buffer.from('image3'), fileName: 'image3.jpg', mimeType: 'image/jpeg' }
      ];
      
      cy.get('[data-cy="media-upload-input"]').selectFile(files, { action: 'select' });
      
      // Verify all uploads are shown
      cy.get('[data-cy="media-preview"]').should('have.length', 3);
      
      // Complete post
      cy.get('[data-cy="post-content-input"]').type('Multiple photos!');
      cy.get('[data-cy="submit-post-button"]').click();
      
      cy.get('[data-cy="feed-container"]').should('contain', 'Multiple photos!');
      cy.get('[data-cy="media-attachment"]').should('have.length', 3);
    });
  });

  describe('Error Handling Tests', () => {
    it('should handle failed upload gracefully', () => {
      cy.intercept('POST', '/graphql', (req) => {
        if (req.body.operationName === 'UploadMedia') {
          req.reply({
            errors: [{
              message: 'Upload failed',
              extensions: { code: 'UPLOAD_ERROR' }
            }]
          });
        }
      });
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="error-message"]').should('contain', 'Upload failed');
      cy.get('[data-cy="retry-upload-button"]').should('be.visible');
    });

    it('should handle processing failure', () => {
      cy.intercept('POST', '/graphql', (req) => {
        if (req.body.operationName === 'UploadMedia') {
          req.reply({
            data: {
              uploadMedia: {
                id: 'test-media-id',
                status: 'PROCESSING',
                __typename: 'Media'
              }
            }
          });
        }
      });
      
      // Mock WebSocket to send processing failure
      cy.window().then((win) => {
        cy.stub(win, 'WebSocket').callsFake((url) => {
          const ws = {
            url,
            readyState: 1,
            send: cy.stub(),
            close: cy.stub(),
            onmessage: null,
            onopen: null,
            onclose: null
          };
          
          setTimeout(() => {
            ws.onmessage && ws.onmessage({
              data: JSON.stringify({
                type: 'media_processing_failed',
                payload: {
                  mediaId: 'test-media-id',
                  error: 'Processing failed'
                }
              })
            });
          }, 1000);
          
          return ws;
        });
      });
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="processing-error"]').should('contain', 'Processing failed');
      cy.get('[data-cy="retry-processing-button"]').should('be.visible');
    });

    it('should handle network interruption', () => {
      cy.intercept('POST', '/graphql', (req) => {
        req.destroy(); // Simulate network failure
      }).as('uploadRequest');
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="network-error"]').should('contain', 'Network error');
      cy.get('[data-cy="retry-upload-button"]').should('be.visible');
    });
  });

  describe('Edge Cases', () => {
    it('should handle large file uploads', () => {
      const largeFile = new Array(5 * 1024 * 1024).fill('a').join(''); // 5MB
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from(largeFile),
        fileName: 'large-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="upload-progress"]').should('be.visible');
      cy.get('[data-cy="file-size-warning"]').should('not.exist');
    });

    it('should handle file size limit exceeded', () => {
      const hugeFile = new Array(50 * 1024 * 1024).fill('a').join(''); // 50MB
      
      cy.intercept('POST', '/graphql', (req) => {
        if (req.body.operationName === 'UploadMedia') {
          req.reply({
            errors: [{
              message: 'File size exceeds limit',
              extensions: { code: 'FILE_TOO_LARGE' }
            }]
          });
        }
      });
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from(hugeFile),
        fileName: 'huge-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="error-message"]').should('contain', 'File size exceeds limit');
    });

    it('should allow cancelling upload', () => {
      cy.intercept('POST', '/graphql', (req) => {
        req.on('response', (res) => {
          res.setDelay(2000); // Delay to simulate slow upload
        });
      });
      
      cy.get('[data-cy="create-post-button"]').click();
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="cancel-upload-button"]').click();
      cy.get('[data-cy="upload-cancelled-message"]').should('be.visible');
    });
  });

  describe('Performance Tests', () => {
    it('should measure upload time', () => {
      cy.get('[data-cy="create-post-button"]').click();
      
      const startTime = Date.now();
      
      cy.get('[data-cy="media-upload-input"]').selectFile({
        contents: Cypress.Buffer.from('fake image data'),
        fileName: 'test-image.jpg',
        mimeType: 'image/jpeg'
      });
      
      cy.get('[data-cy="media-preview"]').should('be.visible').then(() => {
        const uploadTime = Date.now() - startTime;
        cy.log(`Upload completed in ${uploadTime}ms`);
        expect(uploadTime).to.be.lessThan(5000); // Should complete within 5 seconds
      });
    });

    it('should handle concurrent users', () => {
      // Simulate multiple users uploading simultaneously
      const createUserSession = (userId) => {
        return cy.session(`user-${userId}`, () => {
          cy.visit('/');
          cy.window().then((win) => {
            win.localStorage.setItem('userId', userId);
          });
        });
      };
      
      // Test 3 concurrent users