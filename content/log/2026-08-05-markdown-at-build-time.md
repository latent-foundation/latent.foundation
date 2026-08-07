+++
title   = "Markdown at build time"
date    = "2026-08-05"
summary = "This site compiles its writing into the binary instead of parsing it in the browser. ido does the opposite, for a good reason."
tags    = ["rust", "leptos", "ido"]
draft   = true
+++

This site is a Leptos app compiled to WebAssembly. There is no server and no database —
the whole thing is a static bundle on a CDN. So when I added writing to it, the first
question was where markdown turns into HTML.

There are two honest answers, and the ecosystem now uses both.

## Two places a parse can live

**ido** parses markdown in the browser. It has to: it renders *your* files, live, as you
type, and a round trip to a backend for every keystroke would be felt. The parser is part
of the product.

This site parses markdown in `build.rs`. Its content is fixed the moment the binary is
built — the files are in the repo, reviewed in git, and cannot change between builds. A
parse that can only ever produce one answer belongs in the build.

| | build time | in the browser |
|---|---|---|
| bundle | parser never ships | ~150 KB of parser, every visit |
| bad frontmatter | build fails | renders broken in production |
| listing entries | metadata is already there | parse every file to build a list |

### What that buys

The third row is the one that changed my mind. An index page needs each entry's title,
date, and summary — but not its body. Parsing at runtime means reading every entry in
full just to render a list of links to them.

Doing it at build time, the list is a `static` array:

```rust
pub static LOG: &[Entry] = &[
    Entry {
        slug: "markdown-at-build-time",
        title: "Markdown at build time",
        reading_minutes: 3,
        html: include_str!("…/markdown-at-build-time.html"),
        // …
    },
];
```

The bodies sit in separate files pulled in by `include_str!` rather than inline string
literals, which sidesteps escaping arbitrary HTML into Rust source. `Entry` is `Copy`, so
it passes through reactive closures without a clone.

## Failing in the right place

The part I care about more than bundle size: a malformed entry is now a **build error**.

A missing title, a date that isn't `YYYY-MM-DD`, two files claiming the same slug — each
panics in `build.rs` with the offending path, which surfaces as a failed `just verify` and
a red CI run. None of them can reach a browser.

> The frontmatter fence is `+++`, not `---`. `---` is also a markdown thematic break, so a
> fence you forget to close parses as perfectly valid markdown and silently swallows the
> document. `+++` has no such second meaning.

## What it costs

- `build.rs` is a real file to maintain — about 150 lines.
- Publishing means recompiling. This is not a new constraint: the content is in the repo,
  so a rebuild was always going to happen.
- Drafts need handling. `draft = true` entries compile in debug and are dropped from
  release, so `trunk serve` shows them and the deployed bundle never contains them.

Both paths use the same parser, [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark),
pinned to the same version in both repos. The rendering rules stay one decision rather
than two that drift.

The full reasoning, and the plan the rest of this section was built from, is in
`docs/log-section.md`. The [projects archive](/projects) has more on ido itself.
