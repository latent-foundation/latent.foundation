//! Schematic miniatures of ido's interface, built from divs and design tokens.
//!
//! These are **illustrations, not screenshots** — re-implementations that will drift from
//! the real app as it moves. Three rules keep that honest:
//!
//! 1. They stay schematic. No invented feature, no data implying a capability ido lacks.
//! 2. They are `aria-hidden`, and the prose beside them carries every piece of
//!    information. A reader who never sees them loses nothing.
//! 3. They theme for free, because they are made of the same tokens as the rest of the
//!    site — which is the reason they exist rather than a pile of PNGs that would need a
//!    light and a dark capture each.

use leptos::prelude::*;

/// The rail and the three sections it switches between.
#[allow(non_snake_case)]
#[component]
pub fn SectionsMini() -> impl IntoView {
    let sections: &[(&str, &str, bool)] = &[
        ("notes", "foldered markdown", false),
        ("wiki", "linked pages", true),
        ("tasks", "board and goals", false),
    ];

    view! {
        <div class="ido-mini ido-mini-sections" aria-hidden="true">
            <div class="ido-mini-rail">
                <span class="ido-mini-rail-mark">"井"</span>
                <span class="ido-mini-rail-dot"></span>
                <span class="ido-mini-rail-dot active"></span>
                <span class="ido-mini-rail-dot"></span>
            </div>
            <div class="ido-mini-sections-body">
                {sections
                    .iter()
                    .map(|(name, note, active)| {
                        view! {
                            <div class="ido-mini-section" class:active=*active>
                                <span class="ido-mini-section-name">{*name}</span>
                                <span class="ido-mini-section-note">{*note}</span>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// The three editor modes, with the middle one selected.
#[allow(non_snake_case)]
#[component]
pub fn EditorMini() -> impl IntoView {
    view! {
        <div class="ido-mini ido-mini-editor" aria-hidden="true">
            <div class="ido-mini-modes">
                <span class="ido-mini-mode">"source"</span>
                <span class="ido-mini-mode active">"live"</span>
                <span class="ido-mini-mode">"reading"</span>
            </div>
            <div class="ido-mini-doc">
                <span class="ido-mini-h">"Auth token storage"</span>
                <span class="ido-mini-line long"></span>
                <span class="ido-mini-line"></span>
                // The block under the cursor shows its raw markdown; the rest is rendered.
                <div class="ido-mini-block-editing">
                    <span class="ido-mini-raw">"the **keychain**, not a file"</span>
                </div>
                <span class="ido-mini-line long"></span>
                <span class="ido-mini-line short"></span>
            </div>
        </div>
    }
}

/// A kanban board: three columns, cards, an insertion line mid-drag.
#[allow(non_snake_case)]
#[component]
pub fn BoardMini() -> impl IntoView {
    view! {
        <div class="ido-mini ido-mini-board" aria-hidden="true">
            <div class="ido-mini-col">
                <span class="ido-mini-col-head">"todo"</span>
                <div class="ido-mini-card">
                    <span class="ido-mini-card-title"></span>
                    <span class="ido-mini-card-meta">"3/7"</span>
                </div>
                <div class="ido-mini-card">
                    <span class="ido-mini-card-title short"></span>
                    <span class="ido-mini-card-meta due">"7 Jul"</span>
                </div>
            </div>
            <div class="ido-mini-col">
                <span class="ido-mini-col-head">"doing"</span>
                // The accent rule is the drop indicator, shown between cards mid-drag.
                <span class="ido-mini-insertion"></span>
                <div class="ido-mini-card">
                    <span class="ido-mini-card-title short"></span>
                </div>
            </div>
            <div class="ido-mini-col">
                <span class="ido-mini-col-head">"done"</span>
                <div class="ido-mini-card done">
                    <span class="ido-mini-card-title"></span>
                </div>
                <div class="ido-mini-card done">
                    <span class="ido-mini-card-title short"></span>
                </div>
            </div>
        </div>
    }
}

/// A tab strip across two panes, one tab in preview (italic) and a split divider.
#[allow(non_snake_case)]
#[component]
pub fn TabsMini() -> impl IntoView {
    view! {
        <div class="ido-mini ido-mini-tabs" aria-hidden="true">
            <div class="ido-mini-pane">
                <div class="ido-mini-tabstrip">
                    <span class="ido-mini-tab active">"tokens"</span>
                    <span class="ido-mini-tab preview">"standup"</span>
                </div>
                <div class="ido-mini-doc compact">
                    <span class="ido-mini-line long"></span>
                    <span class="ido-mini-line"></span>
                    <span class="ido-mini-line short"></span>
                </div>
            </div>
            <div class="ido-mini-split"></div>
            <div class="ido-mini-pane">
                <div class="ido-mini-tabstrip">
                    <span class="ido-mini-tab active">"ship-auth"</span>
                </div>
                <div class="ido-mini-doc compact">
                    <span class="ido-mini-line"></span>
                    <span class="ido-mini-line long"></span>
                </div>
            </div>
        </div>
    }
}
