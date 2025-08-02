use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::{Course as DatabaseCourse, Module as DatabaseModule, Lesson as DatabaseLesson};
use crate::error::AppError;
use crate::utils::{validate_not_empty, parse_uuid};

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");
tonic::include_proto!("cpc.learning_platform_server");

pub struct CourseService {
    repository: DatabaseRepository,
}

impl CourseService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
    
    async fn convert_db_course_to_proto(&self, db_course: DatabaseCourse) -> Result<Course, AppError> {
        // Fetch modules for this course
        let db_modules = self.repository.get_modules_by_course_id(db_course.id).await?;
        
        // Convert modules to proto modules
        let mut proto_modules = Vec::new();
        for db_module in db_modules {
            // Fetch lessons for this module
            let db_lessons = self.repository.get_lessons_by_module_id(db_module.id).await?;
            
            // Convert lessons to proto lessons
            let proto_lessons: Vec<Lesson> = db_lessons.into_iter().map(|lesson| Lesson {
                id: lesson.id.to_string(),
                title: lesson.title,
                content: lesson.content,
            }).collect();
            
            let proto_module = Module {
                id: db_module.id.to_string(),
                title: db_module.title,
                lessons: proto_lessons,
            };
            
            proto_modules.push(proto_module);
        }
        
        Ok(Course {
            id: db_course.id.to_string(),
            title: db_course.title,
            description: db_course.description,
            creator_id: db_course.creator_id.to_string(),
            modules: proto_modules,
        })
    }
}

#[tonic::async_trait]
impl course_service_server::CourseService for CourseService {
    async fn list_courses(
        &self,
        _request: Request<ListCoursesRequest>,
    ) -> Result<Response<ListCoursesResponse>, Status> {
        // Fetch courses from the database
        let db_courses = self.repository.list_courses().await
            .map_err(AppError::from)?;
        
        // Convert database courses to protobuf courses
        let mut proto_courses = Vec::new();
        for db_course in db_courses {
            let proto_course = self.convert_db_course_to_proto(db_course).await
                .map_err(AppError::from)?;
            proto_courses.push(proto_course);
        }
        
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
            // Validate inputs
            validate_not_empty(&course.title, "Course title")?;
            validate_not_empty(&course.description, "Course description")?;
            
            // Validate creator ID
            let creator_id = parse_uuid(&course.creator_id)?;
            
            // Create a database course from the protobuf course
            let course_id = Uuid::new_v4();
            let db_course = DatabaseCourse {
                id: course_id,
                title: course.title,
                description: course.description,
                creator_id,
                created_at: Utc::now(),
            };
            
            // Save course to database
            let saved_course = self.repository.create_course(&db_course).await
                .map_err(AppError::from)?;
            
            // Create modules if provided
            for (index, module) in course.modules.iter().enumerate() {
                // Validate module inputs
                validate_not_empty(&module.title, "Module title")?;
                
                let module_id = Uuid::new_v4();
                let db_module = DatabaseModule {
                    id: module_id,
                    course_id: saved_course.id,
                    title: module.title.clone(),
                    order_index: index as i32,
                    created_at: Utc::now(),
                };
                
                // Save module to database
                let saved_module = self.repository.create_module(&db_module).await
                    .map_err(AppError::from)?;
                
                // Create lessons if provided
                for (lesson_index, lesson) in module.lessons.iter().enumerate() {
                    // Validate lesson inputs
                    validate_not_empty(&lesson.title, "Lesson title")?;
                    validate_not_empty(&lesson.content, "Lesson content")?;
                    
                    let lesson_id = Uuid::new_v4();
                    let db_lesson = DatabaseLesson {
                        id: lesson_id,
                        module_id: saved_module.id,
                        title: lesson.title.clone(),
                        content: lesson.content.clone(),
                        media_url: String::new(), // Not in proto, default to empty
                        order_index: lesson_index as i32,
                        created_at: Utc::now(),
                    };
                    
                    // Save lesson to database
                    self.repository.create_lesson(&db_lesson).await
                        .map_err(AppError::from)?;
                }
            }
            
            let response = CreateCourseResponse {
                course_id: saved_course.id.to_string(),
            };
            
            Ok(Response::new(response))
        } else {
            Err(AppError::Validation("Course data is required".to_string()).into())
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
            Err(AppError::Validation("Course data is required".to_string()).into())
        }
    }
}