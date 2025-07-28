# Document Editor Module

This module provides a word processor with basic formatting, image insertion, and PDF/DOCX export capabilities.

## Features

- Create, read, update, and delete documents
- Share documents with other users
- Version control for documents
- Export documents to PDF and DOCX formats
- Basic text formatting (bold, italic, underline)
- Image insertion
- Real-time collaboration (planned)

## Architecture

The document editor follows a hexagonal architecture pattern with the following layers:

- **Domain**: Core business logic and entities
- **Application**: Use cases and services
- **Infrastructure**: Database implementations, file storage, etc.
- **Presentation**: UI components
- **Web**: GraphQL API

## Database Schema

The module uses the following tables:

- `documents`: Stores document metadata and content
- `document_shares`: Manages document sharing permissions
- `document_versions`: Tracks document version history

## GraphQL API

The module exposes a GraphQL API for document management:

- Queries:
  - `document(id: UUID!)`: Get a document by ID
  - `documentsByOwner(ownerId: UUID!)`: Get all documents owned by a user

- Mutations:
  - `createDocument(input: CreateDocumentInput!)`: Create a new document
  - `updateDocumentContent(input: UpdateDocumentContentInput!)`: Update document content
  - `updateDocumentTitle(input: UpdateDocumentTitleInput!)`: Update document title
  - `deleteDocument(documentId: UUID!)`: Delete a document
  - `shareDocument(input: ShareDocumentInput!)`: Share a document with another user
  - `exportDocument(input: ExportInput!)`: Export a document to PDF or DOCX

## Testing

The module includes unit tests for all components. To run the tests:

```bash
cd packages/cpc-core/document_editor
cargo test
```

Note: Some tests require a PostgreSQL database to be available.