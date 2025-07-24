# GraphQL Schema Documentation

*Last updated: 2025-07-23*

This document provides an overview of the CPC platform's GraphQL schema.

## Core Concepts

Our GraphQL API is the primary interface for our web and desktop clients. It is organized around the core features of the platform, such as social interactions, user management, and asset handling.

## Schema Reference

### Object Types

#### `Post`

Represents a social post made by a user.

| Field | Type | Description |
|---|---|---|
| `id` | `ID!` | The unique identifier for the post. |
| `content` | `String!` | The text content of the post. |
| `author` | `User!` | The user who created the post. |
| `createdAt` | `DateTime!` | The timestamp when the post was created. |
| `visibility` | `Visibility!` | The visibility setting for the post (e.g., `Public`, `Private`). |
| `media` | `[MediaItem!]!` | A list of media items attached to the post. |

#### `MediaItem`

Represents a piece of media, such as an image or video.

| Field | Type | Description |
|---|---|---|
| `id` | `ID!` | The unique identifier for the media item. |
| `url` | `String!` | The URL where the media can be accessed. |
| `mediaType` | `MediaType!` | The type of media (e.g., `Image`, `Video`). |
| `processingStatus` | `ProcessingStatus!` | The current processing status of the media (`Pending`, `Processing`, `Completed`, `Failed`). |

### Input Types

#### `CreatePostInput`

Used in the `createPost` mutation to create a new post.

| Field | Type | Description |
|---|---|---|
| `content` | `String!` | The text content of the post. |
| `visibility` | `Visibility!` | The visibility setting for the post. |
| `cooperativeId` | `ID` | The ID of the cooperative to associate the post with, if any. |
| `media_ids` | `[ID!]` | A list of `MediaItem` IDs to attach to the post. |

#### `CreateMediaUploadInput`

Used in the `createMediaUpload` mutation to get a presigned URL for an upload.

| Field | Type | Description |
|---|---|---|
| `filename` | `String!` | The name of the file to be uploaded. |
| `contentType` | `String!` | The MIME type of the file. |
| `fileSize` | `Int!` | The size of the file in bytes. |

### Mutations

#### `createPost(input: CreatePostInput!): Post!`

Creates a new social post. It takes a `CreatePostInput` object and returns the newly created `Post`.

#### `createMediaUpload(input: CreateMediaUploadInput!): MediaUpload!`

Initiates a media upload. It returns a `MediaUpload` object containing a presigned URL for the upload and the `mediaId` for the new `MediaItem`.

### Subscriptions

#### `mediaStatus(mediaId: ID!): ProcessingUpdate!`

Subscribes to real-time processing updates for a specific `MediaItem`. This is used to monitor the progress of an upload and update the UI accordingly.

| Field | Type | Description |
|---|---|---|
| `media_id` | `ID!` | The ID of the media item. |
| `status` | `ProcessingStatus!` | The current processing status. |
| `progress` | `Int` | The processing progress, from 0 to 100. |
| `message` | `String` | An optional message about the current status. |
| `error` | `String` | An error message, if the processing failed. |