//! Shared UI components used across multiple pages.
//!
//! All components here are part of the persistent layout shell ([`SiteHeader`],
//! [`SiteFooter`]) or reusable content blocks ([`ProjectRow`]) that appear on
//! both the home preview and the full projects archive.

mod footer;
mod header;
mod project_row;

pub use footer::SiteFooter;
pub use header::SiteHeader;
pub use project_row::ProjectRow;
