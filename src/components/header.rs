//! Persistent site header: wordmark, navigation links, and theme toggle.
//!
//! `SiteHeader` consumes the `RwSignal<bool>` theme context injected by
//! [`crate::app::App`]. Navigation uses `leptos_router`'s [`A`] component,
//! which automatically sets `aria-current="page"` on the active link — the CSS
//! uses that attribute for the active-link underline rather than a JS class toggle.
//!
//! The home link uses `exact=true` so it only activates on `/` and not on every
//! route, since all routes share the `/` prefix.

use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

/// Persistent header rendered above every page.
///
/// Reads theme context to swap between the dark and light wordmark SVGs.
/// `use_location()` is called to subscribe to route changes so the header
/// re-renders when navigation occurs and `aria-current` stays accurate.
#[component]
pub fn SiteHeader() -> impl IntoView {
    let is_dark = use_context::<RwSignal<bool>>().expect("theme context");

    // Subscribes to location changes so aria-current updates on navigation.
    let _ = use_location();

    let mark_src = move || {
        if is_dark.get() {
            "/assets/latent-mark-dark.svg"
        } else {
            "/assets/latent-mark-light.svg"
        }
    };

    view! {
        <header class="site-header">
            <div class="header-inner">
                // exact=true: the / route must match exactly, not prefix-match every page
                <A href="/" exact=true attr:class="logo-link">
                    <img src=mark_src alt="" class="logo-mark" />
                    <span class="header-wordmark">"latent."</span>
                </A>
                <nav class="site-nav">
                    <A href="/" exact=true attr:class="nav-link">
                        "index"
                    </A>
                    <A href="/projects" attr:class="nav-link">
                        "projects"
                    </A>
                    <A href="/about" attr:class="nav-link">
                        "about"
                    </A>
                    <button
                        class="theme-toggle"
                        on:click=move |_| is_dark.update(|d| *d = !*d)
                        title="toggle theme"
                    >
                        {move || if is_dark.get() { "☾" } else { "☀" }}
                    </button>
                </nav>
            </div>
        </header>
    }
}
