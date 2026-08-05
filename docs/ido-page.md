# `/ido` — the product page

> Design + implementation plan. A comprehensive page for **ido** (井戸) — what it is, how
> a well is laid out, what it does today, and where it is going — at its own top-level URL.

Companion plan: [log-section.md](log-section.md). §7 here reuses that content pipeline.

---

## 1. Why this page exists

The `/projects/ido` detail page is an *archive entry*: two paragraphs, tags, a year. ido is
now a real, substantial desktop app — notes, a linked wiki, a kanban board with goals,
calendar and table views, split panes, a command palette — with an end-to-end-specced next
phase. That does not fit an archive row.

So: `/projects` stays the archive, and ido gets a product page. The two do not compete.

**Everything on this page must be traceable** to ido's `README.md`, `CLAUDE.md`, or
`docs/mcp-server.md`. No claim about an unshipped feature stated in the present tense; no
marketing verbs. Shipped and planned are visually distinct, always. This is a brand
requirement, not a stylistic preference — *quiet confidence, never overclaim*.

---

## 2. Routing and how the page is reached

```rust
<Route path=path!("ido") view=IdoPage />
```

Top-level, no nav-bar entry. Adding `ido` to the header would elevate one project above the
lab framing the site is built on; the page is reached from where people already look for it.

**Reached from** — a new optional field on `Project` rather than a special case in a view:

```rust
// src/data.rs
pub struct Project {
    …
    /// A bespoke product page for this project, if it has one.
    pub page: Option<&'static str>,   // ido: Some("/ido")
}
```

- `ProjectRow` links to `project.page` when set, otherwise `/projects/{id}` — so the ido
  row on both `/` and `/projects` goes straight to the product page.
- `ProjectDetail` renders a prominent link to `page` when set, so `/projects/ido` stays a
  valid URL and leads onward rather than dead-ending.

Data-driven, so `logos` gets the same treatment later by setting one field.

---

## 3. Page structure

Eight sections, hairline-separated, one idea each. Wider than the 720px reading column —
`--container` (1080px) — because the feature blocks are two-column.

### 3.1 Hero

井戸 splash · `ido` · `ACTIVE · 2026` meta · the positioning line **"a quiet well for
thought"** · a two-sentence lede · a quiet action row.

Reuses `.project-detail-hero` structure so it reads as the same family as the archive
pages, at larger scale.

The action row carries a **source** link to `github.com/latent-foundation/ido` — verified
public (an anonymous request returns 200; a private repo would 404).

> Note: `vendor/latent-design/docs/ecosystem.md` still describes ido as *"private, not yet
> built"*. That is stale on both counts. Correcting it is a PR against `latent-design`,
> not this repo.

### 3.2 What it is — the three-section model

The page's anchor visual: an inline SVG of the rail and the three sections it switches
between.

```
        ┌──────────────────────────────────────────┐
  rail  │  notes    foldered markdown, 3 editor    │
   ▪    │           modes: source · live · reading │
   ▪    │  wiki     a flat slug namespace,         │
   ▪    │           [[linked]], with backlinks     │
        │  tasks    kanban + goals, calendar,      │
        │           table                          │
        └──────────────────────────────────────────┘
```

Prose beside it carries the actual information (the SVG is `aria-hidden`, §5.3).

### 3.3 The well — ido's core promise

A folder-tree SVG of a well on disk, beside the argument for it:

```
my-well/
├── notes/     foldered markdown
├── wiki/      [[linked]] pages, globally unique slugs
├── tasks/     one .md per task, frontmatter metadata
│   └── goals/ milestones with target dates
├── assets/    pasted images
└── .ido/      config + cache — rebuildable, deletable
```

The point being made: **a well is any folder you pick**, everything in it is plain
markdown you can read without ido, and the only ido-specific directory is rebuildable.
No lock-in, no latency, no account. This is the single most important idea on the page and
earns the most vertical space.

### 3.4 Feature tour

