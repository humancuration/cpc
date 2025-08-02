#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_course_creation() {
        let title = "Rust Programming".to_string();
        let description = "Learn Rust programming language".to_string();
        let creator_id = Uuid::new_v4();
        
        let course = Course::new(title.clone(), description.clone(), creator_id);
        
        assert_eq!(course.title, title);
        assert_eq!(course.description, description);
        assert_eq!(course.creator_id, creator_id);
        assert_eq!(course.modules.len(), 0);
    }

    #[test]
    fn test_add_module() {
        let mut course = Course::new(
            "Rust Programming".to_string(),
            "Learn Rust programming language".to_string(),
            Uuid::new_v4()
        );
        
        let module = Module {
            id: Uuid::new_v4(),
            title: "Basics".to_string(),
            lessons: vec![],
        };
        
        course.add_module(module.clone());
        
        assert_eq!(course.modules.len(), 1);
        assert_eq!(course.modules[0].title, "Basics");
    }

    #[test]
    fn test_total_lessons() {
        let mut course = Course::new(
            "Rust Programming".to_string(),
            "Learn Rust programming language".to_string(),
            Uuid::new_v4()
        );
        
        let lesson1 = Lesson {
            id: Uuid::new_v4(),
            title: "Variables".to_string(),
            content: "Learn about variables".to_string(),
            media_url: "".to_string(),
        };
        
        let lesson2 = Lesson {
            id: Uuid::new_v4(),
            title: "Functions".to_string(),
            content: "Learn about functions".to_string(),
            media_url: "".to_string(),
        };
        
        let module1 = Module {
            id: Uuid::new_v4(),
            title: "Basics".to_string(),
            lessons: vec![lesson1],
        };
        
        let module2 = Module {
            id: Uuid::new_v4(),
            title: "Intermediate".to_string(),
            lessons: vec![lesson2],
        };
        
        course.add_module(module1);
        course.add_module(module2);
        
        assert_eq!(course.total_lessons(), 2);
    }
}