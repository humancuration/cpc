//! Pages for the Messenger web application

pub mod home;
pub mod conversation;
pub mod thread;
pub mod not_found;

pub use home::Home;
pub use conversation::Conversation;
pub use thread::Thread;
pub use not_found::NotFound;