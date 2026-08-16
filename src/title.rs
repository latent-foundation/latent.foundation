//! Per-route document title.
//!
//! The site is CSR, so the `<title>` in `index.html` is the only one a crawler or
//! link-preview bot ever sees — that limitation is documented in `docs/log-section.md` §9
//! and is not what this module fixes. What it does fix is the *browser*: tab labels,
//! history entries and bookmarks all follow the route.
//!
//! Every view sets its own title on mount, which is why there is nothing here to reset on
//! navigation — the incoming view always overwrites the outgoing one's.

use leptos::prelude::*;

/// Site name, appended to every page title.
const BRAND: &str = "latent.";

/// Keep `document.title` in sync with the calling view for as long as it is mounted.
///
/// Takes a closure rather than a string so a title that depends on a route param (an
/// entry's own title, say) re-runs when that param changes without remounting the view.
/// An empty string yields the bare brand, for the landing page.
pub fn use_title(title: impl Fn() -> String + 'static) {
    Effect::new(move |_| {
        let title = title();
        let full = if title.is_empty() {
            BRAND.to_string()
        } else {
            format!("{title} — {BRAND}")
        };
        if let Some(document) = window().document() {
            document.set_title(&full);
        }
    });
}
