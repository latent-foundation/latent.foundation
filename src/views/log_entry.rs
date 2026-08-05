//! A single log entry (`/log/:slug`).
//!
//! The body arrives from [`crate::content`] as HTML that was rendered at build time, so
//! this view only lays it out — there is no markdown parsing in the browser. Injecting it
//! with `inner_html` is safe here because every byte originates in a markdown file in this
//! repo (see [`crate::content::Entry::html`]).
//!
//! Layout matches [`crate::views::ProjectDetail`]: a 720px column with a back-link and a
//! hairline divider, so the site has one reading layout rather than two.

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_params_map};

use crate::{
    content::{self, Entry},
    title::use_title,
};

/// Entry page: title, meta, optional table of contents, prose, and prev/next links.
///
/// An unrecognised slug renders the same muted "not found" treatment the project detail
/// page uses, so a stale or mistyped link degrades gracefully.
#[allow(non_snake_case)]
#[component]
pub fn LogEntry() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default().to_string());

    use_title(move || {
        content::by_slug(&slug())
            .map(|e| e.title.to_string())
            .unwrap_or_else(|| "Log".to_string())
    });

    view! {
        <div class="container">
            {move || match content::by_slug(&slug()) {
                None => {
                    view! {
                        <p style="padding-top: 64px; color: var(--color-text-muted)">
                            "entry not found."
                        </p>
                    }
                        .into_any()
                }
                Some(entry) => view! { <EntryBody entry=entry /> }.into_any(),
            }}
        </div>
    }
}

/// The rendered entry. Split out so the lookup above stays a thin match.
#[allow(non_snake_case)]
#[component]
fn EntryBody(entry: Entry) -> impl IntoView {
    let (newer, older) = content::neighbours(entry.slug);
    // Two headings make a list, not a map. Below that the TOC is noise.
    let show_toc = entry.headings.len() >= 3;

    view! {
        // `has-toc` gates the two-column layout: without it the grid would still reserve
        // an empty 200px gutter and the article would look indented for no reason.
        <div class="log-entry" class:has-toc=show_toc>
            <A href="/log" attr:class="back-link">
                "← log"
            </A>

            <h1 class="log-entry-title">{entry.title}</h1>
            <div class="log-entry-meta">
                <span>{entry.date}</span>
                <span class="log-entry-meta-sep">"·"</span>
                <span>{format!("{} min read", entry.reading_minutes)}</span>
            </div>

            {(!entry.tags.is_empty())
                .then(|| {
                    view! {
                        <div class="project-tags log-entry-tags">
                            {entry
                                .tags
                                .iter()
                                .map(|t| view! { <span class="tag">{*t}</span> })
                                .collect_view()}
                        </div>
                    }
                })}

            <div class="log-entry-divider"></div>

            {show_toc
                .then(|| {
                    view! {
                        <nav class="log-toc" aria-label="table of contents">
                            <p class="log-toc-label">"contents"</p>
                            {entry
                                .headings
                                .iter()
                                .map(|h| {
                                    view! {
                                        <a
                                            href=format!("#{}", h.id)
                                            class="log-toc-link"
                                            class:level-3=h.level == 3
                                        >
                                            {h.text}
                                        </a>
                                    }
                                })
                                .collect_view()}
                        </nav>
                    }
                })}

            // Build-time HTML from this repo's own content — see the module docs.
            <div class="markdown-body" inner_html=entry.html></div>

            {(newer.is_some() || older.is_some())
                .then(|| {
                    view! {
                        <div class="log-entry-nav">
                            {newer
                                .map(|e| {
                                    view! {
                                        <A href=e.href() attr:class="log-entry-nav-link">
                                            <span class="log-entry-nav-label">"newer"</span>
                                            {e.title}
                                        </A>
                                    }
                                })}
                            {older
                                .map(|e| {
                                    view! {
                                        <A href=e.href() attr:class="log-entry-nav-link older">
                                            <span class="log-entry-nav-label">"older"</span>
                                            {e.title}
                                        </A>
                                    }
                                })}
                        </div>
                    }
                })}
        </div>
    }
}
