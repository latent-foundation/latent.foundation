# Development

## Prerequisites

| Tool | Purpose | Install |
|------|---------|---------|
| Rust (stable) | Compiler | `rustup toolchain install stable` |
| wasm32 target | WASM compilation | `rustup target add wasm32-unknown-unknown` |
| Trunk | WASM bundler + dev server | `cargo install trunk --locked` |
| just | Task runner | `cargo install just --locked` |
| leptosfmt | Leptos view! formatter | `cargo install leptosfmt --locked` |

---

## First-time setup

```sh
# Clone and enter the repo
git clone https://github.com/latent-foundation/latent.foundation
cd latent.foundation

# Activate the pre-commit hook (blocks commits that fail fmt-check)
git config core.hooksPath .githooks
```

---

## Daily commands

```sh
trunk serve          # start dev server at http://localhost:8080 (hot-reload)
just fmt             # format all Rust + Leptos code
just fmt-check       # check formatting without modifying (runs in CI + pre-commit)
just check           # cargo check + clippy -D warnings
just verify          # fmt-check + check — the full CI gate
```

> **Always run `just fmt` before committing.** The pre-commit hook runs `just fmt-check` and will block the commit if any file is unformatted.

---

## Formatter notes

This project uses **two formatters** together:

- `cargo fmt` handles plain Rust syntax
- `leptosfmt` handles `view!` macro content that `rustfmt` cannot parse

Run both with `just fmt`. Never run `cargo fmt` alone — it will leave `view!` blocks unformatted and `just fmt-check` will fail.

`rust-analyzer.toml` configures format-on-save to delegate to `leptosfmt --stdin --rustfmt`, so the IDE formats correctly on save if the extension respects the config.

---

## Adding a project

1. Open [`src/data.rs`](../src/data.rs)
2. Add a new `Project { ... }` entry to the `PROJECTS` slice
3. Run `just fmt` and `just verify`
4. The home preview (first 2) and the full archive update automatically on next build

---

## Project structure

```
latent.foundation/
├── src/               Rust source (see docs/architecture.md)
├── style/
│   ├── site.css       design tokens + layout styles
│   └── fonts/         19 self-hosted woff2 files (General Sans + Geist)
├── assets/            SVG wordmarks (dark + light variants)
├── docs/              this documentation
├── .github/
│   └── workflows/
│       ├── ci.yml     runs just verify on every push/PR
│       └── deploy.yml builds + deploys to Cloudflare Pages on push to main
├── .githooks/
│   └── pre-commit     runs just fmt-check before every commit
├── _redirects         Cloudflare Pages SPA fallback (/* → /index.html 200)
├── index.html         Trunk entry point
├── Cargo.toml
├── Trunk.toml         (if present) Trunk build config
└── justfile           task definitions
```

---

## Clippy

CI runs `clippy -D warnings` — all warnings are errors. Common things to watch:

- Use `.is_none_or(|x| ...)` instead of `.map_or(true, |x| ...)` (Rust 2024 edition lint)
- Avoid `use leptos::prelude::*` in files that don't need it
- Keep `#[allow(...)]` off — fix the underlying issue instead
