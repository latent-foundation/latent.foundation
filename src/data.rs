//! Static project catalogue.
//!
//! All site content lives here as `'static` data compiled into the WASM binary —
//! no network fetch, no CMS. To add or update a project, edit [`PROJECTS`] and
//! recompile; the home preview and full archive update automatically.

use std::fmt;

/// Project lifecycle state used for display labels and the filter bar on `/projects`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Actively built and maintained.
    Active,
    /// Ongoing research or exploratory work, not yet feature-complete.
    Research,
    /// No longer maintained; kept for historical reference.
    Archived,
}

impl Status {
    /// Short uppercase label rendered in status badges and filter buttons.
    pub fn label(self) -> &'static str {
        match self {
            Status::Active => "ACTIVE",
            Status::Research => "RESEARCH",
            Status::Archived => "ARCHIVED",
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

/// A single entry in the project catalogue.
///
/// All fields are `'static` references or primitives so `Project` is `Copy` —
/// safe to pass across reactive closures without cloning or lifetime annotation.
#[derive(Clone, Copy, PartialEq)]
pub struct Project {
    /// URL-safe slug and display name (e.g. `"ido"`, `"logos"`).
    pub id: &'static str,
    /// Visual mark shown in the project row — typically a kanji or Greek word.
    /// Falls back to the first character of `id` when empty.
    pub splash_text: &'static str,
    /// Current lifecycle state.
    pub status: Status,
    /// One-sentence description of the project's purpose.
    pub description: &'static str,
    /// Technology and domain tags rendered as pill labels.
    pub tags: &'static [&'static str],
    /// Year the project was started or last significantly worked on.
    pub year: u16,
}

/// Complete list of projects, newest first.
///
/// [`crate::views::Home`] renders the first two entries as a preview;
/// [`crate::views::Projects`] renders all entries with optional status filtering.
pub static PROJECTS: &[Project] = &[
    Project {
        id: "ido",
        splash_text: "井戸",
        status: Status::Active,
        description: "A quiet local-first knowledge system built around depth, memory, and long-term organization.",
        tags: &["KNOWLEDGE", "LOCAL-FIRST", "RUST"],
        year: 2026,
    },
    Project {
        id: "logos",
        splash_text: "λόγος",
        status: Status::Research,
        description: "A research system for reasoning under uncertainty.",
        tags: &["MARKETS", "BACKTESTING", "RUST"],
        year: 2026,
    },
    Project {
        id: "proteus",
        splash_text: "",
        status: Status::Archived,
        description: "Change-point detection via a Markov-switching model for commodities.",
        tags: &["MARKOV", "CHANGE-POINT", "THESIS", "RUST"],
        year: 2025,
    },
];
