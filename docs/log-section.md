# `/log` — markdown-authored writing

> Design + implementation plan. Add a writing section to `latent.foundation` whose
> entries are authored as plain markdown files in the repo and compiled into the
> WASM bundle at build time.

Companion plan: [ido-page.md](ido-page.md), which reuses the content pipeline built here.

---

## 1. Goals and non-goals

**Goals**

1. Write an entry by dropping a `.md` file into `content/log/` — no CMS, no network fetch,
   no runtime parsing.
2. Metadata (title, date, summary, tags, reading time, headings) available to the index
   page without shipping every entry body to render a list.
3. A malformed entry fails `just verify` and CI — not the browser.
4. Prose styling that is indistinguishable from ido's reading view, so the ecosystem
   reads as one system.

**Non-goals**

- Comments, analytics, newsletters, social embeds.
- Per-entry OpenGraph/meta tags (CSR limitation — see §9).
- User-supplied content, and therefore HTML sanitization (§3.4).

---

## 2. The core decision: parse at build time, not in the browser

ido parses markdown **in WASM** (`pulldown-cmark` in `src/markdown.rs`) because it renders
*the user's* files, live, as they type. This site's content is fixed the moment the binary
is built, so the parse belongs in the build.

| | build-time (chosen) | runtime, like ido |
|---|---|---|
| WASM size | parser not shipped | +~150 KB of parser in every page load |
| Bad frontmatter | build fails, CI catches it | renders broken in production |
| Index page | reads metadata directly | must parse every entry to list them |
| Content updates | recompile (already required — content is in the repo) | recompile anyway |

The seam stays shared: both use `pulldown-cmark` 0.13 with the same event-transform
shape, so anything learned in one is portable to the other.

**Consequence to accept:** `build.rs` becomes part of the build. It is ~150 lines and runs
on the host, so it costs nothing at runtime and nothing in bundle size.

---

## 3. Content pipeline

### 3.1 Layout

```
content/
  log/
    2026-08-05-why-plain-markdown.md
    2026-09-01-regime-detection-notes.md
build.rs                     ← reads content/, emits Rust into OUT_DIR
src/content.rs               ← Entry / Heading types + the generated ENTRIES
```

`src/content.rs` is deliberately **not** named `log.rs`: the module is a general content
pipeline, and [ido-page.md](ido-page.md) §7 adds a second collection (`content/ido/`)
through the same code. It also avoids ever shadowing the `log` crate.

### 3.2 Entry format

TOML frontmatter between `+++` fences. `+++` rather than `---` because `---` is also a
markdown thematic break, which makes a missing closing fence parse as valid markdown
instead of failing loudly.

```markdown
+++
title   = "Why ido stores everything as plain markdown"
date    = "2026-08-05"
summary = "One paragraph, shown in the index. Plain text, no markup."
tags    = ["ido", "local-first"]
draft   = false
+++

Body starts here. Standard CommonMark plus tables, footnotes, strikethrough,
task lists and smart punctuation.
```

- **slug** — the filename minus the `YYYY-MM-DD-` prefix and `.md`. The date prefix keeps
  the directory sorted on disk and out of the URL: the file above serves at
  `/log/why-ido-stores-everything-as-plain-markdown`.
- **date** — ISO `YYYY-MM-DD`. Sorts lexicographically, so build-time ordering is a string
  sort; no date crate needed.
- **draft** — `true` entries are compiled **in debug only** (`build.rs` reads the `PROFILE`
  env var). They appear under `trunk serve` and vanish from `trunk build --release`. Draft
  bodies never reach the deployed bundle.

### 3.3 What `build.rs` does

```rust
// build.rs — sketch, not the finished file
fn main() {
    println!("cargo:rerun-if-changed=content");           // new/deleted files
    for path in md_files("content/log") {
        println!("cargo:rerun-if-changed={}", path.display());  // edits
    }

    // for each file: split frontmatter, parse TOML, render body, compute metadata
    // write OUT_DIR/html/<slug>.html   (bodies, one file each)
    // write OUT_DIR/entries.rs         (the static array, include!d by src/content.rs)
}
```

Bodies go to **separate files** referenced by `include_str!`, not inlined as string
literals in the generated Rust. Inlining would require escaping arbitrary HTML into a
Rust literal — a raw-string hash-count problem waiting to happen the first time an entry
contains `"#`.

Generated `entries.rs`:

```rust
pub static ENTRIES: &[Entry] = &[
    Entry {
        slug: "why-ido-stores-everything-as-plain-markdown",
        title: "Why ido stores everything as plain markdown",
        date: "2026-08-05",
        summary: "…",
        tags: &["ido", "local-first"],
        reading_minutes: 7,
        headings: &[Heading { level: 2, id: "the-well", text: "The well" }],
        html: include_str!(concat!(env!("OUT_DIR"), "/html/why-ido-….html")),
    },
];
```

