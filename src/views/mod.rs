//! Page-level view components, one per route.
//!
//! Each module corresponds to a route declared in [`crate::app`]:
//! - [`Home`]          → `/`
//! - [`Projects`]      → `/projects`
//! - [`ProjectDetail`] → `/projects/:id`
//! - [`Log`]           → `/log`
//! - [`LogEntry`]      → `/log/:slug`
//! - [`IdoPage`]       → `/ido`
//! - [`About`]         → `/about`

mod about;
mod home;
mod ido;
mod log;
mod log_entry;
mod project_detail;
mod projects;

pub use about::About;
pub use home::Home;
pub use ido::IdoPage;
pub use log::Log;
pub use log_entry::LogEntry;
pub use project_detail::ProjectDetail;
pub use projects::Projects;
