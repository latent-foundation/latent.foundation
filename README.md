# latent.foundation

Source code for [latent.foundation](https://latent.foundation) — built with Rust and [Leptos](https://leptos.dev) (CSR/WASM), bundled by [Trunk](https://github.com/trunk-rs/trunk), deployed to Cloudflare Pages.

## prerequisites

- [Rust](https://rustup.rs) stable toolchain + `wasm32-unknown-unknown` target
- [`just`](https://github.com/casey/just) — `cargo install just --locked`
- [`leptosfmt`](https://github.com/bram209/leptosfmt) — `cargo install leptosfmt --locked`
- [`trunk`](https://trunkrs.dev) — `cargo install trunk --locked`

## setup

```sh
git clone https://github.com/latent-foundation/latent.foundation
cd latent.foundation
git submodule update --init --recursive   # pull in vendor/latent-design
git config core.hooksPath .githooks       # activate the pre-commit format hook
```

The submodule line populates `vendor/latent-design` (CSS, fonts, SVGs) — the build won't
find its styles or assets without it. The hook line installs the pre-commit formatting
gate. Run both once per clone.

## commands

```sh
just fmt      # format (cargo fmt + leptosfmt)
just verify   # fmt-check + clippy — runs in CI
trunk serve   # local dev server with hot reload
trunk build --release  # production build → dist/
```

## project structure

This repo is the **app layer** — page layout, routing, and page components. Brand assets
and shared Rust behavior live in two sibling repos:

| Layer | Repo | Consumed as | Provides |
|---|---|---|---|
| Brand canon | [`latent-design`](https://github.com/latent-foundation/latent-design) | git submodule → `vendor/latent-design` | CSS tokens, component styles, fonts, SVGs |
| Rust behavior | [`latent-ui`](https://github.com/latent-foundation/latent-ui) | Cargo git dep | `ThemeToggle`, `Tag`, theme machinery |
| App | `latent.foundation` *(this repo)* | — | pages, routing, layout |

```
src/
  main.rs              entry point — mounts <App/>
  app.rs               root component, router, theme bootstrap
  data.rs              project data
  components/          header, footer, project_row
  views/               home, projects, project_detail, about
style/
  app.css              page/layout styles only — THIS repo's only CSS
vendor/latent-design/  submodule: tokens.css, components.css, fonts/, assets/
index.html             Trunk entry — wires CSS cascade + copies fonts/assets
```

`style/app.css` is the only stylesheet that belongs here. Tokens and component styles come
from the submodule and load first (`tokens.css` → `components.css` → `app.css` — the cascade
order is mandatory). Never paste tokens inline; edit `latent-design` instead.

### local development against latent-ui

The `latent-ui` dep is pinned to a git tag for production builds. To work against a local
checkout, add a `[patch]` table to `Cargo.toml` (do not commit it):

```toml
[patch."https://github.com/latent-foundation/latent-ui"]
latent-ui = { path = "../latent-ui" }
```
