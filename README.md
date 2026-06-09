# latent.foundation

Source code for [latent.foundation](https://latent.foundation) — built with Rust and [Leptos](https://leptos.dev) (CSR/WASM), bundled by [Trunk](https://trunkrs.dev), deployed to Cloudflare Pages.

## prerequisites

- [Rust](https://rustup.rs) stable toolchain + `wasm32-unknown-unknown` target
- [`just`](https://github.com/casey/just) — `cargo install just --locked`
- [`leptosfmt`](https://github.com/bram209/leptosfmt) — `cargo install leptosfmt --locked`
- [`trunk`](https://trunkrs.dev) — `cargo install trunk --locked`

## setup

```sh
git clone https://github.com/latent-foundation/latent.foundation
cd latent.foundation
git config core.hooksPath .githooks
```

The last line activates the pre-commit formatting hook. Run it once per clone.

## commands

```sh
just fmt      # format (cargo fmt + leptosfmt)
just verify   # fmt-check + clippy — runs in CI
trunk serve   # local dev server with hot reload
trunk build --release  # production build → dist/
```
