//! Page-level view components, one per route.
//!
//! Each module corresponds to a route declared in [`crate::app`]:
//! - [`Home`]          → `/`
//! - [`Projects`]      → `/projects`
//! - [`ProjectDetail`] → `/projects/:id`
//! - [`About`]         → `/about`

mod about;
mod home;
mod project_detail;
mod projects;

pub use about::About;
pub use home::Home;
pub use project_detail::ProjectDetail;
pub use projects::Projects;
