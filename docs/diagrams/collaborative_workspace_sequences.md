# Collaborative Workspace Sequence Diagrams

This document contains sequence diagrams for the collaborative workspace flows implemented in the CPC platform.

## 1. Real-time Document Collaboration Flow

```mermaid
sequenceDiagram
    participant UserA
    participant UserB
    participant GraphQL_API
    participant DocumentService
    participant DocumentRepository
    participant EventBus
    participant WebSocket
    participant NotificationService

    UserA->>GraphQL_API: applyDocumentOperation(documentId, operation)
    GraphQL_API->>DocumentService: apply_operation(userId, documentId, operation)
    
    DocumentService->>DocumentRepository: get_document_state(documentId)
    DocumentRepository-->>DocumentService: current_document_state
    
    Note over DocumentService: Apply CRDT operation to document state
    
    DocumentService->>DocumentRepository: update_document_state(documentId, newState)
    DocumentRepository-->>DocumentService: success
    
    DocumentService->>EventBus: publish_event(DocumentUpdated)
    EventBus-->>DocumentService: success
    
    Note over EventBus: Event is broadcast to all subscribers
    
    EventBus->>WebSocket: broadcast_update(documentUpdated)
    WebSocket->>UserB: real_time_update(documentUpdated)
    
    EventBus->>NotificationService: send_document_notification(documentUpdated, collaborators)
    NotificationService-->>EventBus: success
    
    DocumentService-->>GraphQL_API: updated_document
    GraphQL_API-->>UserA: DocumentDto
    
    UserB->>WebSocket: acknowledge_update()
```

### Flow Description:
1. UserA sends a GraphQL mutation to apply an operation to a document
2. GraphQL API validates the input and calls the DocumentService
3. DocumentService retrieves the current document state
4. The service applies the CRDT operation to update the document state
5. The updated state is saved via the repository
6. The service publishes a DocumentUpdated event to the EventBus
7. EventBus broadcasts the event to all subscribed WebSocket clients for real-time updates
8. UserB receives the real-time update about the document change
9. EventBus triggers notification sending for all collaborators
10. The updated document is returned to UserA
11. UserB acknowledges receiving the update

## 2. Project Board Creation and Task Management Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant ProjectService
    participant ProjectRepository
    participant EventBus
    participant NotificationService

    User->>GraphQL_API: createProjectBoard(title)
    GraphQL_API->>ProjectService: create_board(userId, title)
    
    ProjectService->>ProjectRepository: create_board(board)
    ProjectRepository-->>ProjectService: success
    
    ProjectService->>EventBus: publish_event(ProjectBoardCreated)
    EventBus-->>ProjectService: success
    
    ProjectService-->>GraphQL_API: project_board
    GraphQL_API-->>User: ProjectBoardDto
    
    Note over User: User adds columns and tasks
    
    User->>GraphQL_API: addColumn(boardId, title, position)
    GraphQL_API->>ProjectService: add_column(boardId, title, position)
    ProjectService->>ProjectRepository: add_column(column)
    ProjectRepository-->>ProjectService: success
    
    ProjectService->>EventBus: publish_event(ColumnAdded)
    EventBus-->>ProjectService: success
    
    User->>GraphQL_API: createTask(columnId, title, description, position)
    GraphQL_API->>ProjectService: create_task(columnId, title, description, position)
    ProjectService->>ProjectRepository: create_task(task)
    ProjectRepository-->>ProjectService: success
    
    ProjectService->>EventBus: publish_event(TaskCreated)
    EventBus-->>ProjectService: success
    
    EventBus->>NotificationService: send_project_notification(task, assignedUsers)
    NotificationService-->>EventBus: success
