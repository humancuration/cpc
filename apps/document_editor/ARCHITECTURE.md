# Document Editor Module Architecture

This document outlines the architecture for the document_editor module, implementing the word processor functionality as requested in `docs/planned_apps.md` line 21. The design follows our hexagonal architecture principles and screaming architecture pattern as documented in planned_apps.md (lines 154-182).

## 1. Module Overview and Responsibilities

The Document Editor module provides a streamlined word processing capability with focus on:

- **Core Document Creation & Editing**: Basic text formatting, document structure, and content management
- **Rich Media Integration**: Insertion and management of images within documents
- **Document Export**: Conversion to industry-standard formats (PDF, DOCX)
- **Storage & Sharing**: Secure storage of documents with sharing capabilities

This module adheres to our cooperative principles by:
- Prioritizing user control over document ownership and sharing
- Supporting privacy through granular consent controls
- Enabling collaboration while maintaining data sovereignty

## 2. Directory Structure Following Vertical Slice Pattern

```
├── Cargo.toml
├── migrations/
│   └── 20250801000000_create_document_tables.sql
└── src/
    ├── lib.rs                  # Main crate entry, exports the module
    ├── domain/                 # Core business models
    │   ├── models.rs           # Primary entities
    │   ├── value_objects.rs    # Domain-specific types
    │   └── errors.rs           # Custom error types
    ├── application/            # Business logic services
    │   ├── document_service.rs # Core operations for documents
    │   ├── export_service.rs   # Document export operations
    │   └── collaboration_service.rs # Real-time collaboration (future)
    ├── infrastructure/         # External implementations
    │   ├── repository.rs       # Database access layer
    │   ├── p2p_store.rs        # p2panda integration for distributed storage
    │   ├── media_processor.rs  # Image/video processing integration
    │   ├── pdf_exporter.rs     # PDF export implementation
    │   └── docx_exporter.rs    # DOCX export implementation
    └── presentation/           # UI components (Yew)
        ├── mod.rs
        ├── editor.rs           # Main document editor component
        ├── toolbar.rs          # Formatting toolbar
        └── preview.rs          # Document preview component
```

## 3. Component Diagram Showing Domain Layers

