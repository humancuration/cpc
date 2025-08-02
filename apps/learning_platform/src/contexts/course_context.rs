use yew::prelude::*;
use std::collections::HashMap;
use crate::types::{Course, Enrollment};

#[derive(Debug, Clone, PartialEq)]
pub struct CourseState {
    pub courses: HashMap<String, Course>,
    pub enrollments: HashMap<String, Enrollment>,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CourseAction {
    SetCourses(Vec<Course>),
    SetEnrollments(Vec<Enrollment>),
    AddCourse(Course),
    UpdateEnrollment(Enrollment),
    SetLoading(bool),
    SetError(Option<String>),
}

impl Reducible for CourseState {
    type Action = CourseAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            CourseAction::SetCourses(courses) => {
                let mut course_map = HashMap::new();
                for course in courses {
                    course_map.insert(course.id.clone(), course);
                }
                Rc::new(CourseState {
                    courses: course_map,
                    enrollments: self.enrollments.clone(),
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            CourseAction::SetEnrollments(enrollments) => {
                let mut enrollment_map = HashMap::new();
                for enrollment in enrollments {
                    enrollment_map.insert(enrollment.id.clone(), enrollment);
                }
                Rc::new(CourseState {
                    courses: self.courses.clone(),
                    enrollments: enrollment_map,
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            CourseAction::AddCourse(course) => {
                let mut courses = self.courses.clone();
                courses.insert(course.id.clone(), course);
                Rc::new(CourseState {
                    courses,
                    enrollments: self.enrollments.clone(),
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            CourseAction::UpdateEnrollment(enrollment) => {
                let mut enrollments = self.enrollments.clone();
                enrollments.insert(enrollment.id.clone(), enrollment);
                Rc::new(CourseState {
                    courses: self.courses.clone(),
                    enrollments,
                    loading: self.loading,
                    error: self.error.clone(),
                })
            }
            CourseAction::SetLoading(loading) => {
                Rc::new(CourseState {
                    courses: self.courses.clone(),
                    enrollments: self.enrollments.clone(),
                    loading,
                    error: self.error.clone(),
                })
            }
            CourseAction::SetError(error) => {
                Rc::new(CourseState {
                    courses: self.courses.clone(),
                    enrollments: self.enrollments.clone(),
                    loading: self.loading,
                    error,
                })
            }
        }
    }
}

#[function_component(CourseContextProvider)]
pub fn course_context_provider(props: &ChildrenProperties) -> Html {
    let course_state = use_reducer(|| CourseState {
        courses: HashMap::new(),
        enrollments: HashMap::new(),
        loading: false,
        error: None,
    });

    html! {
        <ContextProvider<UseReducerHandle<CourseState>> context={course_state}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<CourseState>>>
    }
}

pub fn use_courses() -> UseReducerHandle<CourseState> {
    use_context::<UseReducerHandle<CourseState>>()
        .expect("Course context is missing")
}