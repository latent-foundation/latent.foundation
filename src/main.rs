//! Entry point for the latent.foundation WASM bundle.
//!
//! `trunk build` compiles this crate to WASM and injects it into `index.html`.
//! [`console_error_panic_hook`] forwards Rust panics to the browser console so
//! they surface as readable messages during development instead of opaque traps.

mod app;
mod components;
mod data;
mod views;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
