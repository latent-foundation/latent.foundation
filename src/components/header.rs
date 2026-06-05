use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

#[component]
pub fn SiteHeader() -> impl IntoView {
    let is_dark = use_context::<RwSignal<bool>>().expect("theme context");

    // suppress unused warning — location is used reactively for aria-current via the A component
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
