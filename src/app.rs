use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::{
    components::{SiteFooter, SiteHeader},
    theme::{initial_theme, setup_theme_effect},
    views::{About, Home, Projects},
};

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
                    <Route path=path!("about") view=About />
                </Routes>
            </main>
            <SiteFooter />
        </Router>
    }
}
