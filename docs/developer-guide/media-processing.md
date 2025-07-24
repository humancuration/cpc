# Developer Guide: Media Processing Workflow

*Last updated: 2025-07-23*

This guide provides developers with a detailed understanding of the media processing workflow in the CPC platform.

## Overview

The media processing workflow is designed to be asynchronous and resilient. When a user uploads a media file, it is first stored in a temporary location and then queued for processing. This processing can include tasks like generating thumbnails, transcoding videos to different formats, and optimizing images for the web.

## The Workflow

1.  **Initiate Upload**: The client calls the `createMediaUpload` GraphQL mutation. This creates a `MediaItem` in the database with a `processingStatus` of `Pending` and returns a presigned URL for the client to upload the file to.
2.  **File Upload**: The client uploads the file directly to the provided URL.
3.  **Processing Trigger**: Once the upload is complete, the client will call the `completeMediaUpload` mutation. This will trigger the media processing.
4.  **gRPC Call to `cpc-node`**: The backend (Axum server) sends a gRPC request to a `cpc-node` worker to process the media. This request includes the media ID and the location of the uploaded file.
5.  **Processing in `cpc-node`**: The `cpc-node` worker performs the necessary processing tasks. As it works, it sends status updates back to the backend via a gRPC server stream. These updates include the current status (`Processing`, `Completed`, `Failed`) and any relevant progress information.
6.  **Status Updates**: The backend receives these status updates and persists them to the `MediaItem` record in the database.
7.  **GraphQL Subscription**: The backend broadcasts these status changes to any clients subscribed to the `mediaStatus` GraphQL subscription for that `mediaId`. This allows the UI to display real-time feedback to the user.

## Error Handling and Retries

Our system is designed to handle failures gracefully.

*   **Processing Failure**: If a `cpc-node` worker fails to process a file, it will report a `Failed` status back to the backend. This status will be saved to the database and sent to the client. The UI should display an appropriate error message to the user.
*   **Retry Mechanism**: Currently, there is no automatic retry mechanism for failed processing jobs. This is a potential area for future improvement. A failed job can be manually retried through an administrative interface or by having the user re-upload the file.

## Testing the Workflow

The media processing workflow can be tested end-to-end. Our test suite includes tests that simulate the entire process, from uploading a file to verifying that it has been processed correctly. These tests can be found in `apps/backend/tests/e2e_media_post_workflow.rs`.