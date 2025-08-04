//! Integration-like tests for Collaborative Workspace GraphQL
//!
//! Uses simple mock services implementing the service traits to validate
//! resolver wiring, ID parsing, context auth, and DTO mapping.

use async_graphql::{EmptySubscription, Request, Schema, ID};
use async_trait::async_trait;
use chrono::{TimeZone, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::graphql::collaborative_workspace::{
    CollaborativeWorkspaceMutations, CollaborativeWorkspaceQueries, DocumentDto, FileVersionDto,
    MeetingRoomDto, ProjectBoardDto, ProjectColumnDto, ProjectTaskDto,
};
use shared_packages::collaborative_workspace::domain::models::{
    CollaborativeDocument, FileVersion, MeetingRoom, ProjectBoard, ProjectColumn, ProjectTask,
};
use shared_packages::collaborative_workspace::domain::service::{
    DocumentService, FileService, MeetingService, ProjectService, ServiceError,
};

#[derive(Clone, Default)]
struct MockDocumentService;
#[async_trait]
impl DocumentService for MockDocumentService {
    async fn create_document(
        &self,
        title: String,
        created_by: Uuid,
    ) -> Result<CollaborativeDocument, ServiceError> {
        Ok(CollaborativeDocument {
            id: Uuid::new_v4(),
            title,
            created_by,
            created_at: Utc.timestamp_opt(0, 0).unwrap(),
            updated_at: Utc.timestamp_opt(0, 0).unwrap(),
        })
    }

    async fn get_document(
        &self,
        document_id: Uuid,
    ) -> Result<Option<CollaborativeDocument>, ServiceError> {
        Ok(Some(CollaborativeDocument {
            id: document_id,
            title: "Doc".into(),
            created_by: Uuid::nil(),
            created_at: Utc.timestamp_opt(0, 0).unwrap(),
            updated_at: Utc.timestamp_opt(0, 0).unwrap(),
        }))
    }

    async fn apply_operation(
        &self,
        document_id: Uuid,
        _operation: String,
        _user_id: Uuid,
    ) -> Result<CollaborativeDocument, ServiceError> {
        Ok(CollaborativeDocument {
            id: document_id,
            title: "Updated".into(),
            created_by: Uuid::nil(),
            created_at: Utc.timestamp_opt(0, 0).unwrap(),
            updated_at: Utc::now(),
        })
    }
}

#[derive(Clone, Default)]
struct MockProjectService;
#[async_trait]
impl ProjectService for MockProjectService {
    async fn create_board(
        &self,
        title: String,
        owner_id: Uuid,
    ) -> Result<ProjectBoard, ServiceError> {
        Ok(ProjectBoard {
            id: Uuid::new_v4(),
            title,
            owner_id,
            created_at: Utc.timestamp_opt(0, 0).unwrap(),
        })
    }

    async fn get_board(&self, board_id: Uuid) -> Result<Option<ProjectBoard>, ServiceError> {
        Ok(Some(ProjectBoard {
            id: board_id,
            title: "Board".into(),
            owner_id: Uuid::nil(),
            created_at: Utc.timestamp_opt(0, 0).unwrap(),
        }))
    }

    async fn add_column(
        &self,
        board_id: Uuid,
        title: String,
        position: i32,
    ) -> Result<ProjectColumn, ServiceError> {
        Ok(ProjectColumn {
            id: Uuid::new_v4(),
            board_id,
            title,
            position,
        })
    }

    async fn add_task(
        &self,
        column_id: Uuid,
        title: String,
        description: Option<String>,
        position: i32,
    ) -> Result<ProjectTask, ServiceError> {
        Ok(ProjectTask {
            id: Uuid::new_v4(),
            column_id,
            title,
            description,
            position,
            created_at: Utc::now(),
            updated_at: None,
        })
    }

    async fn move_task(
        &self,
        task_id: Uuid,
        new_column_id: Uuid,
        position: i32,
    ) -> Result<ProjectTask, ServiceError> {
        Ok(ProjectTask {
            id: task_id,
            column_id: new_column_id,
            title: "Task".into(),
            description: None,
            position,
            created_at: Utc::now(),
            updated_at: None,
        })
    }
}

#[derive(Clone, Default)]
struct MockFileService;
#[async_trait]
impl FileService for MockFileService {
    async fn create_version(
        &self,
        file_id: Uuid,
        created_by: Uuid,
    ) -> Result<FileVersion, ServiceError> {
        Ok(FileVersion {
            id: Uuid::new_v4(),
            file_id,
            version: 1,
            created_by,
            created_at: Utc::now(),
        })
    }

    async fn get_version(
        &self,
        file_id: Uuid,
        _version: i32,
    ) -> Result<Option<FileVersion>, ServiceError> {
        Ok(Some(FileVersion {
            id: Uuid::new_v4(),
            file_id,
            version: 1,
            created_by: Uuid::nil(),
            created_at: Utc::now(),
        }))
    }
}

#[derive(Clone, Default)]
struct MockMeetingService;
#[async_trait]
impl MeetingService for MockMeetingService {
    async fn create_meeting(
        &self,
        title: String,
        owner_id: Uuid,
    ) -> Result<MeetingRoom, ServiceError> {
        Ok(MeetingRoom {
            id: Uuid::new_v4(),
            title,
            owner_id,
            created_at: Utc::now(),
            ended_at: None,
        })
    }

    async fn end_meeting(&self, room_id: Uuid) -> Result<MeetingRoom, ServiceError> {
        Ok(MeetingRoom {
            id: room_id,
            title: "Ended".into(),
            owner_id: Uuid::nil(),
            created_at: Utc::now(),
            ended_at: Some(Utc::now()),
        })
    }
}

type TestSchema = Schema<CollaborativeWorkspaceQueries, CollaborativeWorkspaceMutations, EmptySubscription>;

fn build_test_schema(user_id: Uuid) -> TestSchema {
    Schema::build(
        CollaborativeWorkspaceQueries::default(),
        CollaborativeWorkspaceMutations::default(),
        EmptySubscription,
    )
    .data(user_id)
    .data::<Box<dyn DocumentService>>(Box::new(MockDocumentService::default()))
    .data::<Box<dyn ProjectService>>(Box::new(MockProjectService::default()))
    .data::<Box<dyn FileService>>(Box::new(MockFileService::default()))
    .data::<Box<dyn MeetingService>>(Box::new(MockMeetingService::default()))
    .finish()
}

#[tokio::test]
async fn create_and_get_document_works() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    let req = Request::new(
        r#"
        mutation CreateDoc($title:String!) {
          createDocument(title:$title) { id title createdBy createdAt updatedAt }
        }
        "#,
    ).variables(async_graphql::Variables::from_value(async_graphql::Value::from_json(serde_json::json!({"title":"Test Doc"})).unwrap()));

    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);

    // Extract ID and then query getDocument
    let created = resp.data.into_json().unwrap();
    let doc_id = created["createDocument"]["id"].as_str().unwrap().to_string();

    let query = Request::new(
        r#"
        query GetDoc($id:ID!) {
          getDocument(id:$id) { id title }
        }
        "#,
    ).variables(async_graphql::Variables::from_json(serde_json::json!({"id": doc_id.clone()})).unwrap());

    let resp2 = schema.execute(query).await;
    assert!(resp2.errors.is_empty(), "errors: {:?}", resp2.errors);
    let got = resp2.data.into_json().unwrap();
    assert_eq!(got["getDocument"]["id"].as_str().unwrap(), doc_id);
}

