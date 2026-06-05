//! Home page (`/`): hero section and two-project preview.
//!
//! Intentionally minimal — the hero copy establishes tone, and the project
//! preview gives visitors an immediate sense of the work without overwhelming
//! the landing. The single "view all →" CTA drives toward `/projects`.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{components::ProjectRow, data::PROJECTS};

/// Landing page: hero headline and the first two projects from [`PROJECTS`].
///
/// The slice `PROJECTS[..2]` is a compile-time constant — it panics if fewer
/// than two projects are defined, which is intentional (the layout requires two).
#[allow(non_snake_case)]
#[component]
pub fn Home() -> impl IntoView {
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
        </div>
    }
}
