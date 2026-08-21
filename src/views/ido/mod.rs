//! The ido product page (`/ido`).
//!
//! `/projects/ido` is an archive entry — two paragraphs and a year. ido is a real desktop
//! app with a specced next phase, which does not fit an archive row, so it gets its own
//! page. The two do not compete: the archive row links straight here (via
//! [`crate::data::Project::page`]), and the detail page links onward.
//!
//! **Content rule:** every claim on this page is traceable to ido's own repo — its
//! `README.md` or `CLAUDE.md`. Nothing shipped is written in the future tense, nothing
//! unshipped in the present. Design notes: the latent well's `website-ido-page` page.
//!
//! This module composes only; the pieces live beside it — [`roadmap`] owns the roadmap
//! data and timeline, [`diagrams`] the structural drawings, [`miniatures`] the schematic
//! UI illustrations.

mod diagrams;
mod miniatures;
mod roadmap;

use leptos::prelude::*;
use leptos_router::components::A;

use crate::title::use_title;
use diagrams::{LinkGraph, WellTree};
use miniatures::{BoardMini, EditorMini, SectionsMini, TabsMini};
use roadmap::{NON_GOALS, Roadmap};

/// ido's product page: what it is, how a well is laid out, what it does, where it's going.
#[allow(non_snake_case)]
#[component]
pub fn IdoPage() -> impl IntoView {
    use_title(|| "ido".to_string());

    view! {
        <div class="container">
            <div class="ido-page">
                <A href="/projects" attr:class="back-link">
                    "← projects"
                </A>

                <IdoHero />
                <IdoWhatItIs />
                <IdoWell />
                <IdoFeatures />
                <IdoRoadmapSection />
                <IdoPrinciples />
                <IdoClosing />
            </div>
        </div>
    }
}

/// Hero: mark, name, status, positioning line, and the source link.
#[allow(non_snake_case)]
#[component]
fn IdoHero() -> impl IntoView {
    view! {
        <header class="ido-hero">
            <div class="ido-hero-mark">"井戸"</div>
            <div class="ido-hero-body">
                <h1 class="ido-hero-name">"ido"</h1>
                <p class="ido-hero-tagline">"a local-first knowledge system"</p>
                <div class="ido-hero-meta">
                    <span>"ACTIVE"</span>
                    <span class="ido-hero-meta-sep">"·"</span>
                    <span>"LOCAL-FIRST"</span>
                    <span class="ido-hero-meta-sep">"·"</span>
                    <span>"TAURI + LEPTOS"</span>
                </div>
            </div>
        </header>

        <p class="ido-lede">
            "A " <strong>"well"</strong>
            " is any folder you pick. Inside it live notes, a linked wiki, and a task board
            with goals. All as plain markdown files on your own machine. Durable, portable,
            yours: no account, no sync dependency, no latency."
        </p>

        <div class="ido-hero-links">
            <a
                href="https://github.com/latent-foundation/ido"
                target="_blank"
                rel="noopener noreferrer"
                class="ido-link-primary"
            >
                "source on github →"
            </a>
            <a
                href="https://github.com/latent-foundation/ido/releases/latest"
                target="_blank"
                rel="noopener noreferrer"
                class="ido-link-secondary"
            >
                "windows installer →"
            </a>
            <span class="ido-hero-status">
                "in active development · macOS/Linux build from source"
            </span>
        </div>
    }
}

/// The three-section model.
#[allow(non_snake_case)]
#[component]
fn IdoWhatItIs() -> impl IntoView {
    view! {
        <section class="ido-section">
            <h2 class="ido-section-label">"three sections, one window"</h2>
            <div class="ido-split">
                <div class="ido-split-prose">
                    <p class="ido-body">
                        "A rail on the left switches between " <strong>"notes"</strong>
                        " (freeform markdown in folders), the " <strong>"wiki"</strong>
                        " (a namespace of pages linked with double brackets), and "
                        <strong>"tasks"</strong>
                        " (a kanban board with goals, a calendar and a table)."
                    </p>
                    <p class="ido-body">
                        "They are separate sections but not separate worlds. Search spans all
                        three. Backlinks span all three. A task's body is edited with the same
                        editor as a note."
                    </p>
                </div>
                <SectionsMini />
            </div>
        </section>
    }
}

/// The well on disk — ido's core promise.
#[allow(non_snake_case)]
#[component]
fn IdoWell() -> impl IntoView {
    view! {
        <section class="ido-section">
            <h2 class="ido-section-label">"a well is just a folder"</h2>
            <div class="ido-split">
                <div class="ido-split-prose">
                    <p class="ido-body">
                        "There is no database and no proprietary format. Point ido at a folder
                        and it scaffolds three directories of markdown files, same files
                        you can open in any editor, grep, or commit to git."
                    </p>
                    <p class="ido-body">
                        "The only ido-specific directory is " <code>".ido/"</code>
                        ", which holds configuration and cache. Delete it and nothing of yours
                        is lost, it gets rebuild. That is the whole guarantee: "
                        <strong>"the app can go away and your notes are still notes."</strong>
                    </p>
                </div>
                <div class="ido-diagram">
                    <WellTree />
                </div>
            </div>
        </section>
    }
}

