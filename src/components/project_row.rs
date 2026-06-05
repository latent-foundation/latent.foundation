use leptos::prelude::*;

use crate::data::Project;

#[component]
pub fn ProjectRow(project: Project) -> impl IntoView {
    let mark = if project.splash_text.is_empty() {
        project.id[..1].to_string()
    } else {
        project.splash_text.to_string()
    };

    view! {
        <div class="project-row">
            <div class="project-kanji">{mark}</div>
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
        </div>
    }
}
