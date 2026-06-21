# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`latent.foundation` is the website for latent. — a personal engineering lab. It is written in **Rust** with **Leptos** (CSR mode, compiled to WASM). The site hosts two sub-projects: **ido** (local-first notes/wiki/tasks) and **logos** (market data and trading research tooling).

## Commands

```sh
just fmt          # format all Rust + Leptos code (cargo fmt + leptosfmt)
just fmt-check    # check formatting without modifying (runs in pre-commit hook)
just check        # cargo check + clippy -D warnings
just verify       # fmt-check + check — this is what CI runs
trunk serve       # start local dev server (hot-reload, runs on localhost)
```

After a fresh clone, activate the pre-commit hook:
```sh
git config core.hooksPath .githooks
```

The pre-commit hook blocks commits that fail `just fmt-check`. Run `just fmt` before committing.

## Architecture

**Rendering model:** CSR (client-side rendering). Leptos compiles to WASM; `trunk serve` / `trunk build` bundles everything. `index.html` is Trunk's entry point.

**Formatter:** `leptosfmt` is required alongside `cargo fmt`. It handles `view!` macro formatting that `rustfmt` cannot parse. `rust-analyzer.toml` configures rust-analyzer's format-on-save to delegate to `leptosfmt --stdin --rustfmt`. Format with `just fmt`, never `cargo fmt` alone.

**Signals:** Leptos 0.8 uses `signal()` returning `(ReadSignal<T>, WriteSignal<T>)`. Mutate via `.write()` (returns a `DerefMut` guard). Pass `ReadSignal<T>` into child components as props.

**Component pattern:**
```rust
#[component]
fn MyComponent(prop: ReadSignal<i32>) -> impl IntoView {
    view! { <div>{prop}</div> }
}
```

## Shared ecosystem

This repo is one of three layers in the latent. design system:

| Layer | Repo | What it provides |
|---|---|---|
| Brand canon | `latent-design` | CSS tokens, fonts, SVGs, component styles |
| Rust behavior | `latent-ui` | `ThemeToggle`, `Tag`, theme machinery |
| App | `latent.foundation` (this repo) | Page layout, routing, page components |

### Static assets — `vendor/latent-design` submodule

CSS, fonts, and SVGs come from the `latent-design` repo via a git submodule at `vendor/latent-design`.

After cloning, initialize it:
```sh
git submodule update --init --recursive
```

Trunk loads them in `index.html` in this exact order (cascade is mandatory):
```html
<link data-trunk rel="css"      href="vendor/latent-design/tokens.css" />
<link data-trunk rel="css"      href="vendor/latent-design/components.css" />
<link data-trunk rel="css"      href="style/app.css" />
<link data-trunk rel="copy-dir" href="vendor/latent-design/fonts" />
<link data-trunk rel="copy-dir" href="vendor/latent-design/assets" />
```

`style/app.css` is the only CSS file that belongs to this repo — page/layout styles only. Never paste tokens or component styles inline; edit `latent-design` instead.

### Rust components — `latent-ui` crate

Shared Leptos components and theme machinery live in `latent-ui`. The Cargo dep is a pinned git tag in production:

```toml
latent-ui = { git = "https://github.com/latent-foundation/latent-ui", tag = "v0.1.0" }
```

For local development, override it in `Cargo.toml` with a `[patch]` table (do not commit this):
```toml
[patch."https://github.com/latent-foundation/latent-ui"]
latent-ui = { path = "../latent-ui" }
```

Key exports used in this repo:
- `latent_ui::theme::{initial_theme, setup_theme_effect, STORAGE_KEY}`
- `latent_ui::ThemeToggle`
- `latent_ui::Tag`

## Design

latent. is typography-first, warm-neutral, and architecturally restrained. The guiding principle: **depth over noise** — borders over shadows, accent used sparingly, minimal interfaces.

The `/latent-design` skill contains the full design system (color tokens, type scale, UI kits for site/ido/logos, font files, wordmark SVGs). Invoke it when building any UI.

- Fonts: Geist (UI) + Geist Mono (code), self-hosted via submodule. Wordmark uses SVG assets from submodule.
- Icons: Lucide via CDN — never hand-drawn icons or emoji
- Dark theme: warm graphite; light theme: stone/paper

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs `just verify` on every push to `main` and on all PRs. PRs must pass before merging.

Both CI and deploy workflows use `submodules: recursive` on checkout so `vendor/latent-design` is populated.
