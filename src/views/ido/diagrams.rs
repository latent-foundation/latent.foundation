//! Structural diagrams for the ido page.
//!
//! Two different techniques, chosen per subject rather than for consistency:
//!
//! - [`WellTree`] is a **monospace HTML** tree. A directory listing is text; drawing it as
//!   SVG would fix its line breaks, hide it from selection and search, and gain nothing.
//! - [`LinkGraph`] is **inline SVG**, because it shows a flow — arrows converging from
//!   three different sections onto one page — which prose describes poorly and text
//!   cannot draw at all.
//!
//! Both take their colour from `currentColor` so the surrounding `.ido-diagram` rule
//! themes them, and neither ships an asset.

use leptos::prelude::*;

/// A well on disk: the folder layout, annotated.
///
/// The point being made is that everything is plain markdown in ordinary directories, and
/// the only ido-specific folder is rebuildable — so nothing here is a lock-in.
#[allow(non_snake_case)]
#[component]
pub fn WellTree() -> impl IntoView {
    // (indent-prefix, name, annotation) — annotation is muted and right-aligned.
    let rows: &[(&str, &str, &str)] = &[
        ("", "my-well/", "any folder you pick"),
        ("├── ", "notes/", "foldered markdown"),
        ("├── ", "wiki/", "[[linked]] pages, unique slugs"),
        ("├── ", "tasks/", "one .md per task"),
        ("│   └── ", "goals/", "milestones with target dates"),
        ("├── ", "assets/", "pasted images"),
        ("└── ", ".ido/", "config + cache — rebuildable"),
    ];

    view! {
        <div
            class="ido-tree"
            role="img"
            aria-label="A well is a folder containing notes, wiki, tasks, assets and a rebuildable .ido directory"
        >
            {rows
                .iter()
                .map(|(prefix, name, note)| {
                    view! {
                        <div class="ido-tree-row">
                            <span class="ido-tree-path">
                                <span class="ido-tree-prefix">{*prefix}</span>
                                <span class="ido-tree-name">{*name}</span>
                            </span>
                            <span class="ido-tree-note">{*note}</span>
                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

/// Inbound links converging on one wiki page from all three sections.
///
/// Drawn rather than described because the claim — that a note, another wiki page and a
/// *task body* all count as backlinks, and that a rename rewrites every one of them — is
/// a shape, and shapes read faster than sentences.
#[allow(non_snake_case)]
#[component]
pub fn LinkGraph() -> impl IntoView {
    // (top edge of the source box, label)
    let sources: &[(f32, &str)] = &[
        (18.0, "notes/standup.md"),
        (92.0, "wiki/tokens.md"),
        (166.0, "tasks/ship-auth.md"),
    ];

    view! {
        // The accessible name is carried by `aria-label` rather than an SVG `<title>`
        // child: Leptos resolves `title` to the HTML element, which inside an `<svg>`
        // parent would not reliably name the graphic.
        <svg
            class="ido-svg"
            viewBox="0 0 520 224"
            role="img"
            preserveAspectRatio="xMidYMid meet"
            aria-label="Notes, wiki pages and task bodies all link to one wiki page; renaming it rewrites every inbound link"
        >

            {sources
                .iter()
                .map(|(y, label)| {
                    let cy = y + 20.0;
                    view! {
                        <g>
                            <rect
                                x="1"
                                y=y.to_string()
                                width="196"
                                height="40"
                                rx="2"
                                class="ido-svg-box"
                            />
                            <text x="16" y=(cy + 4.0).to_string() class="ido-svg-label">
                                {*label}
                            </text>
                            // Curve out to the target's left edge, then a small arrowhead.
                            <path
                                d=format!("M197 {cy} C 260 {cy}, 268 112, 326 112")
                                class="ido-svg-line"
                            />
                        </g>
                    }
                })
                .collect_view()}

            <path d="M318 107 L328 112 L318 117" class="ido-svg-arrow" />

            <rect x="330" y="88" width="188" height="48" rx="2" class="ido-svg-box target" />
            <text x="346" y="110" class="ido-svg-label target">
                "wiki/auth.md"
            </text>
            <text x="346" y="126" class="ido-svg-sub">
                "3 linked from"
            </text>
        </svg>
    }
}
