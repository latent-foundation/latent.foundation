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

## Design

latent. is typography-first, warm-neutral, and architecturally restrained. The guiding principle: **depth over noise** — borders over shadows, accent used sparingly, minimal interfaces.

The `/latent-design` skill contains the full design system (color tokens, type scale, UI kits for site/ido/logos, font files, wordmark SVGs). Invoke it when building any UI.

- Fonts: General Sans (body) + Geist Mono (code), self-hosted
- Icons: Lucide via CDN — never hand-drawn icons or emoji
- Dark theme: warm graphite; light theme: stone/paper

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs `just verify` on every push to `main` and on all PRs. PRs must pass before merging.
