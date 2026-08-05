# Architecture

## Overview

`latent.foundation` is a client-side rendered (CSR) single-page application written in Rust and compiled to WebAssembly via [Leptos](https://leptos.dev). There is no server — the entire application runs in the browser. Trunk bundles the WASM binary and all static assets into a `dist/` directory; Cloudflare Pages serves that directory globally.

```
Browser
  └── index.html          (Trunk entry point)
       ├── WASM bundle    (Rust → wasm32-unknown-unknown)
       ├── tokens.css     (submodule: fonts, themes, scale, reset)
       ├── components.css (submodule: shared component styles)
       ├── app.css        (this repo: page/layout styles)
       ├── fonts/         (submodule)
       └── assets/        (submodule: SVG marks)
```

Theme machinery and shared components (`ThemeToggle`, `Tag`, `Icon`, `platform::is_mac`)
come from the `latent-ui` crate, not this repo — see
[`vendor/latent-design/docs/ecosystem.md`](../vendor/latent-design/docs/ecosystem.md).

---

## Module structure

```
build.rs                  content pipeline — markdown → HTML at build time
content/
└── log/                  written entries, one markdown file each
src/
├── main.rs               entry point — mounts App to <body>
├── app.rs                root component, router, theme context
├── content.rs            Entry/Heading types + the build-generated arrays
├── data.rs               static project catalogue (no network fetch)
├── title.rs              per-route document.title
├── components/
│   ├── mod.rs
│   ├── header.rs         SiteHeader — nav links, wordmark, theme toggle
│   ├── footer.rs         SiteFooter — copyright, external links
│   └── project_row.rs    ProjectRow — reused on Home + Projects
└── views/
    ├── mod.rs
    ├── home.rs           / — hero + project preview + log preview
    ├── projects.rs       /projects — full archive + filter bar
    ├── project_detail.rs /projects/:id — one project
    ├── log.rs            /log — writing archive + tag filter
    ├── log_entry.rs      /log/:slug — one entry
    ├── ido/              /ido — the ido product page
    │   ├── mod.rs        IdoPage — composes the sections
    │   ├── roadmap.rs    roadmap data + timeline
    │   ├── diagrams.rs   well tree (mono HTML) + link graph (inline SVG)
    │   └── miniatures.rs schematic UI illustrations, aria-hidden
    └── about.rs          /about — colophon
```

---

## Rendering model

Leptos compiles to WASM. `trunk serve` / `trunk build` handles:

1. Compiling `src/main.rs` for `wasm32-unknown-unknown`
2. Running `wasm-bindgen` to produce the JS bindings
3. Copying CSS, fonts, assets, and `_redirects` to `dist/`
4. Injecting the WASM loader into `index.html`

There is no server-side rendering (SSR). The `index.html` shell is served for every URL (via `_redirects`), and Leptos Router handles navigation client-side.

---

## Routing

Routes are declared in [`src/app.rs`](../src/app.rs) using `leptos_router`:

| Path            | Component       |
|-----------------|-----------------|
| `/`             | `Home`          |
| `/projects`     | `Projects`      |
| `/projects/:id` | `ProjectDetail` |
| `/log`          | `Log`           |
| `/log/:slug`    | `LogEntry`      |
| `/ido`          | `IdoPage`       |
| `/about`        | `About`         |
| *(any)*         | fallback 404    |

`/ido` is a bespoke product page, not an archive entry. Which projects have one is data, not a branch in a view: setting `Project::page` makes the archive row link straight there and gives the generic detail page a link onward — see [`ido-page.md`](ido-page.md).

`leptos_router`'s `A` component automatically sets `aria-current="page"` on the active link. The CSS targets `.nav-link[aria-current="page"]` for the active underline — no JS class toggle needed.

The home link uses `exact=true` to prevent it from matching every route (all routes start with `/`).

**Sibling routes match in declaration order — first match wins, not by specificity** (see `leptos_router`'s `matching/nested/tuples.rs`). No current route pair overlaps, but a static path added next to an existing `:param` sibling must be declared *before* it, or the param swallows it.

Per-route `document.title` is handled by `title::use_title`, called from each view. Because every view sets its own title on mount, there is no stale-title state to reset on navigation. This fixes tab labels, history and bookmarks — but **not** crawler or link-preview metadata, which CSR cannot serve per-route; see [`log-section.md`](log-section.md) §9.

---

## Theme system

The theme system is split across three layers to avoid a flash of the wrong theme (FOUC):

1. **`index.html` inline script** — runs synchronously before WASM loads. Reads `localStorage["latent-theme"]`, falls back to `prefers-color-scheme`, then sets `data-theme` on `<html>`.
2. **`theme::initial_theme()`** — reads `data-theme` from the DOM when the Leptos app initialises, so the `RwSignal<bool>` starts in the correct state.
3. **`theme::setup_theme_effect()`** — a `leptos::Effect` that re-runs whenever `is_dark` changes, updating `data-theme` on `<html>` and writing to `localStorage`.

The signal is provided via `leptos::provide_context` from `App` and consumed by `SiteHeader` via `use_context`.

---

## Static data

All content is compiled into the WASM binary as `'static` references — no JSON files, no API calls. The full project list lives in [`src/data.rs`](../src/data.rs). Adding a project means editing that file and redeploying.

`Project` is `Copy` so it can be passed freely across reactive closures without lifetime annotations. `content::Entry` and `content::Heading` follow the same rule.

---

## Content pipeline

Markdown in `content/**/*.md` is rendered to HTML by [`build.rs`](../build.rs) at build time — not in the browser. The parser is a **build-dependency**, so it never reaches the WASM bundle; entry metadata is available to the index page without parsing any body; and a malformed entry fails the build instead of rendering broken in production.

```
content/log/2026-08-05-slug.md
        │
        │  build.rs (host): split +++ TOML frontmatter, render, slugify headings
        ▼
OUT_DIR/html/log/slug.html   ← body, pulled in by include_str!
OUT_DIR/entries.rs           ← pub static LOG: &[Entry], include!d by src/content.rs
```

Bodies are written as separate files rather than inlined as string literals, so arbitrary HTML never has to be escaped into Rust source. Generated code is kept trivial: `rustfmt` and `leptosfmt` never see `OUT_DIR`, but **clippy does**.

`cargo:rerun-if-changed` is emitted for `content/` (catching added and deleted files) and for each file individually (catching edits), so Trunk's watcher rebuilds on any content change.

`draft = true` entries compile in debug and are dropped from release — `PROFILE` is read in `build.rs`, so a draft body is never written into a release `OUT_DIR` at all.

This is deliberately the opposite of ido, which parses markdown in WASM because it renders the user's files live as they type. Both pin the same `pulldown-cmark` version. Full reasoning: [`log-section.md`](log-section.md).

---

## Asset pipeline

| Source                             | Trunk directive   | Output in `dist/`  |
|------------------------------------|-------------------|--------------------|
| `vendor/latent-design/tokens.css`  | `rel="css"`       | `tokens-[hash].css`|
| `vendor/latent-design/components.css` | `rel="css"`    | `components-[hash].css` |
| `style/app.css`                    | `rel="css"`       | `app-[hash].css`   |
| `vendor/latent-design/fonts/`      | `rel="copy-dir"`  | `fonts/`           |
| `vendor/latent-design/assets/`     | `rel="copy-dir"`  | `assets/`          |
| `_redirects`                       | `rel="copy-file"` | `_redirects`       |

The cascade order is mandatory — `tokens.css` defines the custom properties the other two rely on.

CSS font paths use `url("fonts/...")` — relative to the `dist/` root — which resolves correctly because Trunk copies the CSS to the dist root and fonts to `dist/fonts/`.
