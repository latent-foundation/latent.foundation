//! Build-time content pipeline: `content/**/*.md` → HTML + metadata, compiled in.
//!
//! ido parses markdown in the browser because it renders *the user's* files as they
//! type. This site's content is fixed the moment the binary is built, so the parse
//! belongs here instead: the parser never reaches the WASM bundle, the index page gets
//! entry metadata without parsing every body to build a list, and a malformed entry
//! fails `just verify` rather than rendering broken in production.
//!
//! Two artefacts land in `OUT_DIR`:
//! - `html/<collection>/<slug>.html` — one rendered body per entry, pulled in by
//!   `include_str!`. Bodies live in files rather than inline string literals so arbitrary
//!   HTML never has to be escaped into Rust source.
//! - `entries.rs` — the `static` arrays, `include!`d by `src/content.rs`.
//!
//! Generated code is deliberately trivial (plain struct literals, no expressions):
//! `rustfmt` and `leptosfmt` never see `OUT_DIR`, but **clippy does**.

use std::{
    collections::HashMap,
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, TagEnd, html};

/// A directory under `content/` and the `static` it generates in `src/content.rs`.
///
/// Adding a collection is one line here plus a route — which is how `content/ido/`
/// (the ido guides) arrives later without touching this pipeline.
struct Collection {
    dir: &'static str,
    static_name: &'static str,
}

const COLLECTIONS: &[Collection] = &[Collection {
    dir: "log",
    static_name: "LOG",
}];

/// Words per minute used for the reading-time estimate. Deliberately on the slow side —
/// this is technical prose, not a news feed.
const WORDS_PER_MINUTE: usize = 220;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR set by cargo"));
    // Draft entries compile in debug so `trunk serve` shows them, and are dropped from
    // release builds — a draft body never reaches the deployed bundle.
    let include_drafts = env::var("PROFILE").as_deref() == Ok("debug");

    // Catches added and deleted files (the directory's mtime moves); the per-file
    // directives below catch edits to existing ones.
    println!("cargo:rerun-if-changed=content");

    let mut generated = String::new();

    for collection in COLLECTIONS {
        let dir = Path::new("content").join(collection.dir);
        let html_dir = out_dir.join("html").join(collection.dir);
        // Cleared first so a renamed or deleted entry can't leave a stale body behind.
        let _ = fs::remove_dir_all(&html_dir);
        fs::create_dir_all(&html_dir).expect("create OUT_DIR html directory");

        let mut entries = Vec::new();
        let mut seen: HashMap<String, PathBuf> = HashMap::new();
        for path in markdown_files(&dir) {
            println!("cargo:rerun-if-changed={}", path.display());
            let entry = parse_entry(&path);
            if entry.draft && !include_drafts {
                continue;
            }
            // Before writing: two entries sharing a slug would otherwise have the second
            // silently overwrite the first's body on disk.
            if let Some(first) = seen.insert(entry.slug.clone(), path.clone()) {
                panic!(
                    "content/{}: two entries resolve to the slug {:?} — {} and {}",
                    collection.dir,
                    entry.slug,
                    first.display(),
                    path.display(),
                );
            }
            let html_path = html_dir.join(format!("{}.html", entry.slug));
            fs::write(&html_path, &entry.html).expect("write rendered body");
            entries.push((entry, html_path));
        }

        // Newest first. ISO dates sort lexicographically, so this needs no date crate.
        entries.sort_by(|(a, _), (b, _)| b.date.cmp(&a.date).then(a.slug.cmp(&b.slug)));

        emit_collection(&mut generated, collection.static_name, &entries);
    }

    fs::write(out_dir.join("entries.rs"), generated).expect("write generated entries.rs");
}

/// Every `.md` file directly inside `dir`, sorted for a deterministic build.
///
/// A missing directory yields nothing rather than failing: a collection with no entries
/// yet is a legitimate state, and an empty `static` is perfectly valid.
fn markdown_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(read_dir) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths: Vec<PathBuf> = read_dir
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    paths.sort();
    paths
}

/// One parsed entry, owned — the shape mirrors `content::Entry` on the Rust side.
struct Entry {
    slug: String,
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
    draft: bool,
    reading_minutes: usize,
    headings: Vec<(u8, String, String)>,
    html: String,
}