Four blocks. Each: mono kicker · heading · 2–3 sentences · a UI miniature (§5.2).
Alternating which side the miniature sits on; stacking below 900px.

| Block | Says | Miniature |
|---|---|---|
| **notes** | foldered markdown; three editor modes — source (autosaves per keystroke), live (block preview, click a block to edit), reading | mode switch + a live block mid-edit |
| **wiki** | `[[wikilinks]]`, created on click when they don't exist; slugs globally unique so moving a page between folders never breaks a link; backlinks span **every** section | a page + its "linked from" panel |
| **tasks** | kanban with quick capture, drag to a precise position, due dates that flag overdue/due-today, checklist rollups, recurrence, goals with progress; board ⇄ calendar ⇄ table | kanban columns with cards, insertion line, `3/7` rollup, due chip |
| **across everything** | tabs (preview vs pinned), split panes, `Ctrl+K` palette searching all three sections, soft-delete with undo | tab strip + split divider |

### 3.5 Roadmap — the big arcs

Not a changelog. Four horizons plus what shipped, each with one sentence of substance.
Grounded in ido's own docs — sources noted below, and the page must not outrun them.

| State | Arc | Substance | Source |
|---|---|---|---|
| `shipped` | the workspace | notes, wiki, tasks, goals, calendar, table, tabs + split, `Ctrl+K` search, undo, images, math | ido `CLAUDE.md` §Current state |
| `next` | **MCP server** | extract the store into a tauri-free `ido-store` crate, then an `rmcp` + stdio binary with seven read-only tools — any MCP client can read and search the well without ido running | `docs/mcp-server.md` P0–P1 |
| `planned` | **semantic search** | local embeddings via candle (pure Rust, no ONNX runtime to bundle), brute-force cosine fused with the existing lexical scan; the same index powers in-app `Ctrl+K` | `docs/mcp-server.md` P2–P3 |
| `planned` | **on-device AI** | a local model answering over your own notes — private, offline, reusing the candle groundwork the index already lays down | ido `CLAUDE.md` §What's next |
| `exploring` | **sync** | self-hosted first, optional cloud, end-to-end encrypted — local-first stays the default, sync is opt-in | ido `CLAUDE.md` §What's next |
| `exploring` | **mobile** | reading and capture on a phone against the same well | direction, not yet in ido's docs |

**Mobile is the one arc not backed by ido's own docs** — a direction rather than a
commitment. It stays on the page under `exploring`, which is precisely why that state
exists: it lets a real intention be shown without being promised. If it later gains a spec
in ido's repo, it moves up to `planned` like the others.

**Version numbers stay off the public page.** ido targets the workspace at 1.0.0 and the
MCP server at 1.1.0 (§10), which is useful for sequencing our own work — but a version on a
public roadmap reads as a date, and ido has not shipped 1.0.0 yet. The `shipped` / `next` /
`planned` / `exploring` states carry the same ordering without committing to a calendar.
Revisit once 1.0.0 is actually out.

Closing the section: a short **deliberate non-goals** line — notes are not `[[link]]`
targets; nothing leaves the machine; no telemetry; not a productivity dashboard. Saying
what a thing refuses to be is more informative than another feature bullet, and it is very
much the house voice.

### 3.6 Principles

Three or four short statements on the existing `.fact-row` grid from `/about` — local-first,
plain files, calm over dashboards, durability. Reuse, not new markup.

### 3.7 Guides *(later phase — §7)*

`/ido/:slug`, fed by `content/ido/*.md` through the log pipeline.

### 3.8 Closing

Honest status ("in active development, not yet released"), a link to log entries tagged
`ido`, and the repo link if §3.1's open item resolves.

---

## 4. Screenshots — one, in the hero

Following up on your second thought: **yes, but exactly one.**

A product page for a real desktop app should show the app. But a screenshot earns its place
where it answers *"what does this actually look like"* — which is a question you answer
once, at the top. Inside the feature tour, a screenshot of the notes pane is a picture of
text: it explains less than two sentences and a diagram, weighs more, and goes stale the
moment the UI moves. So: one hero image, structure below it.

