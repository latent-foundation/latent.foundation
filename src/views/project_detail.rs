//! Individual project page (`/projects/:id`).
//!
//! Looks up the project by `id` param in [`crate::data::PROJECTS`]. All optional
//! sections (rationale, long description, links) are omitted when `None` / empty
//! so the page is always clean regardless of how much content is filled in.
//! To populate a section, set the corresponding field in [`crate::data::PROJECTS`].

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::data::PROJECTS;

/// Project detail page: full description, rationale, links, and metadata.
///
/// Renders a "not found" message if the `:id` param doesn't match any project —
/// this handles direct URL access to a deleted or mistyped slug gracefully.
#[allow(non_snake_case)]
#[component]
pub fn ProjectDetail() -> impl IntoView {
    let params = use_params_map();

    view! {
        <div class="container">
            {move || {
                let id = params.with(|p| p.get("id").unwrap_or_default().to_string());
                match PROJECTS.iter().find(|p| p.id == id.as_str()).copied() {
                    None => {
                        view! {
                            <p style="padding-top: 64px; color: var(--color-text-muted)">
                                "project not found."
                            </p>
                        }
                            .into_any()
                    }
                    Some(project) => {
                        view! {
                            <div class="project-detail">
                                <a href="/projects" class="back-link">
                                    "← projects"
                                </a>

                                <div class="project-detail-hero">
                                    <div class="project-detail-splash">{project.splash_text}</div>
                                    <div class="project-detail-hero-body">
                                        <h1 class="project-detail-name">{project.id}</h1>
                                        <div class="project-detail-meta">
                                            <span>{project.status.label()}</span>
                                            <span class="project-detail-meta-sep">"·"</span>
                                            <span>{project.year.to_string()}</span>
                                        </div>
                                    </div>
                                </div>

                                <div class="project-tags project-detail-tags">
                                    {project
                                        .tags
                                        .iter()
                                        .map(|t| view! { <span class="tag">{*t}</span> })
                                        .collect_view()}
                                </div>

                                <div class="project-detail-divider"></div>

                                <p class="project-detail-lead">{project.description}</p>

                                {project
                                    .rationale
                                    .map(|r| {
                                        view! {
                                            <div class="project-detail-section">
                                                <h2 class="project-detail-section-label">"rationale"</h2>
                                                <p class="project-detail-body">{r}</p>
                                            </div>
                                        }
                                    })}

                                {project
                                    .long_description
                                    .map(|d| {
                                        view! {
                                            <div class="project-detail-section">
                                                <h2 class="project-detail-section-label">"about"</h2>
                                                <p class="project-detail-body">{d}</p>
                                            </div>
                                        }
                                    })}

                                {(!project.links.is_empty())
                                    .then(|| {
                                        view! {
                                            <div class="project-detail-section">
                                                <h2 class="project-detail-section-label">"links"</h2>
                                                <div class="project-detail-links">
                                                    {project
                                                        .links
                                                        .iter()
                                                        .map(|(label, url)| {
                                                            view! {
                                                                <a
                                                                    href=*url
                                                                    target="_blank"
                                                                    rel="noopener noreferrer"
                                                                    class="project-detail-link"
                                                                >
                                                                    {*label}
                                                                </a>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </div>
                                            </div>
                                        }
                                    })}
                            </div>
                        }
                            .into_any()
                    }
                }
            }}
        </div>
    }
}