```

### Flow Description:
1. User sends a GraphQL mutation to create a new project board
2. GraphQL API validates the input and calls the ProjectService
3. ProjectService creates the board and saves it via the repository
4. The service publishes a ProjectBoardCreated event to the EventBus
5. The project board is returned to the user
6. User adds columns to the board, each triggering a ColumnAdded event
7. User creates tasks within columns, each triggering a TaskCreated event
8. EventBus triggers notification sending for assigned users

## 3. Whiteboard Synchronization Process

```mermaid
sequenceDiagram
    participant UserA
    participant UserB
    participant GraphQL_API
    participant WhiteboardService
    participant WhiteboardRepository
    participant EventBus
    participant WebSocket

    UserA->>GraphQL_API: createWhiteboard(title)
    GraphQL_API->>WhiteboardService: create_whiteboard(userId, title)
    WhiteboardService->>WhiteboardRepository: create_whiteboard(whiteboard)
    WhiteboardRepository-->>WhiteboardService: success
    
    WhiteboardService->>EventBus: publish_event(WhiteboardCreated)
    EventBus-->>WhiteboardService: success
    
    WhiteboardService-->>GraphQL_API: whiteboard
    GraphQL_API-->>UserA: WhiteboardDto
    
    Note over UserA: UserA draws on whiteboard
    
    UserA->>GraphQL_API: addWhiteboardElement(whiteboardId, element)
    GraphQL_API->>WhiteboardService: add_element(whiteboardId, element)
    WhiteboardService->>WhiteboardRepository: add_element(element)
    WhiteboardRepository-->>WhiteboardService: success
    
    WhiteboardService->>EventBus: publish_event(WhiteboardElementAdded)
    EventBus-->>WhiteboardService: success
    
    EventBus->>WebSocket: broadcast_update(elementAdded)
    WebSocket->>UserB: real_time_update(elementAdded)
    
    Note over UserB: UserB modifies the element
    
    UserB->>GraphQL_API: updateWhiteboardElement(whiteboardId, elementId, newData)
    GraphQL_API->>WhiteboardService: update_element(whiteboardId, elementId, newData)
    WhiteboardService->>WhiteboardRepository: update_element(elementId, newData)
    WhiteboardRepository-->>WhiteboardService: success
    
    WhiteboardService->>EventBus: publish_event(WhiteboardElementUpdated)
    EventBus-->>WhiteboardService: success
    
    EventBus->>WebSocket: broadcast_update(elementUpdated)
    WebSocket->>UserA: real_time_update(elementUpdated)
```

### Flow Description:
1. UserA creates a new whiteboard
2. WhiteboardService saves the whiteboard and publishes a creation event
3. UserA adds elements to the whiteboard, each triggering an ElementAdded event
4. EventBus broadcasts the events to all subscribed WebSocket clients
5. UserB receives real-time updates about new elements
6. UserB modifies an existing element, triggering an ElementUpdated event
7. UserA receives real-time updates about the modified element

## 4. Meeting Room Initiation with WebRTC

```mermaid
sequenceDiagram
    participant Host
    participant Participant
    participant GraphQL_API
    participant MeetingService
    participant MeetingRepository
    participant EventBus
    participant WebRTCService
    participant STUN/TURN

    Host->>GraphQL_API: createMeeting(title)
    GraphQL_API->>MeetingService: create_meeting(userId, title)
    MeetingService->>MeetingRepository: create_meeting(meeting)
    MeetingRepository-->>MeetingService: success
    
    MeetingService->>EventBus: publish_event(MeetingCreated)
    EventBus-->>MeetingService: success
    
    MeetingService-->>GraphQL_API: meeting
    GraphQL_API-->>Host: MeetingDto
    
    Participant->>GraphQL_API: joinMeeting(meetingId)
    GraphQL_API->>MeetingService: join_meeting(meetingId, participantId)
    MeetingService->>MeetingRepository: add_participant(meetingId, participantId)
    MeetingRepository-->>MeetingService: success
    
    MeetingService->>EventBus: publish_event(ParticipantJoined)
    EventBus-->>MeetingService: success
    
    Host->>WebRTCService: generate_offer(meetingId)
    WebRTCService->>STUN/TURN: get_ice_candidates()
    STUN/TURN-->>WebRTCService: ice_candidates
    
    WebRTCService->>EventBus: publish_event(WebRTCOfferGenerated)
    EventBus-->>WebRTCService: success
    
    EventBus->>WebSocket: broadcast_webrtc_offer(offer)
    WebSocket->>Participant: webrtc_offer(offer)
    
    Participant->>WebRTCService: generate_answer(meetingId, offer)
    WebRTCService->>STUN/TURN: get_ice_candidates()
    STUN/TURN-->>WebRTCService: ice_candidates
    
    WebRTCService->>EventBus: publish_event(WebRTCAnswerGenerated)
    EventBus-->>WebRTCService: success
    
    EventBus->>WebSocket: broadcast_webrtc_answer(answer)
    WebSocket->>Host: webrtc_answer(answer)
    
    Note over Host, Participant: WebRTC connection established