**Capture spec** — the full workspace, well open, tasks board visible (the most visually
distinctive section), realistic but unremarkable content, no personal data:

```
assets/ido/workspace-dark.png     ~2400px wide, 2× for retina
assets/ido/workspace-light.png    same frame, light theme
```

**Wiring** — `index.html` gains one line, `<link data-trunk rel="copy-dir" href="assets" />`
(this repo has no local `assets/` yet; `vendor/latent-design/assets` is separate and already
copied). Theme swap without JS:

```html
<img src="/assets/ido/workspace-dark.png"  class="ido-shot ido-shot-dark"  alt="…" />
<img src="/assets/ido/workspace-light.png" class="ido-shot ido-shot-light" alt="…" />
```

with `[data-theme="dark"] .ido-shot-light { display: none }` and the inverse — the theme
attribute is already on `<html>` before first paint, so there is no flash. Frame it in a
`1px solid var(--color-border-subtle)` container with `--radius-md`; no drop shadow, no
perspective mock-up, no floating device frame. Borders over shadows.

**The page must ship complete without it.** The hero renders correctly with the image
absent, so this never blocks the rest of the work — drop the files in when you capture
them and the slot fills.

---

## 5. The visual system

Three kinds of visual, each with a different job. All theme-aware, all zero-asset except
the hero shot.

### 5.1 Diagrams — inline SVG

For structure: the three-section model (§3.2), the well on disk (§3.3), optionally the
MCP shape in the roadmap.

