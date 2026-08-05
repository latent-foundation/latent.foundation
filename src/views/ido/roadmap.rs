//! The ido roadmap: what is built, what is next, and what is only a direction.
//!
//! Content is static data rather than markup, following the same convention as
//! [`crate::data`] — the copy is the thing being maintained, not the `view!` around it.
//!
//! **Every claim here must be traceable to ido's own repo** (`README.md`, `CLAUDE.md`, or
//! `docs/mcp-server.md`). The one exception is mobile, which is a stated direction with no
//! spec behind it, and is marked [`Horizon::Exploring`] precisely so it does not read as a
//! promise. Nothing shipped is described in the future tense and nothing unshipped is
//! described in the present.
//!
//! Version numbers (1.0.0, 1.1.0) are deliberately absent: a version on a public roadmap
//! reads as a date, and ido has not released yet. The horizons carry the ordering instead.

use leptos::prelude::*;

use latent_ui::Icon;

/// How far along an arc is. Ordering here is the order shown.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Horizon {
    /// Built and in daily use.
    Shipped,
    /// Actively being worked on, with a spec behind it.
    Next,
    /// Specced and sequenced, not yet started.
    Planned,
    /// A direction. No commitment, no schedule.
    Exploring,
}

impl Horizon {
    /// Mono label shown beside the marker.
    fn label(self) -> &'static str {
        match self {
            Horizon::Shipped => "SHIPPED",
            Horizon::Next => "NEXT",
            Horizon::Planned => "PLANNED",
            Horizon::Exploring => "EXPLORING",
        }
    }

    /// Lucide glyph from `latent-ui`'s shared table.
    ///
    /// Shipped work gets a filled-feeling check; everything else gets a hollow or
    /// directional mark, so built and unbuilt are distinguishable at a glance and not
    /// only by reading the label.
    fn icon(self) -> &'static str {
        match self {
            Horizon::Shipped => "square-check",
            Horizon::Next => "chevron-right",
            Horizon::Planned => "square",
            Horizon::Exploring => "circle-help",
        }
    }

    /// CSS modifier — drives the muting of unshipped rows.
    fn class(self) -> &'static str {
        match self {
            Horizon::Shipped => "shipped",
            Horizon::Next => "next",
            Horizon::Planned => "planned",
            Horizon::Exploring => "exploring",
        }
    }
}

/// One arc of the roadmap.
pub struct Arc {
    pub horizon: Horizon,
    pub title: &'static str,
    pub body: &'static str,
    /// Concrete sub-points. Empty renders nothing.
    pub points: &'static [&'static str],
}

/// The roadmap, in order.
pub static ARCS: &[Arc] = &[
    Arc {
        horizon: Horizon::Shipped,
        title: "the workspace",
        body: "Notes, wiki and tasks, in one window, over a folder of plain markdown. \
               This is what ido is today — not a prototype.",
        points: &[
            "three editor modes: source, live block-preview, reading",
            "kanban board with goals, recurrence, calendar and table views",
            "tabs, split panes, and a Ctrl+K palette across every section",
            "soft-delete with undo, pasted images, inline and display math",
        ],
    },
    Arc {
        horizon: Horizon::Next,
        title: "an MCP server over the store",
        body: "The store moves into a Tauri-free crate, and a separate binary exposes it \
               over MCP with read-only tools. Any MCP client can then read and search a \
               well — without ido running, and without a second copy of how a task file \
               is parsed.",
        points: &[
            "read-only first; write tools are a later, opt-in phase",
            "one implementation of the store, shared with the app",
        ],
    },
    Arc {
        horizon: Horizon::Planned,
        title: "semantic search",
        body: "Ask what you decided about something and find the note that never used \
               that word. Local embeddings, brute-force cosine over the vectors, fused \
               with the lexical scan that already exists.",
        points: &[
            "runs on-device — no embedding API, no cloud index, nothing leaves the machine",
            "the same index powers in-app search, which is the honest test of whether it works",
        ],
    },
    Arc {
        horizon: Horizon::Planned,
        title: "on-device AI",
        body: "A model that answers over your own notes, running locally. The groundwork \
               the search index lays down — model cache, device setup — is what makes this \
               a smaller step than it sounds.",
        points: &["private by construction: offline, on your machine, over your files"],
    },
    Arc {
        horizon: Horizon::Exploring,
        title: "sync",
        body: "Self-hosted first, cloud optional, end-to-end encrypted. Local-first stays \
               the default: sync is something you turn on, never something you depend on.",
        points: &[],
    },
    Arc {
        horizon: Horizon::Exploring,
        title: "mobile",
        body: "Reading and capture on a phone, against the same well. A direction rather \
               than a commitment — there is no design for it yet.",
        points: &[],
    },
];

/// Things ido deliberately will not do.
///
/// Stating these is not modesty — it is the most information-dense part of the page.
/// A tool defined only by its features is indistinguishable from every other tool.
pub static NON_GOALS: &[&str] = &[
    "no cloud account, and no cloud requirement",
    "no telemetry, ever",
    "not a productivity dashboard — no streaks, no scores",
    "notes are not [[link]] targets; that namespace belongs to the wiki",
];

/// The roadmap timeline.
#[allow(non_snake_case)]
#[component]
pub fn Roadmap() -> impl IntoView {
    view! {
        <div class="ido-roadmap">
            {ARCS
                .iter()
                .map(|arc| {
                    view! {
                        <div class=format!("ido-arc {}", arc.horizon.class())>
                            <div class="ido-arc-marker">
                                <Icon name=arc.horizon.icon() size=14 />
                                <span class="ido-arc-horizon">{arc.horizon.label()}</span>
                            </div>
                            <div class="ido-arc-body">
                                <h3 class="ido-arc-title">{arc.title}</h3>
                                <p class="ido-arc-text">{arc.body}</p>
                                {(!arc.points.is_empty())
                                    .then(|| {
                                        view! {
                                            <ul class="ido-arc-points">
                                                {arc
                                                    .points
                                                    .iter()
                                                    .map(|p| view! { <li>{*p}</li> })
                                                    .collect_view()}
                                            </ul>
                                        }
                                    })}
                            </div>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}