```
┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                       Presentation                                                  │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │    Yew Components │──────▶│   GraphQL Queries   │──────▶│  Document Service │──────▶│        Domain Models        │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐                                       │
│ │    Mobile UI      │──────▶│ Export Operations   │──────▶│   Export Service  │                                       │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘                                       │
│                                                                                                                       │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
                                     ▲                               ▲                               ▲
                                     │                               │                               │
                                     ▼                               ▼                               ▼
┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                                     Infrastructure                                                    │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │     SQL Database  │◀─────▶│   Document Repository├──────▶│    PDF Exporter   │◀─────▶│       pdf-rs library        │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐       ┌───────────────────┐       ┌─────────────────────────────┐ │
│ │    p2p Storage    │◀─────▶│     P2P Adapter     ├──────▶│   DOCX Exporter   │◀─────▶│    docx-rs library (MIT)    │ │
│ └───────────────────┘       └─────────────────────┘       └───────────────────┘       └─────────────────────────────┘ │
│                                                                                                                       │
│ ┌───────────────────┐       ┌─────────────────────┐                                                                 │ │
│ │    Media Assets   │◀─────▶│   Media Processor   │                                                                 │ │
│ └───────────────────┘       └─────────────────────┘                                                                 │ │
│                                                                                                                       │
└───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

## 4. Data Flow for Key Operations

### Document Creation Workflow
1. User initiates document creation via GraphQL mutation
2. DocumentService creates a new Document entity with proper ownership
3. Repository stores the document in SQL database
4. System returns Document ID to presentation layer
5. Editor component loads empty document for editing

### Document Editing Workflow
1. User makes changes in the Yew editor component
2. Changes are batched and sent as operations via GraphQL mutation
3. DocumentService validates operations against document permissions
4. Domain models apply changes while maintaining document integrity
5. Repository persists the updated document state
6. p2p_store propagates changes to distributed storage (if enabled)

### Document Export Workflow
1. User selects export format via UI
2. ExportService is called with Document ID and format parameter
3. Document content is retrieved from repository
4. Appropriate exporter (PDF or DOCX) is invoked with document content
5. MediaProcessor processes embedded images for appropriate format
6. Exported document is generated as binary stream
7. Binary stream is returned to client via GraphQL subscription

## 5. Integration Points with Other Modules



The module will provide the following integration points for other modules:

- **Document Embedding API**: Allow other modules to embed document previews
- **Collaboration Hooks**: Future real-time collaboration context
- **Document Metadata API**: Access to document properties and ownership info

## 6. Technology Choices

### Core Components
- **Domain Logic**: Pure Rust with strong type system
- **Database**: PostgreSQL via SQLx (already in workspace dependencies)
- **P2P Storage**: p2panda-core for distributed document storage
- **Tracing**: tracing crate for observability

### Export Capabilities
- **PDF Export**: `pdf = "0.7"` (already in workspace dependencies)
  - Permissively licensed (Apache 2.0)
  - Supports text, basic formatting, and image embedding
  - Production-ready with active maintenance

- **DOCX Export**: `docx-rs = "0.7"` (to be added with MIT license)
  - MIT licensed (verified compliance)
  - Supports basic formatting, lists, and image insertion
  - Actively maintained with good documentation
  - Alternative considered: `simple_docx` (but lacks image support)

### UI Components
- **Frontend Framework**: Yew (already in tech stack)
  - Replacing Svelte as per refactoring plans
  - WASM-based for consistent experience across platforms
  - Mobile-responsive design patterns

### Media Processing
- **Image Handling**: Integration with existing media processing infrastructure
  - Uses royalty-free codecs (AVIF for optimized images)
  - Resizes and optimizes images for document embedding
  - Maintains original quality options for export

## 7. Security Considerations for Document Storage

### Data Protection
- **End-to-End Encryption**: Documents encrypted at rest using AES-256
- **Access Control**: Granular permissions system based on cooperative roles
- **Audit Logs**: Comprehensive logging of document access and modifications
- **Data Minimization**: Only necessary metadata stored outside encrypted content

### Storage Architecture
- **Local-First**: Documents primarily stored on user's device
- **Optional Sync**: Users can opt into p2p synchronization
- **Consent-Based Sharing**: Explicit consent required for document sharing
- **Revocable Access**: Owners can revoke access at any time

### Privacy Safeguards
- **Metadata Protection**: Limited metadata exposure in p2p network
- **Anonymized Analytics**: Usage data aggregated and anonymized
- **Zero-Knowledge Proof**: Optional verification without content disclosure
- **Data Sovereignty**: Users control where documents are stored

## 8. Future Expansion Points

### Immediate Roadmap
- **Real-time Collaboration**: Building on p2panda's orchestration capabilities
- **Version History**: Document versioning with diff capabilities
- **Templates System**: Pre-designed templates for common document types
- **Offline Editing**: Enhanced offline capabilities with sync on reconnect

### Medium-Term
- **Document AI**: Smart suggestions and content analysis
- **Accessibility Tools**: Enhanced screen reader support and accessibility checks
- **Collaborative Review**: Commenting and suggestion workflows
- **Multi-format Import**: Support for importing from other document formats

### Long-Term
- **Document Verification**: Cryptographic verification of document authenticity
- **Smart Contracts**: Integration with cooperative governance documents
- **Cross-Document Linking**: Knowledge graph capabilities between documents
- **AI-Powered Drafting**: Context-aware content generation

## 9. Compliance Verification

✅ Hexagonal Architecture with clear domain/application/infrastructure separation  
✅ Screaming Architecture reflecting document editing capabilities   
✅ Permissively licensed dependencies (MIT/Apache 2.0)  
✅ Integration with existing media processing infrastructure  
✅ Support for royalty-free image formats  
✅ Mobile-first responsive UI design  
✅ Data privacy and security considerations addressed  
✅ Export capabilities meeting requirements (PDF/DOCX)  
✅ Future-proof design for real-time collaboration