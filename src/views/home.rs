use leptos::prelude::*;
use leptos_router::components::A;

use crate::{components::ProjectRow, data::PROJECTS};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container">
            <section class="hero">
                <p class="hero-kicker">"latent.foundation / a quiet systems laboratory"</p>
                <h1 class="hero-headline">"Systems beneath the surface."</h1>
                <p class="hero-lede">
                    "A personal engineering lab for software projects, research tooling, and long-term
                    systems. Built carefully, reasoned about, iterated over time. Depth over noise."
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