/// Feature tour: four blocks, each with prose and an illustration.
#[allow(non_snake_case)]
#[component]
fn IdoFeatures() -> impl IntoView {
    view! {
        <section class="ido-section">
            <h2 class="ido-section-label">"what it does"</h2>

            <div class="ido-feature">
                <div class="ido-feature-prose">
                    <h3 class="ido-feature-title">"writing, in three modes"</h3>
                    <p class="ido-body">
                        "Notes live in folders and are edited three ways: "
                        <strong>"source"</strong> " (raw markdown, saved on every keystroke), "
                        <strong>"live"</strong>
                        " (each block renders as HTML until you click into it), and "
                        <strong>"reading"</strong>
                        " (fully rendered). Math renders inline and display."
                    </p>
                </div>
                <EditorMini />
            </div>

            <div class="ido-feature reversed">
                <div class="ido-feature-prose">
                    <h3 class="ido-feature-title">"links that survive being moved"</h3>
                    <p class="ido-body">
                        "Wiki pages are addressed by slug, and a slug is unique across the
                        whole section. Folders are organisation, never identity. Move a
                        page and every link to it still resolves. Rename one and inbound links
                        are rewritten across notes, wiki pages, and task bodies alike."
                    </p>
                    <p class="ido-body">
                        "Link to a page that doesn't exist yet and clicking it creates it."
                    </p>
                </div>
                <div class="ido-diagram">
                    <LinkGraph />
                </div>
            </div>

            <div class="ido-feature">
                <div class="ido-feature-prose">
                    <h3 class="ido-feature-title">"tasks without the dashboard"</h3>
                    <p class="ido-body">
                        "A kanban board where each task is one markdown file. Type in a
                        column's footer to capture without opening anything. Drag cards
                        between columns or to an exact position within one. Due dates flag
                        overdue and due-today; checklists roll up onto the card; finishing a
                        recurring task spawns its next occurrence."
                    </p>
                    <p class="ido-body">
                        <strong>"Goals"</strong>
                        " group tasks and show derived progress. The same tasks render as a
                        calendar or a sortable table. The board is a view, not the storage."
                    </p>
                </div>
                <BoardMini />
            </div>

            <div class="ido-feature reversed">
                <div class="ido-feature-prose">
                    <h3 class="ido-feature-title">"a workspace, not a page viewer"</h3>
                    <p class="ido-body">
                        "Tabs keep several entries open and are restored when you reopen a
                        well. A single click previews, an edit pins. Split the editor into two
                        panes and drag tabs between them. " <code>"Ctrl+K"</code>
                        " searches every section at once, ranked, with matches highlighted."
                    </p>
                    <p class="ido-body">"Deleting anything is a soft delete with an undo toast."</p>
                </div>
                <TabsMini />
            </div>
        </section>
    }
}

/// Roadmap plus the deliberate non-goals.
#[allow(non_snake_case)]
#[component]
fn IdoRoadmapSection() -> impl IntoView {
    view! {
        <section class="ido-section">
            <h2 class="ido-section-label">"where it's going"</h2>
            <p class="ido-body ido-section-lede">
                "The workspace above is built. What follows is ordered by how settled it is:
                specced and started, specced and waiting, or simply a direction."
            </p>

            <Roadmap />

            <div class="ido-nongoals">
                <h3 class="ido-nongoals-label">"deliberately not"</h3>
                <ul class="ido-nongoals-list">
                    {NON_GOALS.iter().map(|n| view! { <li>{*n}</li> }).collect_view()}
                </ul>
            </div>
        </section>
    }
}

/// Principles, on the same fact-grid idiom the about page uses.
#[allow(non_snake_case)]
#[component]
fn IdoPrinciples() -> impl IntoView {
    let principles: &[(&str, &str)] = &[
        (
            "local-first",
            "Your files are on your disk. The network is never in the path between you and \
             your own notes.",
        ),
        (
            "plain files",
            "Markdown in ordinary folders. Readable without ido, and readable in ten years.",
        ),
        (
            "durable",
            "Built to still be in use in five years, which rules out a great many \
             shortcuts taken to ship in five weeks.",
        ),
    ];

    view! {
        <section class="ido-section">
            <h2 class="ido-section-label">"principles"</h2>
            <div class="facts-grid">
                {principles
                    .iter()
                    .map(|(key, value)| {
                        view! {
                            <div class="fact-row">
                                <span class="fact-key">{*key}</span>
                                <span class="fact-value">{*value}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}

/// Closing: honest status and where to read more.
#[allow(non_snake_case)]
#[component]
fn IdoClosing() -> impl IntoView {
    view! {
        <section class="ido-section ido-closing">
            <p class="ido-body">
                "ido is in active development. It is built in the
                open and used daily by the person building it, which is the only reason the
                feature list above is honest about what works."
            </p>
            <div class="ido-closing-links">
                // <A href="/log" attr:class="ido-link-primary">
                // "writing about ido →"
                // </A>
                <A href="/projects/ido" attr:class="ido-link-secondary">
                    "archive entry"
                </A>
            </div>
        </section>
    }
}
