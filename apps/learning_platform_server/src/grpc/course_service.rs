use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::Course as DatabaseCourse;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");

pub struct CourseService {
    repository: DatabaseRepository,
}

impl CourseService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl course_service_server::CourseService for CourseService {
    async fn list_courses(
        &self,
        _request: Request<ListCoursesRequest>,
    ) -> Result<Response<ListCoursesResponse>, Status> {
        // Fetch courses from the database
        let courses = self.repository.list_courses().await
            .map_err(|e| Status::internal(format!("Failed to fetch courses: {}", e)))?;
        
        // Convert database courses to protobuf courses
        let proto_courses: Vec<Course> = courses.into_iter().map(|course| Course {
            id: course.id.to_string(),
            title: course.title,
            description: course.description,
            modules: vec![], // In a full implementation, we would fetch modules
        }).collect();
        
        let response = ListCoursesResponse {
            courses: proto_courses,
        };
        
        Ok(Response::new(response))
    }

    async fn create_course(
        &self,
        request: Request<CreateCourseRequest>,
    ) -> Result<Response<CreateCourseResponse>, Status> {
        let req = request.into_inner();
        
        if let Some(course) = req.course {
            // Create a database course from the protobuf course
            let course_id = Uuid::new_v4();
            let db_course = DatabaseCourse {
                id: course_id,
                title: course.title,
                description: course.description,
                creator_id: Uuid::parse_str(&course.creator_id)
                    .map_err(|_| Status::invalid_argument("Invalid creator ID"))?,
                created_at: Utc::now(),
            };
            
            // Save to database
            let saved_course = self.repository.create_course(&db_course).await
                .map_err(|e| Status::internal(format!("Failed to create course: {}", e)))?;
            
            let response = CreateCourseResponse {
                course_id: saved_course.id.to_string(),
            };
            
            Ok(Response::new(response))
        } else {
            Err(Status::invalid_argument("Course data is required"))
        }
    }

    async fn update_course(
        &self,
        request: Request<UpdateCourseRequest>,
    ) -> Result<Response<UpdateCourseResponse>, Status> {
        let req = request.into_inner();
        
        if let Some(course) = req.course {
            // In a real implementation, we would update the course in the database
            // For now, we'll just return the same course data
            let response = UpdateCourseResponse {
                course: Some(course),
            };
            
            Ok(Response::new(response))
        } else {
            Err(Status::invalid_argument("Course data is required"))
        }
    }
}