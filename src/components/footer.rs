//! Persistent site footer with copyright notice and external profile links.

use leptos::prelude::*;

/// Footer rendered below every page — copyright line and GitHub / LinkedIn links.
#[allow(non_snake_case)]
#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-inner">
                <span class="footer-meta">
                    "latent.foundation · © 2026 · built by Maksymilian Neumann"
                </span>
                <div class="footer-links">
                    <a
                        href="https://github.com/Spiryd"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="footer-link"
                    >
                        "github"
                    </a>
                    <a
                        href="https://linkedin.com/in/maksymilian-neumann"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="footer-link"
                    >
                        "linkedin"
                    </a>
                </div>
            </div>
        </footer>
    }
}
