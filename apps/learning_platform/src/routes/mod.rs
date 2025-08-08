use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::{
    course_catalog::CourseCatalogPage,
    course_detail::CourseDetailPage,
    enrollment::EnrollmentPage,
    credential::CredentialPage,
    tipping::TippingPage,
    skill_dashboard::SkillDashboardPage,
};

#[derive(Switch, Clone, Debug, PartialEq)]
pub enum AppRoute {
    #[to = "/courses"]
    CourseCatalog,
    #[to = "/course/{id}"]
    CourseDetail(String),
    #[to = "/enrollments"]
    Enrollments,
    #[to = "/credentials"]
    Credentials,
    #[to = "/tip"]
    Tip,
    #[to = "/skills"]
    Skills,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::CourseCatalog => html! { <CourseCatalogPage /> },
        AppRoute::CourseDetail(id) => html! { <CourseDetailPage id={id} /> },
        AppRoute::Enrollments => html! { <EnrollmentPage /> },
        AppRoute::Credentials => html! { <CredentialPage /> },
        AppRoute::Tip => html! { <TippingPage /> },
        AppRoute::Skills => html! { <SkillDashboardPage /> },
    }
}