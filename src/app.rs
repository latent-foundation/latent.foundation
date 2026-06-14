//! Root application component and client-side router.
//!
//! [`App`] is the top-level Leptos component mounted by `main`. It owns the
//! global theme signal, exposes it via context so any descendant can read or
//! toggle the theme without prop drilling, and wraps the four-route SPA in a
//! persistent layout shell (header + footer that survive page transitions).

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use latent_ui::theme::{initial_theme, setup_theme_effect};

use crate::{
    components::{SiteFooter, SiteHeader},
    views::{About, Home, ProjectDetail, Projects},
};

/// Root component: initialises theme context and declares the client-side routes.
///
/// Route table:
/// - `/`              → [`Home`]
/// - `/projects`      → [`Projects`]
/// - `/projects/:id`  → [`ProjectDetail`]
/// - `/about`         → [`About`]
///
/// The `fallback` renders a minimal "page not found" message for any unmatched
/// path rather than silently showing a blank page.
#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    let is_dark = RwSignal::new(initial_theme());
    provide_context(is_dark);
    setup_theme_effect(is_dark);

    view! {
        <Router>
            <SiteHeader />
            <main class="main-content">
                <Routes fallback=|| {
                    view! {
                        <div class="container">
                            <p style="padding-top: 64px; color: var(--color-text-muted)">
                                "page not found."
                            </p>
                        </div>
                    }
                }>
                    <Route path=path!("") view=Home />
                    <Route path=path!("projects") view=Projects />
                    <Route path=path!("projects/:id") view=ProjectDetail />
                    <Route path=path!("about") view=About />
                </Routes>
            </main>
            <SiteFooter />
        </Router>
    }
}
