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

Shared ecosystem knowledge is **not duplicated here**. It lives in the **latent well** —
an ido well that is the single authoritative home for latent. docs, readable in ido or
over its MCP server (the former `vendor/latent-design/docs/` paths hold pointer stubs):

| Well wiki page | What it covers |
|---|---|
| `ecosystem` | The 3-layer architecture, sharing model, CSS cascade, anti-FOUC, dependency pinning |
| `conventions` | Rust/Leptos/Trunk conventions, formatting, signals, theme, CI |
| `bootstrap-new-app` | Standing up a new latent. app |
| `glossary` | What latent. / ido / logos / each layer mean |
| `knowledge-architecture` | Knowledge-centralization + MCP plan |

This repo's own design docs moved there too, as the `website/` wiki group:
`website-architecture`, `website-deployment`, `website-development`,
`website-log-section`, `website-ido-page`. ("website", not "foundation" —
latent.foundation is the domain and repo name, not the product's name.)

The `/latent-design` skill carries the full design system (color, type, assets). Invoke it
when building any UI.

## This app

**Layers consumed:**
- `vendor/latent-design` — git submodule: CSS tokens, components, fonts, SVGs (+ this canon).
- `latent-ui` — Cargo git dep: `ThemeToggle`, `Tag`, `Icon`, `platform::is_mac`,
  `theme::{initial_theme, setup_theme_effect}`.

**Structure:**
```
build.rs             renders content/**/*.md → HTML at build time
content/log/         written entries, one markdown file each
src/
  main.rs            entry — mounts <App/>
  app.rs             root component, router, theme bootstrap
  content.rs         Entry/Heading types + the build-generated arrays
  data.rs            project data
  title.rs           per-route document.title
  components/        header, footer, project_row
  views/             home, projects, project_detail, log, log_entry, about
    ido/             the /ido product page (mod, roadmap, diagrams, miniatures)
style/app.css        page/layout styles — this repo's ONLY stylesheet
vendor/latent-design submodule (CSS, fonts, assets, docs/)
index.html           Trunk entry — CSS cascade + copy-dir
```

**Content:** markdown in `content/log/` (`YYYY-MM-DD-slug.md`, `+++`-fenced TOML
frontmatter) is rendered to HTML by `build.rs` and compiled in — the parser never reaches
the WASM bundle and a malformed entry fails the build, not the browser. Note this is the
*opposite* of ido, which parses in the browser because it renders the user's files live;
both use the same pinned `pulldown-cmark`. Adding a collection (e.g. ido guides) is one
line in `build.rs`'s `COLLECTIONS` plus a route. Design notes:
the well's `website-log-section` and `website-ido-page` pages.

**The `/ido` page:** every claim on it must be traceable to ido's own docs (`README.md`,
`CLAUDE.md`, the well's `ido-mcp-design` page) — nothing shipped in the future tense, nothing unshipped
in the present, and no version numbers (a version on a public roadmap reads as a date).
The miniatures in `views/ido/miniatures.rs` are schematic illustrations, not screenshots:
keep them `aria-hidden` with the prose carrying the information, and never let one imply a
feature ido lacks. Which projects get a bespoke page is data — set `Project::page`.

**App-specific rules:**
- `style/app.css` is the only CSS that belongs here. Tokens/components come from the
  submodule and load first — never paste them inline. Cascade:
  `tokens.css` → `components.css` → `app.css` (mandatory order).
- Format with `just fmt` (cargo fmt + leptosfmt) — **never `cargo fmt` alone**; it can't
  format `view!` macros. Details in the well's `conventions` page.
- For local dev against a sibling `latent-ui`, add a `[patch]` override in `Cargo.toml`
  (do not commit) — see the well's `ecosystem` page (dependency pinning).

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

# Wire the /latent-design skill — symlink, so it tracks the submodule automatically.
# `.claude/` is gitignored, so this is machine-local: every clone repeats it.
mkdir -p .claude/skills
ln -s ../../vendor/latent-design .claude/skills/latent-design
```

Run the symlink step **after** the submodule init above, or it dangles. On Windows use
`New-Item -ItemType SymbolicLink` (needs Developer Mode) — see the well's
`bootstrap-new-app` page.

## CI

`.github/workflows/ci.yml` runs `just verify` on every push to `main` and all PRs; PRs
must pass before merging. Both CI and deploy checkouts use `submodules: recursive` so
`vendor/latent-design` is populated.