/// Read a file, split its frontmatter, render its body.
///
/// Every failure here is a `panic!` with the offending path, which cargo surfaces as a
/// build error. That is the point: a broken entry must not reach a browser.
fn parse_entry(path: &Path) -> Entry {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let (frontmatter, body) = split_frontmatter(&raw, path);

    let table: toml::Table = frontmatter
        .parse()
        .unwrap_or_else(|e| panic!("{}: invalid TOML frontmatter — {e}", path.display()));

    let required_str = |key: &str| -> String {
        table
            .get(key)
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "{}: frontmatter is missing the required string key `{key}`",
                    path.display()
                )
            })
            .to_string()
    };

    let title = required_str("title");
    let date = required_str("date");
    let summary = required_str("summary");
    check_date(&date, path);

    let tags = match table.get("tags") {
        None => Vec::new(),
        Some(value) => value
            .as_array()
            .unwrap_or_else(|| panic!("{}: `tags` must be an array of strings", path.display()))
            .iter()
            .map(|t| {
                t.as_str()
                    .unwrap_or_else(|| {
                        panic!("{}: `tags` must be an array of strings", path.display())
                    })
                    .to_string()
            })
            .collect(),
    };

    let draft = table
        .get("draft")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let (html, headings) = render(body);

    Entry {
        slug: slug_from_path(path),
        title,
        date,
        summary,
        tags,
        draft,
        reading_minutes: (body.split_whitespace().count() / WORDS_PER_MINUTE).max(1),
        headings,
        html,
    }
}

/// Split `+++`-fenced TOML frontmatter from the markdown body.
///
/// `+++` rather than `---` because `---` is also a markdown thematic break: with `---`
/// fences, a missing closing fence parses as valid markdown and silently swallows the
/// whole document instead of failing.
fn split_frontmatter<'a>(raw: &'a str, path: &Path) -> (&'a str, &'a str) {
    let rest = raw
        .strip_prefix("+++\n")
        .or_else(|| raw.strip_prefix("+++\r\n"))
        .unwrap_or_else(|| {
            panic!(
                "{}: must start with a `+++` TOML frontmatter fence",
                path.display()
            )
        });
    let end = rest.find("\n+++").unwrap_or_else(|| {
        panic!(
            "{}: frontmatter fence opened but never closed with `+++`",
            path.display()
        )
    });
    let body = rest[end + 4..].trim_start_matches(['\r', '\n']);
    (rest[..end].trim_end_matches('\r'), body)
}

/// The URL slug: the file stem minus its `YYYY-MM-DD-` prefix.
///
/// The date prefix keeps `content/log/` sorted on disk without putting the date in the
/// URL, so an entry can be re-dated without breaking its link.
fn slug_from_path(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| panic!("{}: non-UTF-8 filename", path.display()));

    let bytes = stem.as_bytes();
    let dated = bytes.len() > 11
        && bytes[..10].iter().enumerate().all(|(i, b)| {
            if i == 4 || i == 7 {
                *b == b'-'
            } else {
                b.is_ascii_digit()
            }
        })
        && bytes[10] == b'-';

    if dated {
        stem[11..].to_string()
    } else {
        stem.to_string()
    }
}

/// Reject anything that isn't `YYYY-MM-DD`, since ordering relies on it sorting as text.
fn check_date(date: &str, path: &Path) {
    let ok = date.len() == 10
        && date.as_bytes().iter().enumerate().all(|(i, b)| {
            if i == 4 || i == 7 {
                *b == b'-'
            } else {
                b.is_ascii_digit()
            }
        });
    if !ok {
        panic!(
            "{}: `date` must be YYYY-MM-DD, got {date:?}",
            path.display()
        );
    }
}

/// CommonMark plus the extensions technical prose actually uses.
///
/// Math is deliberately absent for now: it would pull in `latex2mathml` and no entry
/// needs it yet. Adding it later is this function plus one build-dependency.
fn options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_TASKLISTS
}