```

### Flow Description:
1. Host creates a new meeting room
2. MeetingService saves the meeting and publishes a creation event
3. Participant joins the meeting, triggering a ParticipantJoined event
4. Host generates a WebRTC offer, contacting STUN/TURN servers for ICE candidates
5. The offer is broadcast to all participants via WebSocket
6. Participant generates a WebRTC answer, also contacting STUN/TURN servers
7. The answer is broadcast back to the host via WebSocket
8. WebRTC connection is established between host and participant

## 5. File Versioning Workflow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant FileService
    participant FileRepository
    participant EventBus
    participant NotificationService
    participant ObjectStorage

    User->>GraphQL_API: uploadFile(fileData, filename)
    GraphQL_API->>FileService: upload_file(userId, fileData, filename)
    
    FileService->>ObjectStorage: store_file(fileData, filename)
    ObjectStorage-->>FileService: file_id
    
    FileService->>FileRepository: create_file(file_metadata)
    FileRepository-->>FileService: success
    
    FileService->>EventBus: publish_event(FileUploaded)
    EventBus-->>FileService: success
    
    FileService-->>GraphQL_API: file_info
    GraphQL_API-->>User: FileDto
    
    Note over User: User modifies the file
    
    User->>GraphQL_API: createFileVersion(fileId, newFileData)
    GraphQL_API->>FileService: create_version(fileId, newFileData, userId)
    
    FileService->>FileRepository: get_latest_version(fileId)
    FileRepository-->>FileService: latest_version
    
    FileService->>ObjectStorage: store_file_version(fileId, newVersion, newFileData)
    ObjectStorage-->>FileService: version_id
    
    FileService->>FileRepository: create_file_version(version_metadata)
    FileRepository-->>FileService: success
    
    FileService->>EventBus: publish_event(FileVersionCreated)
    EventBus-->>FileService: success
    
    EventBus->>NotificationService: send_file_version_notification(fileVersion, collaborators)
    NotificationService-->>EventBus: success
    
    FileService-->>GraphQL_API: version_info
    GraphQL_API-->>User: FileVersionDto
    
    Note over User: User requests file history
    
    User->>GraphQL_API: getFileVersions(fileId)
    GraphQL_API->>FileService: get_file_versions(fileId)
    FileService->>FileRepository: get_versions(fileId)
    FileRepository-->>FileService: versions
    
    FileService-->>GraphQL_API: versions
    GraphQL_API-->>User: [FileVersionDto]
```

### Flow Description:
1. User uploads a new file
2. FileService stores the file in object storage and saves metadata
3. A FileUploaded event is published
4. User creates a new version of the file
5. FileService retrieves the latest version and stores the new version
6. A FileVersionCreated event is published
7. EventBus triggers notification sending for collaborators
8. User requests the file version history
9. FileService retrieves and returns all versions of the file

## Error Handling Flow

```mermaid
sequenceDiagram
    participant User
    participant GraphQL_API
    participant DocumentService
    participant DocumentRepository

    User->>GraphQL_API: applyDocumentOperation(documentId, operation)
    GraphQL_API->>DocumentService: apply_operation(userId, documentId, operation)
    
    DocumentService->>DocumentRepository: get_document_state(documentId)
    DocumentRepository-->>DocumentService: DocumentNotFound
    
    DocumentService-->>GraphQL_API: NotFoundError("Document not found")
    GraphQL_API-->>User: Error("Document not found")
```

### Flow Description:
1. User attempts to apply an operation to a non-existent document
2. DocumentService attempts to retrieve the document state
3. Repository returns that the document was not found
4. Service returns a NotFoundError
5. The error is propagated back to the user via GraphQL

## Performance Considerations

- **CRDT Optimization**: Document operations use efficient CRDT algorithms to minimize state size
- **WebSocket Management**: WebSocket connections are efficiently managed for real-time collaboration
- **Event Processing**: EventBus uses weak references and efficient broadcasting to prevent memory leaks
- **File Storage**: Large files are stored in object storage with metadata in PostgreSQL
- **WebRTC Optimization**: ICE candidate gathering is optimized for fast connection establishment
- **Version History**: File versions are stored with delta compression to reduce storage requirements

## Security Considerations

- **Authorization**: All operations verify user permissions before modifications
- **Document Encryption**: Document contents are encrypted end-to-end
- **WebRTC Security**: WebRTC connections use DTLS and SRTP for security
- **File Sanitization**: Uploaded files are scanned for malware and malicious content
- **Rate Limiting**: API endpoints implement rate limiting to prevent abuse
- **Permission Models**: Workspace permissions are enforced at the service level