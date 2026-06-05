# Design system

latent. is typography-first, warm-neutral, and architecturally restrained. The guiding principle: **depth over noise** — borders over shadows, accent used sparingly, minimal interfaces.

---

## Fonts

All fonts are self-hosted as woff2 files in [`style/fonts/`](../style/fonts/). They are referenced in [`style/site.css`](../style/site.css) via `@font-face` rules and copied to `dist/fonts/` by Trunk at build time.

| Font | Role | Weights |
|------|------|---------|
| General Sans | Body, headings, UI text | 300, 400, 500, 600, 700, italic, variable |
| Geist Mono | Code, monospaced labels | 400, 500, 600, italic, variable |
| Geist | Utility / alternative sans | 300, 400, 500, 600, 700, italic, variable |

Only these 19 woff2 files are committed — unused weights and legacy `.ttf`/`.woff` formats were removed to keep the repo lean (~400 KB vs ~3.7 MB).

---

## Color tokens

All colors are CSS custom properties declared on `:root` (dark theme default) with a `[data-theme="light"]` override block. **Never hardcode hex values** — always use a token.

### Dark theme (default)

| Token | Value | Use |
|-------|-------|-----|
| `--color-bg` | `#11110f` | Page background |
| `--color-bg-subtle` | `#151512` | Slightly elevated surface |
| `--color-surface` | `#1a1a16` | Cards, panels |
| `--color-border` | `#2a2a24` | Dividers, outlines |
| `--color-text` | `#e8e4dc` | Primary text |
| `--color-text-muted` | `#8a8578` | Secondary / supporting text |
| `--color-text-faint` | `#4a4840` | Placeholder, disabled |
| `--color-primary` | `#c4a882` | Accent — links, active states |
| `--color-primary-subtle` | `#2a2318` | Accent background tint |

### Light theme

| Token | Value | Use |
|-------|-------|-----|
| `--color-bg` | `#f5f2ed` | Page background |
| `--color-bg-subtle` | `#f0ece5` | Slightly elevated surface |
| `--color-surface` | `#ebe6dd` | Cards, panels |
| `--color-border` | `#d4cdc2` | Dividers, outlines |
| `--color-text` | `#2a2520` | Primary text |
| `--color-text-muted` | `#6b6358` | Secondary / supporting text |
| `--color-text-faint` | `#b0a898` | Placeholder, disabled |
| `--color-primary` | `#8b6d45` | Accent — links, active states |
| `--color-primary-subtle` | `#ede5d8` | Accent background tint |

---

## Type scale

| Token | Value | Use |
|-------|-------|-----|
| `--font-body` | `"General Sans", system-ui, sans-serif` | All body text |
| `--font-mono` | `"Geist Mono", monospace` | Code, tags, labels |
| `--text-xs` | `0.75rem` | Micro labels (tags, kickers) |
| `--text-sm` | `0.875rem` | Secondary text, captions |
| `--text-base` | `1rem` | Body copy |
| `--text-lg` | `1.125rem` | Lead paragraphs |
| `--text-xl` | `1.25rem` | Section headings |
| `--text-2xl` | `1.5rem` | Subsection headings |
| `--text-3xl` | `1.875rem` | Page headings |
| `--text-4xl` | `2.25rem` | Hero headline |
| `--text-5xl` | `3rem` | Large hero (breakpoint-scaled) |
| `--leading-tight` | `1.2` | Headings |
| `--leading-normal` | `1.6` | Body copy |

---

## Spacing scale

All spacing uses a base-4 scale via CSS custom properties:

```
--space-1  0.25rem    --space-8   2rem
--space-2  0.5rem     --space-10  2.5rem
--space-3  0.75rem    --space-12  3rem
--space-4  1rem       --space-16  4rem
--space-5  1.25rem    --space-20  5rem
--space-6  1.5rem     --space-24  6rem
```

---

## Icons

Icons use [Lucide](https://lucide.dev) loaded via CDN — never hand-drawn SVGs or emoji.

```html
<i data-lucide="arrow-right"></i>
```

Call `lucide.createIcons()` after the icons are in the DOM. From Leptos, trigger via `window().eval("lucide.createIcons()")` after mount.

---

## Theming conventions

- `data-theme="dark"` or `data-theme="light"` is set on `<html>` — CSS custom properties cascade down from there
- The Leptos `RwSignal<bool>` (`true` = dark) is the source of truth at runtime
- `localStorage["latent-theme"]` persists the choice across page loads
- The inline script in `index.html` prevents FOUC by setting `data-theme` before WASM loads

---

## Component conventions

- **Borders over shadows** — prefer `1px solid var(--color-border)` over `box-shadow`
- **Accent sparingly** — `--color-primary` on interactive elements and key text only
- **No decorative emoji** — Lucide icons only
- **Monospace for data** — use `--font-mono` for tags, status labels, year numbers, and any code-like text
