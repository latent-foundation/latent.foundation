//! Static project catalogue.
//!
//! All site content lives here as `'static` data compiled into the WASM binary —
//! no network fetch, no CMS. To add or update a project, edit [`PROJECTS`] and
//! recompile; the home preview and full archive update automatically.
//!
//! Detail-page fields (`long_description`, `rationale`, `links`) are `None`/empty
//! by default. Fill them in as each project matures — the detail page renders only
//! the sections that have content.

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
///
/// The `description` field is the short one-liner used in list views.
/// `long_description` and `rationale` are for the detail page and may be `None`
/// until a project is mature enough to document. `links` follows the same rule.
#[derive(Clone, Copy, PartialEq)]
pub struct Project {
    /// URL-safe slug, display name, and route segment (e.g. `"ido"`, `"logos"`).
    pub id: &'static str,
    /// Visual mark shown in the project row — typically a kanji or Greek word.
    /// Falls back to the first character of `id` when empty.
    pub splash_text: &'static str,
    /// Current lifecycle state.
    pub status: Status,
    /// One-sentence description used in list rows and as the lead on the detail page.
    pub description: &'static str,
    /// Technology and domain tags rendered as pill labels.
    pub tags: &'static [&'static str],
    /// Year the project was started or last significantly worked on.
    pub year: u16,
    /// Extended description shown only on the detail page. `None` until finalised.
    pub long_description: Option<&'static str>,
    /// The reasoning behind building this project. `None` until finalised.
    pub rationale: Option<&'static str>,
    /// External links as (label, URL) pairs. Empty until finalised.
    pub links: &'static [(&'static str, &'static str)],
}

/// Complete list of projects, newest first.
///
/// [`crate::views::Home`] renders the first two entries as a preview;
/// [`crate::views::Projects`] renders all with optional status filtering;
/// [`crate::views::ProjectDetail`] renders individual entries by `id`.
pub static PROJECTS: &[Project] = &[
    Project {
        id: "ido",
        splash_text: "井戸",
        status: Status::Active,
        description: "A quiet local-first knowledge system built around depth, memory, project management and long-term organization.",
        tags: &["KNOWLEDGE", "LOCAL-FIRST", "RUST"],
        year: 2026,
        long_description: Some(
            "Ido is an early-stage notes, wiki, task tracking, and goal-setting system. \
        The project is being designed as a local-first workspace where writing, planning, \
        and long-term personal knowledge can live close together without turning into a noisy \
        productivity dashboard. Its backbone is intended to be plain Markdown, with structure \
        added gradually through links, metadata, projects, and lightweight organizational tools.",
        ),
        rationale: Some(
            "Ido comes from 井戸, meaning a water well. The metaphor is intentional: a quiet place for \
        storing, returning to, and drawing from accumulated thought. The project exists as an \
        attempt to build a slower and more personal alternative to large cloud-first workspaces, \
        with an emphasis on ownership, durability, and calm organization.",
        ),
        links: &[],
    },
    Project {
        id: "logos",
        splash_text: "λόγος",
        status: Status::Research,
        description: "A research system for reasoning under uncertainty. An algorithmic trading, backtesting and analytics platform.",
        tags: &["MARKETS", "BACKTESTING", "RUST"],
        year: 2026,
        long_description: Some(
            "Logos is an early-stage research platform for market data, backtesting, analytics, \
        and algorithmic trading experiments. The project is currently in the design and research \
        phase, with the goal of building a disciplined environment for testing market hypotheses, \
        validating strategies, and reasoning about financial time series without relying on ad-hoc \
        notebooks or fragile scripts.",
        ),
        rationale: Some(
            "The name Logos refers to reason, order, and the search for intelligible structure beneath \
        appearances. In the context of markets, the project is not meant to be a speculative black box \
        or a gambling bot. It is intended as a system for careful experimentation: collecting data, \
        testing assumptions, measuring risk, and deciding only when a strategy has earned that decision.",
        ),
        links: &[],
    },
    Project {
        id: "proteus",
        splash_text: "Πρωτεύς",
        status: Status::Archived,
        description: "Change-point detection via a Markov-switching model for financial time series. Made as my thesis project. No longer maintained but kept here for posterity.",
        tags: &["MARKOV", "CHANGE-POINT", "THESIS", "RUST"],
        year: 2026,
        long_description: Some(
            "Proteus is a research-oriented Rust project for online change-point detection in financial time series. \
            It was built around a Gaussian Markov-switching model: the model is trained offline, then used in a causal \
            online filtering loop to infer latent market regimes and raise alarms when the regime structure changes. \
            The project explores detectors based on hard regime switches, posterior movement, and predictive surprise, \
            with experiments on both synthetic and real market data.",
        ),
        rationale: Some(
            "Proteus was created as the implementation layer of my master's thesis. \
            The goal was to test whether regime inference from Markov-switching models can be turned into a practical \
            online detector, rather than only a retrospective modelling tool. \
            It is archived because the thesis work is complete, but the repository remains as a record of the modelling, \
            experimentation, and synthetic-to-real evaluation process.",
        ),
        links: &[("GitHub", "https://github.com/Spiryd/proteus")],
    },
];
