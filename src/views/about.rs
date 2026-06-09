//! About / colophon page (`/about`): the person behind the lab.
//!
//! The facts grid is built from a `Vec<(&str, String)>` so key/value pairs
//! can be added or reordered without touching the template markup.
//! The "experience" entry is computed from the engineering start date (July 2024)
//! via [`js_sys::Date`] so it increments automatically each year.
//! The "elsewhere" row is appended outside the iterator because it contains
//! rich link markup rather than plain strings.

use js_sys::Date;
use leptos::prelude::*;

/// Colophon page: monogram, bio, "now" status bar, facts grid, and external links.
#[allow(non_snake_case)]
#[component]
pub fn About() -> impl IntoView {
    // Compute full years of professional experience since July 2024.
    // get_month() is 0-indexed in JS, so +1 to normalise to 1–12.
    let now = Date::new_0();
    let current_year = now.get_full_year() as i32;
    let current_month = now.get_month() as i32 + 1;
    let elapsed_months = (current_year - 2024) * 12 + (current_month - 7);
    let years = (elapsed_months / 12).max(0) as u32;
    let experience = format!("{}+ years as a software engineer", years);

    let facts: Vec<(&str, String)> = vec![
        (
            "focus",
            "thoroughly designed software · long-term systems · engineering as a craft".into(),
        ),
        ("languages", "Rust · Python · Java".into()),
        ("experience", experience),
        (
            "education",
            "M.Sc. Computer Science @ Wroclaw University of Science and Technology".into(),
        ),
        ("location", "Wrocław, Poland".into()),
    ];

    view! {
        <div class="container">
            <div class="about-page">
                <p class="about-kicker">"colophon · the person behind the lab"</p>
                <div class="about-header">
                    <div class="monogram">"MN"</div>
                    <div class="about-intro">
                        <h1 class="about-headline">"latent. is the work of one person."</h1>
                        <p class="about-byline">"Maksymilian Neumann · software engineer"</p>
                    </div>
                </div>
                <p class="about-bio-lg">
                    "I'm drawn to systems, the hidden structure beneath a larger whole, the slow accrual of
                    a well-made tool. " <span class="text-primary">"latent."</span>
                    " is where I keep that work: software, research, and long-term experiments, built carefully
                    and reasoned about rather than rushed out."
                </p>
                <p class="about-bio">
                    "I'd rather ship one carefully-reasoned system than ten quick ones. Most of what I make
                    is for the long term, tools I expect to still be using in five years."
                </p>
                <div class="now-bar">
                    <span class="now-label">"now"</span>
                    <span class="now-text">
                        "Finishing a Master's in Computer Science and building "
                        <span class="text-primary">"ido"</span> " and "
                        <span class="text-primary">"logos"</span>
                        " in the open. Writing when something is worth writing down. "
                        "Fulltime and fullstack software engineer @ BNY."
                    </span>
                </div>
                <div class="facts-grid">
                    {facts
                        .into_iter()
                        .map(|(k, v)| {
                            view! {
                                <div class="fact-row">
                                    <span class="fact-key">{k}</span>
                                    <span class="fact-value">{v}</span>
                                </div>
                            }
                        })
                        .collect_view()} <div class="fact-row">
                        <span class="fact-key">"elsewhere"</span>
                        <div class="elsewhere-links">
                            <a
                                href="https://github.com/Spiryd"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="elsewhere-link"
                            >
                                "github"
                            </a>
                            <a
                                href="https://linkedin.com/in/maksymilian-neumann"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="elsewhere-link"
                            >
                                "linkedin"
                            </a>
                            <a href="mailto:neumann.maks@gmail.com" class="elsewhere-link">
                                "email"
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