/// Render markdown to HTML, returning the body and its `(level, id, text)` headings.
///
/// The event stream is collected up front so headings can be rewritten with an id that
/// depends on text arriving *after* the opening tag — pulldown-cmark only populates
/// `Tag::Heading { id }` from explicit `{#id}` attributes, so auto-anchors are ours.
fn render(body: &str) -> (String, Vec<(u8, String, String)>) {
    let events: Vec<Event> = Parser::new_ext(body, options()).collect();
    let mut out = Vec::with_capacity(events.len());
    let mut headings = Vec::new();
    let mut used: HashMap<String, usize> = HashMap::new();

    let mut i = 0;
    while i < events.len() {
        let Event::Start(Tag::Heading {
            level,
            id: None,
            ref classes,
            ref attrs,
        }) = events[i]
        else {
            out.push(events[i].clone());
            i += 1;
            continue;
        };

        let end = events[i + 1..]
            .iter()
            .position(|e| matches!(e, Event::End(TagEnd::Heading(_))))
            .map(|offset| i + 1 + offset)
            .unwrap_or(events.len() - 1);

        let text: String = events[i + 1..end]
            .iter()
            .filter_map(|e| match e {
                Event::Text(t) | Event::Code(t) => Some(t.as_ref()),
                _ => None,
            })
            .collect();

        let id = unique_id(slugify(&text), &mut used);
        // h2/h3 only: the TOC is a map of the entry, not an outline of every subsection.
        let depth = level as u8;
        if (2..=3).contains(&depth) {
            headings.push((depth, id.clone(), text));
        }

        out.push(Event::Start(Tag::Heading {
            level,
            id: Some(CowStr::from(id)),
            classes: classes.clone(),
            attrs: attrs.clone(),
        }));
        i += 1;
    }

    let mut html_out = String::with_capacity(body.len() * 3 / 2);
    html::push_html(&mut html_out, out.into_iter());
    (external_links_open_in_new_tab(&html_out), headings)
}

/// Mark external links `target="_blank"`, leaving internal ones to the router.
///
/// Done as a string pass rather than in the event stream because pulldown-cmark's HTML
/// writer has no hook for extra anchor attributes — rewriting `Tag::Link` would mean
/// emitting raw HTML for every link and giving up its escaping.
fn external_links_open_in_new_tab(html: &str) -> String {
    html.replace(
        "<a href=\"http",
        "<a target=\"_blank\" rel=\"noopener noreferrer\" href=\"http",
    )
}

/// Lowercase, runs of non-alphanumerics collapse to `-`, ends trimmed.
///
/// Character-for-character the algorithm in ido's `markdown::slugify`, so a heading
/// anchor here and a wiki slug there are spelled the same way.
fn slugify(text: &str) -> String {
    let mut out = String::new();
    for ch in text.trim().chars() {
        if ch.is_alphanumeric() {
            out.extend(ch.to_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Disambiguate repeated headings (`overview`, `overview-2`, …) so anchors stay unique.
fn unique_id(base: String, used: &mut HashMap<String, usize>) -> String {
    let base = if base.is_empty() {
        "section".to_string()
    } else {
        base
    };
    let count = used.entry(base.clone()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}-{count}")
    }
}

/// Write one `pub static NAME: &[Entry]` array into the generated source.
fn emit_collection(out: &mut String, name: &str, entries: &[(Entry, PathBuf)]) {
    writeln!(out, "pub static {name}: &[Entry] = &[").expect("write to String");
    for (entry, html_path) in entries {
        writeln!(out, "    Entry {{").expect("write to String");
        // `{:?}` on a &str emits a correctly escaped Rust string literal for free.
        writeln!(out, "        slug: {:?},", entry.slug).expect("write to String");
        writeln!(out, "        title: {:?},", entry.title).expect("write to String");
        writeln!(out, "        date: {:?},", entry.date).expect("write to String");
        writeln!(out, "        summary: {:?},", entry.summary).expect("write to String");
        write!(out, "        tags: &[").expect("write to String");
        for tag in &entry.tags {
            write!(out, "{tag:?}, ").expect("write to String");
        }
        writeln!(out, "],").expect("write to String");
        writeln!(out, "        reading_minutes: {},", entry.reading_minutes)
            .expect("write to String");
        write!(out, "        headings: &[").expect("write to String");
        for (level, id, text) in &entry.headings {
            write!(
                out,
                "Heading {{ level: {level}, id: {id:?}, text: {text:?} }}, "
            )
            .expect("write to String");
        }
        writeln!(out, "],").expect("write to String");
        writeln!(
            out,
            "        html: include_str!({:?}),",
            html_path.to_str().expect("UTF-8 OUT_DIR path")
        )
        .expect("write to String");
        writeln!(out, "    }},").expect("write to String");
    }
    writeln!(out, "];").expect("write to String");
}
