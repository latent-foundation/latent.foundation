//! Projects archive (`/projects`): full catalogue with status filtering.
//!
//! Filter state is a `RwSignal<Option<Status>>` local to this component —
//! `None` means "show all". A `Memo` derives the filtered list reactively so
//! Leptos only re-renders the project list when the filter changes, not on
//! every unrelated signal update elsewhere in the tree.

use leptos::prelude::*;

use crate::{
    components::ProjectRow,
    data::{PROJECTS, Status},
};

/// Full project archive with ALL / ACTIVE / RESEARCH / ARCHIVED filter bar.
///
/// The `Memo` wrapper on `shown` ensures the filtered `Vec` is only recomputed
/// when `filter` changes — important if `PROJECTS` grows large enough that
/// re-filtering on every render would be noticeable.
#[allow(non_snake_case)]
#[component]
pub fn Projects() -> impl IntoView {
    let filter = RwSignal::new(None::<Status>);

    let shown = Memo::new(move |_| {
        PROJECTS
            .iter()
            .filter(|p| filter.get().is_none_or(|f| p.status == f))
            .copied()
            .collect::<Vec<_>>()
    });

    view! {
        <div class="container">
            <div class="projects-page">
                <h1>"Projects"</h1>
                <p class="projects-intro">
                    "An archive of software, research tooling and experiments."
                    "Everything done by me that's worth sharing."
                </p>
                <div class="filter-bar">
                    <button
                        class="filter-btn"
                        class:active=move || filter.get().is_none()
                        on:click=move |_| filter.set(None)
                    >
                        "ALL"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == Some(Status::Active)
                        on:click=move |_| filter.set(Some(Status::Active))
                    >
                        "ACTIVE"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == Some(Status::Research)
                        on:click=move |_| filter.set(Some(Status::Research))
                    >
                        "RESEARCH"
                    </button>
                    <button
                        class="filter-btn"
                        class:active=move || filter.get() == Some(Status::Archived)
                        on:click=move |_| filter.set(Some(Status::Archived))
                    >
                        "ARCHIVED"
                    </button>
                </div>
                <div class="project-list">
                    {move || {
                        shown
                            .get()
                            .into_iter()
                            .map(|p| view! { <ProjectRow project=p /> })
                            .collect_view()
                    }}
                </div>
            </div>
        </div>
    }
}
