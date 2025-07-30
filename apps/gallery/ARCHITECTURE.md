# Gallery Module Architecture

## Location
`packages/apps/gallery/`

## Purpose and Scope
The Gallery module provides functionality for managing and displaying media collections including images, videos, and audio files. It supports organizing media into albums, extracting metadata, and providing various viewing experiences.

The module handles:
- Media file management (upload, storage, retrieval)
- Album creation and organization
- Metadata extraction and storage
- Media transcoding for web-compatible formats (AV1/Opus/WebM)
- Visualization of media statistics and analytics
- Integration with the federation's distributed storage

This module is designed to be a standalone application within the CPC ecosystem, with its own database schema and web interface while following the hexagonal architecture pattern.