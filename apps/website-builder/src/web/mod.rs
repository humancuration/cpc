//! Web layer for the website builder module

pub mod routes;
pub mod graphql;
pub mod module;
pub mod modular_module;
pub mod types;

// Frontend pages and components (Yew)
// Volunteer Coordination demo pages/components
pub mod pages {
    pub mod volunteer;
}
pub mod components {
    pub mod volunteer {
        pub mod opportunity_form;
        pub mod application_card;
        pub mod contribution_tracker;
    }
}