- Strokes `currentColor` at `stroke-width="1.5"` (the icon canon's weight), `fill="none"`;
  labels `fill="currentColor"` in `--font-mono` at 11–12px.
- The wrapper sets `color: var(--color-text-muted)`; elements needing emphasis carry
  `class="accent"` → `.ido-diagram .accent { color: var(--color-accent) }`. Both themes are
  handled by the tokens with zero extra rules.
- `viewBox` + `width: 100%; height: auto` for responsiveness; `role="img"` with a `<title>`
  child for screen readers.
- One Leptos component per diagram, in `src/views/ido/diagrams.rs`.

### 5.2 Miniatures — divs + tokens

For interaction shapes a static diagram can't convey: the kanban board, the tab strip, the
mode switch, the backlinks panel. Built from real markup and design tokens, so they theme
for free and cost nothing to download.

**The honesty rule:** miniatures are re-implementations and *will* drift from the real app.
So they stay schematic — no invented features, no fake data implying capability the app
lacks — and they are `aria-hidden="true"` with the surrounding prose carrying every piece
of information. A reader who never sees them loses nothing.

### 5.3 The roadmap timeline

A vertical track: state marker · arc title · one sentence · optional sub-bullets. Markers
are `latent_ui::Icon` glyphs, not colored dots — `square-check` for shipped, `chevron-right`
for next, `circle` states for planned/exploring. Shipped rows sit at
`--color-text-secondary`; unshipped at `--color-text-muted` with the state label in mono
uppercase. The visual distinction between built and planned is the whole point of the
section.

**Icons:** use only names already in `latent-ui`'s table — verified present:
`square-kanban`, `file-text`, `book`, `search`, `target`, `calendar-days`, `columns-2`,
`folder`, `code`, `repeat`, `square-check`, `settings`, `table`, `bookmark`, `clock`,
`chevron-right`. If the page needs a glyph that isn't there, it gets added **upstream to
`latent-ui`**, never inlined here — an unknown name renders an invisible SVG rather than
failing, so a missing glyph is easy to ship by accident.

---

## 6. Code layout

```
src/views/ido/
  mod.rs          IdoPage — composes the sections, owns nothing else
  diagrams.rs     SectionsDiagram, WellDiagram   (inline SVG)
  miniatures.rs   BoardMini, EditorMini, WikiMini, TabsMini
  roadmap.rs      Phase / PhaseState data + the timeline component
```

A directory rather than one file: `IdoPage` as a single module would be well over 600 lines
of `view!`, which nothing in this repo resembles. Roadmap content is a `static [Phase]` in
`roadmap.rs`, following the same "static data compiled into the binary" convention as
`data.rs` — the copy is data, not markup.

**Styles:** one `=== IDO PAGE ===` block in `style/app.css`, ~300 lines, all tokens.
Namespaced `.ido-*` so nothing leaks into the archive pages. Nothing goes in the submodule:
this is app-specific layout, and `style/app.css` is this repo's only stylesheet.

---

## 7. Guides — a later phase

`/ido/:slug` over `content/ido/*.md`, through [log-section.md](log-section.md)'s pipeline
unchanged. This is why that pipeline is built around a *collection* concept and lives in
`src/content.rs` rather than being log-specific — `build.rs` iterates a list of collections,
and adding `content/ido/` is a one-line change plus a route.

Deferred because ido isn't released: the useful guides ("open your first well", "how
wikilinks resolve") are written against an app people can install. The structure is ready
when it is.

---

## 8. Work order

| # | Step | Done when |
|---|---|---|
| 1 | `page: Option<&'static str>` on `Project`; `ProjectRow` + `ProjectDetail` honour it | ido rows link to `/ido`; `/projects/ido` still resolves and links onward |
| 2 | `/ido` route + `src/views/ido/mod.rs` skeleton: hero, all section headers, real prose, no visuals | page reads completely as text in both themes; every claim traceable to an ido doc |
| 3 | Roadmap data + timeline (§3.5, §5.3) | shipped vs planned unmistakable at a glance; mobile framing confirmed |
| 4 | `.ido-*` styles in `style/app.css` | responsive at 360 / 768 / 1080+; no horizontal scroll |
| 5 | Diagrams (§5.1) | both themes; legible at 360px; `<title>` present |
| 6 | Miniatures (§5.2) | `aria-hidden`; prose still complete with them hidden |
| 7 | Hero screenshot slot (§4) | renders correctly **with and without** the image files |
| 8 | Docs: `CLAUDE.md` structure block, `docs/architecture.md` routes table | reflect `/ido` and `src/views/ido/` |

`just verify` green at every step. Format with `just fmt` — never `cargo fmt` alone; this
page is almost entirely `view!` macros, which only `leptosfmt` can format.

---

## 9. Open items

1. **Hero screenshot** — you capture, I wire (§4). The page ships complete without it, so
   this blocks nothing.

*Resolved: ido's repo is public, so the source link ships (§3.1). Mobile stays on the
roadmap under `exploring` (§3.5).*

---

## 10. Where this doc lives

Interim home. The ecosystem's documentation moves into ido itself — these plans, the
`latent-design/docs/` canon, and the per-app design docs become linked markdown in a well,
served over ido's MCP server alongside live state.

**The migration happens in the window between ido 1.0.0 and 1.1.0**, because MCP arrives in
1.1.0. That ordering is the point: the canon must already be *in* the well when the server
ships, so 1.1.0 has a real body of knowledge to serve on day one rather than an empty store
and a follow-up task. The migration is a prerequisite for MCP being useful, not a
consequence of it.

`vendor/latent-design/docs/knowledge-architecture.md` already describes this convergence as
step 5 of its sequencing, but marks it *"aspirational, not now"* on the grounds that ido
didn't exist yet. It does now, and the window above is a real condition — worth recording
there so the plan carries a schedule rather than a hope. That is a PR against
`latent-design`.

Two consequences for how these docs are written, starting now:

- **Portable markdown only** — no HTML, no repo-relative link tricks beyond plain relative
  paths, so a migration is a file move rather than a rewrite.
- **Design docs, not status reports** — anything that would go stale between here and 1.0.0
  (branch names, in-flight work) stays in git, not in prose.