#[tokio::test]
async fn project_board_and_column_flow() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    let create_board = Request::new(
        r#"
        mutation { createProjectBoard(title:"Board X") { id title ownerId } }
        "#,
    );
    let resp = schema.execute(create_board).await;
    assert!(resp.errors.is_empty());
    let board_id = resp.data.into_json().unwrap()["createProjectBoard"]["id"]
        .as_str()
        .unwrap()
        .to_string();

    let add_col = Request::new(
        r#"
        mutation AddCol($bid:ID!){
          addProjectColumn(boardId:$bid, title:"Todo", position:0) { id boardId title position }
        }
        "#,
    ).variables(async_graphql::Variables::from_json(serde_json::json!({"bid": board_id.clone()})).unwrap());
    let resp2 = schema.execute(add_col).await;
    assert!(resp2.errors.is_empty(), "errors: {:?}", resp2.errors);
}

#[tokio::test]
async fn move_task_flow() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    // We can't create tasks via GQL in this minimal layer, but we can still exercise moveTask resolver parsing and wiring by using UUIDs.
    let task_id = Uuid::new_v4().to_string();
    let col_id = Uuid::new_v4().to_string();

    let req = Request::new(
        r#"
        mutation Move($tid:ID!, $cid:ID!){
          moveTask(taskId:$tid, newColumnId:$cid, position:1) { id columnId position }
        }
        "#,
    ).variables(async_graphql::Variables::from_json(serde_json::json!({"tid":task_id, "cid": col_id})).unwrap());

    let resp = schema.execute(req).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);
}

#[tokio::test]
async fn file_version_and_meeting_creation() {
    let user = Uuid::new_v4();
    let schema = build_test_schema(user);

    let f_id = Uuid::new_v4().to_string();
    let file_req = Request::new(
        r#"
        mutation CreateVer($fid:ID!){
          createFileVersion(fileId:$fid, content:"ignored") { id fileId version createdBy }
        }
        "#,
    ).variables(async_graphql::Variables::from_json(serde_json::json!({"fid": f_id})).unwrap());
    let resp = schema.execute(file_req).await;
    assert!(resp.errors.is_empty(), "errors: {:?}", resp.errors);

    let meet_req = Request::new(
        r#"
        mutation { createMeeting(title:"Daily") { id title ownerId } }
        "#,
    );
    let resp2 = schema.execute(meet_req).await;
    assert!(resp2.errors.is_empty(), "errors: {:?}", resp2.errors);
}