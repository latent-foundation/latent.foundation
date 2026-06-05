# Architecture

## Overview

`latent.foundation` is a client-side rendered (CSR) single-page application written in Rust and compiled to WebAssembly via [Leptos](https://leptos.dev). There is no server — the entire application runs in the browser. Trunk bundles the WASM binary and all static assets into a `dist/` directory; Cloudflare Pages serves that directory globally.

```
Browser
  └── index.html          (Trunk entry point)
       ├── WASM bundle    (Rust → wasm32-unknown-unknown)
       ├── site.css       (design tokens + layout)
       └── assets/        (SVG marks)
```

---

## Module structure

```
src/
├── main.rs               entry point — mounts App to <body>
├── app.rs                root component, router, theme context
├── data.rs               static project catalogue (no network fetch)
├── theme.rs              dark/light theme: init + reactive effect
├── components/
│   ├── mod.rs
│   ├── header.rs         SiteHeader — nav links, wordmark, theme toggle
│   ├── footer.rs         SiteFooter — copyright, external links
│   └── project_row.rs    ProjectRow — reused on Home + Projects
└── views/
    ├── mod.rs
    ├── home.rs           / — hero + 2-project preview
    ├── projects.rs       /projects — full archive + filter bar
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

| Path        | Component    |
|-------------|--------------|
| `/`         | `Home`       |
| `/projects` | `Projects`   |
| `/about`    | `About`      |
| *(any)*     | fallback 404 |

`leptos_router`'s `A` component automatically sets `aria-current="page"` on the active link. The CSS targets `.nav-link[aria-current="page"]` for the active underline — no JS class toggle needed.

The home link uses `exact=true` to prevent it from matching every route (all routes start with `/`).

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

`Project` is `Copy` so it can be passed freely across reactive closures without lifetime annotations.

---

## Asset pipeline

| Source                  | Trunk directive          | Output in `dist/`    |
|-------------------------|--------------------------|----------------------|
| `style/site.css`        | `rel="css"`              | `site-[hash].css`    |
| `style/fonts/`          | `rel="copy-dir"`         | `fonts/`             |
| `assets/`               | `rel="copy-dir"`         | `assets/`            |
| `_redirects`            | `rel="copy-file"`        | `_redirects`         |

CSS font paths use `url("fonts/...")` — relative to the `dist/` root — which resolves correctly because Trunk copies the CSS to the dist root and fonts to `dist/fonts/`.
