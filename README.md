# latent.

quiet systems / hidden structure

`latent.` is a personal engineering lab for software projects, research tooling, and long-term experiments.

This repository contains the source code for [latent.foundation](https://latent.foundation), written in **Rust** with **Leptos**.

---

## projects

### ido

A work-in-progress notes, wiki, tasks, and goals app.

Local-first. Markdown-based. Built around depth, memory, and quiet organization.

### logos

A work-in-progress platform for market data, backtesting, and algorithmic trading research.

Rust-first. Experimental. Research-oriented.

---

## principles

- depth over noise
- engineering as craft
- minimal interfaces
- readable systems
- strong foundations

---

## development

**Prerequisites**

- [Rust](https://rustup.rs) (stable toolchain)
- [just](https://github.com/casey/just) — `cargo install just --locked`
- [leptosfmt](https://github.com/bram209/leptosfmt) — `cargo install leptosfmt --locked`
- [Trunk](https://trunkrs.dev) — `cargo install trunk --locked`

**Setup**

```sh
git clone https://github.com/latent-foundation/latent.foundation
cd latent.foundation
git config core.hooksPath .githooks
```

The last line activates the pre-commit formatting hook. It must be run once per clone.

**Commands**

```sh
just          # list available commands
just fmt      # format all Rust and Leptos code
just verify   # check formatting + run clippy (runs in CI)
trunk serve   # start local dev server
```

---

[latent.foundation](https://latent.foundation)