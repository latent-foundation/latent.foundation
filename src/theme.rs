use leptos::prelude::*;

pub fn initial_theme() -> bool {
    // Read data-theme set by the inline <script> in index.html before WASM loads.
    // Returns true for dark, false for light.
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
