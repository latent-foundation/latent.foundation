//! The ido roadmap: what is built, what is next, and what is only a direction.
//!
//! Content is static data rather than markup, following the same convention as
//! [`crate::data`] — the copy is the thing being maintained, not the `view!` around it.
//!
//! **Every claim here must be traceable to ido's own repo** (`README.md`, `CLAUDE.md`, or
//! the latent well's `ido-mcp-design` page). The one exception is mobile, which is a stated
//! direction with no spec behind it, and is marked [`Horizon::Exploring`] precisely so it
//! does not read as a promise. Nothing shipped is described in the future tense and nothing
//! unshipped is described in the present.
//!
//! Version numbers (1.0.0, 1.1.0) are deliberately absent even now that both have shipped:
//! a version on a public roadmap reads as a date. The horizons carry the ordering instead.

use leptos::prelude::*;

use latent_ui::Icon;

/// How far along an arc is. Ordering here is the order shown.
///
/// `Planned` (specced and sequenced, not yet started) is deliberately not a variant here:
/// every current arc is either real or a bare direction, and adding a state nothing
/// constructs just to keep a slot warm is the premature abstraction this codebase avoids.
/// Re-add it the day an arc actually needs it — the exhaustive matches below will force it
/// into every place that needs to know.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Horizon {
    /// Built and in daily use.
    Shipped,
    /// Actively being worked on, with a spec behind it.
    Next,
    /// A direction. No commitment, no schedule.
    Exploring,
}

impl Horizon {
    /// Mono label shown beside the marker.
    fn label(self) -> &'static str {
        match self {
            Horizon::Shipped => "SHIPPED",
            Horizon::Next => "NEXT",
            Horizon::Exploring => "EXPLORING",
        }
    }

    /// Lucide glyph from `latent-ui`'s shared table, drawn inside the spine node.
    ///
    /// Read top to bottom they describe a gradient of certainty: done, in motion, unknown.
    fn icon(self) -> &'static str {
        match self {
            Horizon::Shipped => "square-check",
            Horizon::Next => "chevron-right",
            Horizon::Exploring => "circle-help",
        }
    }

    /// CSS modifier — drives the muting of unshipped rows.
    fn class(self) -> &'static str {
        match self {
            Horizon::Shipped => "shipped",
            Horizon::Next => "next",
            Horizon::Exploring => "exploring",
        }
    }

    /// Whether this arc describes work that does not exist yet.
    ///
    /// Drives the spine's solid→dashed transition — the one place the page states where
    /// reality ends. That decision lives here, beside the variants, rather than as a list
    /// of class names in a CSS selector: the match is written out in full precisely so
    /// that adding a `Horizon` fails to compile until someone decides which side of the
    /// line it falls on. A `_ =>` arm would defeat the entire point.
    fn projected(self) -> bool {
        match self {
            Horizon::Shipped | Horizon::Next => false,
            Horizon::Exploring => true,
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
        body: "Notes, wiki and tasks, in one window, over a folder of plain markdown.",
        points: &[
            "three editor modes: source, live block-preview, reading",
            "kanban board with goals, recurrence, calendar and table views",
            "tabs, split panes, and a Ctrl+K palette across every section",
            "soft-delete with undo, pasted images, inline and display math",
        ],
    },
    Arc {
        horizon: Horizon::Shipped,
        title: "an MCP server over the store",
        body: "The store lives in a Tauri-free crate, and a separate binary exposes it over \
               MCP with seven read-only tools. Any MCP client can read and search a well \
               without ido running.",
        points: &[
            "well_info, search, get_entry, list_entries, backlinks, list_tasks, list_goals",
            "ships inside the app as a Tauri sidecar — install once, register once",
        ],
    },
    Arc {
        horizon: Horizon::Next,
        title: "semantic search",
        body: "Ask what you decided about something and find the note that never used \
               that word. Local embeddings, brute-force cosine over the vectors, fused \
               with the lexical scan that already exists.",
        points: &[
            "runs on-device: no embedding API, no cloud index, nothing leaves the machine",
            "the same index powers in-app search, which is the honest test of whether it works",
        ],
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
               than a commitment, there is no design for it yet.",
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
    "notes are not [[link]] targets. That namespace belongs to the wiki",
];

/// The roadmap, drawn as a timeline.
///
/// Each arc is a node on a continuous spine. The connector below a node is solid where
/// the work is real — shipped, or actively being built — and dashed from there on, so the
/// boundary between what exists and what is projected is visible without reading a word
/// of it. That transition is the reason this is a timeline rather than a list.
#[allow(non_snake_case)]
#[component]
pub fn Roadmap() -> impl IntoView {
    view! {
        <div class="ido-roadmap">
            {ARCS
                .iter()
                .map(|arc| {
                    view! {
                        <div class=format!(
                            "ido-arc {}{}",
                            arc.horizon.class(),
                            if arc.horizon.projected() { " projected" } else { "" },
                        )>
                            <div class="ido-arc-spine">
                                <span class="ido-arc-node">
                                    <Icon name=arc.horizon.icon() size=13 />
                                </span>
                            </div>
                            <div class="ido-arc-body">
                                <span class="ido-arc-horizon">{arc.horizon.label()}</span>
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
