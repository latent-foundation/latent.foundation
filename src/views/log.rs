//! Log index (`/log`): every entry, newest first, with tag filtering.
//!
//! Deliberately the same shape as [`crate::views::Projects`] — a hairline-separated list
//! plus a pill filter bar — so the site has one archive idiom rather than two. Filter
//! state is a local `RwSignal<Option<&'static str>>` where `None` means "show all", and a
//! `Memo` derives the filtered list so the rows only re-render when the filter changes.

use leptos::prelude::*;
use leptos_router::components::A;

use crate::{
    content::{self, Entry, LOG},
    title::use_title,
};

/// Full log archive with a tag filter bar.
#[allow(non_snake_case)]
#[component]
pub fn Log() -> impl IntoView {
    use_title(|| "Log".to_string());

    let filter = RwSignal::new(None::<&'static str>);
    let tags = content::all_tags();

    let shown = Memo::new(move |_| {
        LOG.iter()
            .filter(|e| filter.get().is_none_or(|t| e.tags.contains(&t)))
            .copied()
            .collect::<Vec<_>>()
    });

    view! {
        <div class="container">
            <div class="log-page">
                <h1>"Log"</h1>
                <p class="log-intro">
                    "Notes on what I'm building and why. Written when something is worth writing down,
                    not on a schedule."
                </p>

                {(!tags.is_empty())
                    .then(|| {
                        view! {
                            <div class="filter-bar">
                                <button
                                    class="filter-btn"
                                    class:active=move || filter.get().is_none()
                                    on:click=move |_| filter.set(None)
                                >
                                    "ALL"
                                </button>
                                {tags
                                    .into_iter()
                                    .map(|tag| {
                                        view! {
                                            <button
                                                class="filter-btn"
                                                class:active=move || filter.get() == Some(tag)
                                                on:click=move |_| filter.set(Some(tag))
                                            >
                                                {tag.to_uppercase()}
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                    })}

                <div class="log-list">
                    {move || {
                        let entries = shown.get();
                        if entries.is_empty() {
                            view! { <p class="log-empty">"nothing here yet."</p> }.into_any()
                        } else {
                            entries
                                .into_iter()
                                .map(|entry| view! { <LogRow entry=entry /> })
                                .collect_view()
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// One row of the log index: date, title, summary, tags, reading time.
#[allow(non_snake_case)]
#[component]
fn LogRow(entry: Entry) -> impl IntoView {
    view! {
        <A href=entry.href() attr:class="log-row">
            <span class="log-date">{entry.date}</span>
            <div class="log-row-content">
                <h2 class="log-row-title">{entry.title}</h2>
                <p class="log-row-summary">{entry.summary}</p>
                <div class="project-tags">
                    {entry
                        .tags
                        .iter()
                        .map(|t| view! { <span class="tag">{*t}</span> })
                        .collect_view()}
                </div>
            </div>
            <span class="log-reading-time">{format!("{} min", entry.reading_minutes)}</span>
        </A>
    }
}
