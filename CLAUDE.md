# CLAUDE.md

Guidance for Claude Code when working in this repository.

## Project

`latent.foundation` is the website for latent. — a personal engineering lab. Rust +
**Leptos** (CSR mode, compiled to WASM), bundled by Trunk. It hosts the project archive
for the two sub-projects: **ido** (local-first notes/wiki/tasks) and **logos** (market
data and trading research).

This repo is the **app layer** of a three-repo design system. Brand assets and shared
Rust behavior come from two sibling repos — see the canon below.

## Canon — read these first

Shared ecosystem knowledge is **not duplicated here**. It lives in `latent-design` and is
vendored into this repo at `vendor/latent-design/docs/` (populated after
`git submodule update --init --recursive`):

| Doc | What it covers |
|---|---|
| [ecosystem.md](vendor/latent-design/docs/ecosystem.md) | The 3-layer architecture, sharing model, CSS cascade, anti-FOUC, dependency pinning |
| [conventions.md](vendor/latent-design/docs/conventions.md) | Rust/Leptos/Trunk conventions, formatting, signals, theme, CI |
| [bootstrap-new-app.md](vendor/latent-design/docs/bootstrap-new-app.md) | Standing up a new latent. app |
| [glossary.md](vendor/latent-design/docs/glossary.md) | What latent. / ido / logos / each layer mean |
| [knowledge-architecture.md](vendor/latent-design/docs/knowledge-architecture.md) | Knowledge-centralization + MCP roadmap |

The `/latent-design` skill carries the full design system (color, type, assets). Invoke it
when building any UI.

## This app

**Layers consumed:**
- `vendor/latent-design` — git submodule: CSS tokens, components, fonts, SVGs (+ this canon).
- `latent-ui` — Cargo git dep: `ThemeToggle`, `Tag`, `theme::{initial_theme, setup_theme_effect}`.

**Structure:**
```
src/
  main.rs            entry — mounts <App/>
  app.rs             root component, router, theme bootstrap
  data.rs            project data
  components/        header, footer, project_row
  views/             home, projects, project_detail, about
style/app.css        page/layout styles — this repo's ONLY stylesheet
vendor/latent-design submodule (CSS, fonts, assets, docs/)
index.html           Trunk entry — CSS cascade + copy-dir
```

**App-specific rules:**
- `style/app.css` is the only CSS that belongs here. Tokens/components come from the
  submodule and load first — never paste them inline. Cascade:
  `tokens.css` → `components.css` → `app.css` (mandatory order).
- Format with `just fmt` (cargo fmt + leptosfmt) — **never `cargo fmt` alone**; it can't
  format `view!` macros. Details in [conventions.md](vendor/latent-design/docs/conventions.md).
- For local dev against a sibling `latent-ui`, add a `[patch]` override in `Cargo.toml`
  (do not commit) — see [ecosystem.md](vendor/latent-design/docs/ecosystem.md#dependency-pinning).

## Commands

```sh
just fmt                # format (cargo fmt + leptosfmt)
just verify             # fmt-check + check — exactly what CI runs
trunk serve             # local dev server, hot reload
trunk build --release   # production build → dist/
```

## Setup (once per clone)

```sh
git submodule update --init --recursive   # populate vendor/latent-design
git config core.hooksPath .githooks       # activate pre-commit format gate
```

## CI

`.github/workflows/ci.yml` runs `just verify` on every push to `main` and all PRs; PRs
must pass before merging. Both CI and deploy checkouts use `submodules: recursive` so
`vendor/latent-design` is populated.
