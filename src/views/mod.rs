//! Page-level view components, one per route.
//!
//! Each module corresponds to a route declared in [`crate::app`]:
//! - [`Home`]     → `/`
//! - [`Projects`] → `/projects`
//! - [`About`]    → `/about`

mod about;
mod home;
mod projects;

pub use about::About;
pub use home::Home;
pub use projects::Projects;