Metadata strings are emitted with `{:?}` formatting, which produces a correctly escaped
Rust string literal for free.

**Build dependencies** (host-only, zero WASM cost):

```toml
[build-dependencies]
pulldown-cmark = { version = "0.13", default-features = false, features = ["html"] }
toml  = "0.8"
serde = { version = "1", features = ["derive"] }
```

### 3.4 The render step

Mirrors ido's `markdown::render` seam — parse to events, map a transform over them, push
HTML:

1. **Options:** `ENABLE_TABLES | ENABLE_STRIKETHROUGH | ENABLE_FOOTNOTES |
   ENABLE_SMART_PUNCTUATION | ENABLE_TASKLISTS`. (`ENABLE_MATH` + `latex2mathml` is a
   later add-on — §10.)
2. **Heading anchors.** `pulldown-cmark` only populates `Tag::Heading { id }` from explicit
   `{#id}` attributes, so auto-slugs are ours: buffer events between `Start(Heading)` and
   `End(Heading)`, slugify the collected text (same algorithm as ido's `markdown::slugify`),
   re-emit with the id. The same pass collects the `headings` array used by the TOC.
3. **External links** get `target="_blank" rel="noopener noreferrer"`; internal ones
   (starting `/`) are left alone so the router handles them.
4. `html::push_html`.

**No sanitization, deliberately.** Every byte of input is authored in this repo by its
owner and reviewed in git. Sanitizing self-authored content would only break intentional
inline HTML. This assumption must be revisited the day content comes from anywhere else.

### 3.5 Failing loudly

`build.rs` panics with a clear message — which surfaces as a build error in `just verify`
and CI — on: missing or unterminated frontmatter, missing `title`/`date`, a malformed
date, or two files resolving to the same slug.

---

## 4. Routes

```rust
<Route path=path!("log") view=Log />
<Route path=path!("log/:slug") view=LogEntry />
```

No route conflicts. Deep links already work in production: `_redirects` serves
`index.html` for every path, and `trunk serve` does the same in development.

> **Verified while planning:** `leptos_router` 0.8 matches sibling routes **in declaration
> order — first match wins**, not by specificity (`matching/nested/tuples.rs`). It doesn't
> bite here, but it is load-bearing for any future `log/:slug` vs `log/<static>` pair.

---

## 5. Views

### `/log` — index (`src/views/log.rs`)

Reuses the archive idiom already on `/projects` rather than inventing a card grid:
a `.section-label` header, then hairline-separated rows.

```
LOG                                                    5 entries
────────────────────────────────────────────────────────────────
2026-08-05   Why ido stores everything as plain markdown
             One paragraph of summary, muted, ~52ch.
             [IDO] [LOCAL-FIRST]                            7 min
────────────────────────────────────────────────────────────────
```

Date in mono/muted on the left, matching `.project-year`'s role in a project row.
Tag filtering copies the `/projects` filter-bar pattern exactly — `RwSignal<Option<&str>>`
plus a `Memo` for the filtered list — so the two archives behave identically. Tags come
from a build-time-deduplicated list.

### `/log/:slug` — entry (`src/views/log_entry.rs`)

Same shape as `.project-detail` (720px column, `← log` back-link, hairline divider) so
the site has one reading layout, not two:

- back-link · title · meta line (`date · N min · tags`)
- optional TOC — rendered only when the entry has ≥3 `h2`s; a right-hand `position: sticky`
  aside above 1080px, collapsing above the body below that
- `<div class="markdown-body" inner_html=entry.html />`
- footer: previous / next entry links

Unknown slug renders the same muted "not found." treatment `ProjectDetail` uses.

---

## 6. Prose styles

Add a `=== MARKDOWN BODY ===` block to `style/app.css`, adapted from ido's
`.markdown-body` rules (`~/Desktop/ido/style/app.css:757-960`). That block is already
token-only — headings on `--tracking-heading`, links on `--color-accent` with a
`--color-border-strong` underline, code on `--color-code-bg`/`--color-code-text` — so it
transfers with only the outer box changed (this site's 720px column rather than ido's
padded pane).

**Keep the class name `.markdown-body`.** Identical naming in both apps means that when
these rules are promoted into `latent-design/components.css`, both apps delete their copy
and nothing else changes.

> **Follow-up, not part of this work:** promoting `.markdown-body` upstream is a PR against
> `latent-design` plus a submodule bump here and in ido. Worth doing once the rules have
> settled under real content — this repo is the second consumer, which is exactly the
> ecosystem's bar for promotion.

Code blocks ship **unhighlighted**, styled by tokens alone, matching ido. Syntax
highlighting is a separable add-on (§10).

---

## 7. Site integration

- **Header** (`src/components/header.rs`) — add `log` between `projects` and `about`.
  `aria-current` styling is automatic.
- **Home** (`src/views/home.rs`) — a second section below projects: `.section-header`
  reading "Log" with a `view all →` link, then the two most recent entries. Same restraint
  as the projects preview.
- **Footer** — unchanged.

---

## 8. Docs to update

- `CLAUDE.md` — `content/` and `build.rs` in the structure block; a short "adding a log
  entry" recipe.
- `docs/architecture.md` — routes table, module structure, asset pipeline.
  **Note: this file is already stale** — it documents a `src/theme.rs` (now in `latent-ui`)
  and `style/site.css` (now `style/app.css`). Worth correcting in the same pass.
- `README.md` — one line on where writing lives.

---

## 9. Known limitation: per-entry meta tags

The site is CSR. A crawler or a link-preview bot sees `index.html`'s static `<title>` and
OG tags for every URL, including `/log/<slug>`.

**In scope:** set `document.title` per route via `web-sys` (`Document` is already an
enabled feature) — a small `src/title.rs` helper called from an `Effect` in each view.
That fixes browser tabs, history, and bookmarks.

**Out of scope, documented for later:** real per-entry OG tags need static HTML per URL.
The contained version is a post-build step — after `trunk build --release`, walk the
entries and write `dist/log/<slug>/index.html`: a copy of the shell with the entry's
`<title>`/`<meta>` swapped in. Cloudflare Pages serves those files directly and the router
takes over on hydration. Roughly 40 lines and a `just` recipe. Only worth it if link
previews start mattering.

---

## 10. Optional add-ons, each independently shippable

| Add-on | Cost | Notes |
|---|---|---|
| **Syntax highlighting** | `syntect` build-dep + ~12 CSS rules | `ClassedHTMLGenerator` with `ClassStyle::SpacedPrefixed { prefix: "st-" }` emits class names instead of inline colors; map `.st-keyword` / `.st-string` / `.st-comment` to tokens so it themes correctly in light **and** dark. Build-time only — no WASM cost. |
| **Math** | `latex2mathml` build-dep + `ENABLE_MATH` | Same crate and approach ido uses; `$…$` / `$$…$$` → MathML. Worth it if writing about markov models or backtesting. |
| **RSS/Atom** | ~50 lines in `build.rs` | Caveat: Trunk resolves `copy-file` from `index.html`, so `static/feed.xml` must exist before the build — meaning the generated feed has to be **committed**, not gitignored. Slightly impure. Defer until someone actually asks for a feed. |
| **ido guides** | routes only | `content/ido/*.md` through this same pipeline — see [ido-page.md](ido-page.md) §7. |

---

## 11. Work order

| # | Step | Done when |
|---|---|---|
| 1 | `build.rs` + `src/content.rs` + one real entry in `content/log/` | `cargo check` compiles the generated array; a deliberately broken frontmatter fails the build with a readable message |
| 2 | `.markdown-body` styles in `style/app.css` | prose renders in both themes and matches ido's reading view side by side |
| 3 | `/log` index view + tag filter | list renders, filter behaves like `/projects` |
| 4 | `/log/:slug` entry view + TOC + prev/next | deep link reloads correctly under `trunk serve` |
| 5 | Header nav + home preview + `document.title` | `log` link shows `aria-current`; tab title changes per entry |
| 6 | Docs (`CLAUDE.md`, `architecture.md`, `README.md`) | structure block matches reality, `architecture.md` staleness fixed |

`just verify` green at every step. Generated code lives in `OUT_DIR`, so `cargo fmt` and
`leptosfmt` never see it — but **clippy does**, so the generated array stays trivially
simple (plain struct literals, no expressions).

---

## 12. Where this doc lives

Interim home. The ecosystem's documentation moves into ido itself, in the window **between
ido 1.0.0 and 1.1.0** — ahead of the MCP server that arrives in 1.1.0, so it has canon to
serve on day one. See [ido-page.md](ido-page.md) §10 for the full note and what it implies
for how these docs are written.

There is a pleasing symmetry worth naming: this plan builds a pipeline that renders
markdown files from a folder in a repo, and the destination is an app whose whole premise is
markdown files in a folder. The `content/log/` directory is, structurally, a well with one
section. If the migration ever wants it, an ido well could become the source `build.rs`
reads from — no format change required, because there was never a format to change.
