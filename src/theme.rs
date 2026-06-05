//! Dark / light theme wiring.
//!
//! The inline `<script>` in `index.html` sets `data-theme` on `<html>` before
//! the WASM bundle loads, preventing a flash of the wrong theme (FOUC).
//! [`initial_theme`] reads that attribute so the Leptos signal starts in the
//! correct state. [`setup_theme_effect`] then keeps the DOM attribute and
//! `localStorage` in sync whenever the signal changes.

use leptos::prelude::*;

/// Reads the theme that the inline HTML script already applied to `<html data-theme>`.
///
/// Returns `true` for dark, `false` for light. Defaults to dark if the DOM is
/// unavailable (e.g. during server-side evaluation or in tests).
pub fn initial_theme() -> bool {
    let Some(window) = web_sys::window() else {
        return true;
    };
    let Some(doc) = window.document() else {
        return true;
    };
    let Some(html) = doc.document_element() else {
        return true;
    };
    html.get_attribute("data-theme")
        .map(|t| t != "light")
        .unwrap_or(true)
}

/// Registers a reactive effect that syncs theme changes back to the DOM.
///
/// Must be called once from [`crate::app::App`]. Each time `is_dark` changes,
/// the effect writes `data-theme` on `<html>` and persists the choice to
/// `localStorage` under `"latent-theme"` so the next page load restores it
/// before the WASM bundle boots.
pub fn setup_theme_effect(is_dark: RwSignal<bool>) {
    Effect::new(move |_| {
        let theme = if is_dark.get() { "dark" } else { "light" };

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(doc) = window.document() else {
            return;
        };
        if let Some(html) = doc.document_element() {
            let _ = html.set_attribute("data-theme", theme);
        }
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("latent-theme", theme);
        }
    });
}
