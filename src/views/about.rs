use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    let facts: &[(&str, &str)] = &[
        ("focus", "systems · research · long-term software"),
        ("building", "ido · logos — in the open"),
        ("languages", "Rust · Python · Java"),
        ("studying", "M.Sc. Computer Science"),
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
                    "I'm drawn to systems — the hidden structure beneath a problem, the slow accrual of
                    a well-made tool. " <span class="text-primary">"latent."</span>
                    " is where I keep that work: software, research, and long-term experiments, built carefully
                    and reasoned about rather than rushed out."
                </p>
                <p class="about-bio">
                    "I'd rather ship one carefully-reasoned system than ten quick ones. Most of what I make
                    is for the long term — tools I expect to still be using in five years. Engineering as
                    craft, quietly."
                </p>
                <div class="now-bar">
                    <span class="now-label">"now"</span>
                    <span class="now-text">
                        "Finishing a Master's in Computer Science and building "
                        <span class="text-primary">"ido"</span> " and "
                        <span class="text-primary">"logos"</span>
                        " in the open. Writing when something is worth writing down."
                    </span>
                </div>
                <div class="facts-grid">
                    {facts
                        .iter()
                        .map(|(k, v)| {
                            view! {
                                <div class="fact-row">
                                    <span class="fact-key">{*k}</span>
                                    <span class="fact-value">{*v}</span>
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
