// Custom commands for media post workflow testing

Cypress.Commands.add('uploadMedia', (fileName, fileContent, mimeType = 'image/jpeg') => {
  cy.get('[data-cy="media-upload-input"]').selectFile({
    contents: Cypress.Buffer.from(fileContent),
    fileName,
    mimeType
  });
});

Cypress.Commands.add('createPostWithMedia', (content, mediaFiles = []) => {
  cy.get('[data-cy="create-post-button"]').click();
  cy.get('[data-cy="post-composer"]').should('be.visible');
  
  // Upload media if provided
  mediaFiles.forEach(file => {
    cy.uploadMedia(file.name, file.content, file.type);
  });
  
  // Add content
  cy.get('[data-cy="post-content-input"]').type(content);
  
  // Submit
  cy.get('[data-cy="submit-post-button"]').click();
});

Cypress.Commands.add('waitForMediaProcessing', (mediaId, timeout = 10000) => {
  cy.intercept('POST', '/graphql', (req) => {
    if (req.body.operationName === 'GetMediaProcessingStatus') {
      req.reply({
        data: {
          media: {
            id: mediaId,
            status: 'PROCESSED',
            __typename: 'Media'
          }
        }
      });
    }
  });
  
  cy.get('[data-cy="processing-indicator"]', { timeout }).should('not.exist');
});

Cypress.Commands.add('mockGraphQL', (operationName, response) => {
  cy.intercept('POST', '/graphql', (req) => {
    if (req.body.operationName === operationName) {
      req.reply(response);
    }
  });
});

// Mock media service
Cypress.Commands.add('mockMediaService', () => {
  cy.intercept('POST', '/api/media/upload', {
    statusCode: 200,
    body: {
      id: 'mock-media-id',
      url: 'https://cdn.example.com/processed/mock-media-id',
      thumbnailUrl: 'https://cdn.example.com/thumb/mock-media-id'
    }
  });
  
  cy.intercept('GET', '/api/media/status/*', {
    statusCode: 200,
    body: {
      id: 'mock-media-id',
      status: 'PROCESSED',
      progress: 100
    }
  });
});

// Performance testing helpers
Cypress.Commands.add('measureUploadTime', (fileName, fileContent) => {
  const startTime = Date.now();
  
  cy.uploadMedia(fileName, fileContent);
  
  cy.get('[data-cy="media-preview"]').should('be.visible').then(() => {
    const uploadTime = Date.now() - startTime;
    cy.log(`Upload completed in ${uploadTime}ms`);
    return cy.wrap(uploadTime);
  });
});

// Error simulation
Cypress.Commands.add('simulateUploadError', (errorCode = 'UPLOAD_ERROR') => {
  cy.intercept('POST', '/graphql', (req) => {
    if (req.body.operationName === 'UploadMedia') {
      req.reply({
        errors: [{
          message: 'Upload failed',
          extensions: { code: errorCode }
        }]
      });
    }
  });
});

Cypress.Commands.add('simulateProcessingError', () => {
  cy.intercept('POST', '/graphql', (req) => {
    if (req.body.operationName === 'GetMediaProcessingStatus') {
      req.reply({
        data: {
          media: {
            id: 'test-media-id',
            status: 'FAILED',
            error: 'Processing failed',
            __typename: 'Media'
          }
        }
      });
    }
  });
});

// Cleanup utilities
Cypress.Commands.add('cleanupTestData', () => {
  cy.request({
    method: 'POST',
    url: '/api/test/cleanup',
    body: {
      type: 'media',
      userId: 'test-user'
    }
  });
});

// Accessibility testing
Cypress.Commands.add('testAccessibility', () => {
  cy.injectAxe();
  cy.checkA11y();
});

// Visual regression testing
Cypress.Commands.add('testVisualRegression', (testName) => {
  cy.matchImageSnapshot(testName);
});

// Network throttling
Cypress.Commands.add('throttleNetwork', (speed = 'slow') => {
  const speeds = {
    slow: { download: 50, upload: 20 },
    medium: { download: 100, upload: 50 },
    fast: { download: 1000, upload: 500 }
  };
  
  cy.window().then((win) => {
    if (win.navigator.connection) {
      Object.defineProperty(win.navigator, 'connection', {
        value: new NetworkInformation(speeds[speed]),
        writable: true
      });
    }
  });
});