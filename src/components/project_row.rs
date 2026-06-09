//! Single project entry used in both the home preview and the projects archive.
//!
//! The `splash_text` field (kanji / Greek script) doubles as the visual mark for
//! each project. When it is empty, the first character of `id` is used as a
//! fallback so the layout never collapses to an empty cell.
//!
//! Each row is an `<A>` router link to `/projects/{id}`. Using the router component
//! keeps navigation client-side (no full reload). The link reset styles
//! (`color: inherit`, `text-decoration: none`) live in `.project-row` in site.css.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::data::Project;

/// Renders one row of the project list: mark, name, status badge, description, tags, year.
///
/// Accepts a [`Project`] by value — the type is `Copy` so no clone is needed at
/// the call site even when iterating over a slice of projects.
#[allow(non_snake_case)]
#[component]
pub fn ProjectRow(project: Project) -> impl IntoView {
    let mark = if project.splash_text.is_empty() {
        project.id[..1].to_string()
    } else {
        project.splash_text.to_string()
    };

    let href = format!("/projects/{}", project.id);

    view! {
        <A href=href attr:class="project-row">
            <div class="project-splash-text">{mark}</div>
            <div class="project-content">
                <div class="project-header">
                    <span class="project-name">{project.id}</span>
                    <span class="project-status">{project.status.label()}</span>
                </div>
                <p class="project-line">{project.description}</p>
                <div class="project-tags">
                    {project
                        .tags
                        .iter()
                        .map(|t| view! { <span class="tag">{*t}</span> })
                        .collect_view()}
                </div>
            </div>
            <span class="project-year">{project.year.to_string()}</span>
        </A>
    }
}
