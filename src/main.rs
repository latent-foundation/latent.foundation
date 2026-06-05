mod app;
mod components;
mod data;
mod theme;
mod views;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
