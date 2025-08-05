# Feed Utilities

Utility functions for feed management and content preview generation.

## Overview

This crate provides utilities for generating content previews and managing feeds in the CPC platform. It includes:

- Preview service for generating document previews
- Content item conversion utilities
- Feed aggregation helpers

## Features

- **Document Previews**: Generate lightweight previews of collaborative documents
- **Content Conversion**: Convert document previews to social graph content items
- **Feed Integration**: Seamless integration with the social graph feed system

## Usage

### Preview Service

```rust
use feed_utils::PreviewService;
use collaborative_docs::CollaborativeDocService;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Assuming you have a document service
    let doc_service = Arc::new(CollaborativeDocService::new(/* ... */));
    
    // Create preview service
    let preview_service = PreviewService::new(doc_service);
    
    // Generate a document preview
    let document_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    
    match preview_service.get_document_preview(document_id, user_id).await {
        Ok(preview) => {
            println!("Document preview: {}", preview.title);
            println!("Excerpt: {}", preview.excerpt);
        }
        Err(e) => {
            eprintln!("Failed to generate preview: {}", e);
        }
    }
    
    Ok(())
}
```

## Integration

The preview service is designed to work with the collaborative document system and social graph to provide rich previews in user feeds.