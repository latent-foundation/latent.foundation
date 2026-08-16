//! Home page (`/`): hero section and two-project preview.
//!
//! Intentionally minimal — the hero copy establishes tone, and the project
//! preview gives visitors an immediate sense of the work without overwhelming
//! the landing. The single "view all →" CTA drives toward `/projects`.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{components::ProjectRow, content::LOG, data::PROJECTS, title::use_title};

/// Landing page: hero headline, the first two projects from [`PROJECTS`], and the two
/// most recent log entries.
///
/// The slice `PROJECTS[..2]` is a compile-time constant — it panics if fewer
/// than two projects are defined, which is intentional (the layout requires two).
/// The log preview uses `.iter().take(2)` instead, because an empty log is a legitimate
/// state and the section simply doesn't render.
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
    use_title(String::new);

    view! {
        <div class="container">
            <section class="hero">
                <p class="hero-kicker">"latent.foundation / a thoughtful systems laboratory"</p>
                <h1 class="hero-headline">"Systems beneath the surface."</h1>
                <p class="hero-lede">
                    "A personal engineering lab for software projects, research tooling, and long-term
                    systems. Built carefully, reasoned about, iterated over time."
                </p>
            </section>
            <section class="projects-section">
                <div class="section-header">
                    <h2 class="section-label">"Projects"</h2>
                    <A href="/projects" attr:class="view-all">
                        "view all →"
                    </A>
                </div>
                <div class="project-list">
                    {PROJECTS[..2]
                        .iter()
                        .map(|p| view! { <ProjectRow project=*p /> })
                        .collect_view()}
                </div>
            </section>
            {(!LOG.is_empty())
                .then(|| {
                    view! {
                        <section class="projects-section">
                            <div class="section-header">
                                <h2 class="section-label">"Log"</h2>
                                <A href="/log" attr:class="view-all">
                                    "view all →"
                                </A>
                            </div>
                            <div class="log-list">
                                {LOG
                                    .iter()
                                    .take(2)
                                    .map(|entry| {
                                        view! {
                                            <A href=entry.href() attr:class="log-row">
                                                <span class="log-date">{entry.date}</span>
                                                <div class="log-row-content">
                                                    <h3 class="log-row-title">{entry.title}</h3>
                                                    <p class="log-row-summary">{entry.summary}</p>
                                                </div>
                                                <span class="log-reading-time">
                                                    {format!("{} min", entry.reading_minutes)}
                                                </span>
                                            </A>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        </section>
                    }
                })}
        </div>
    }
}
