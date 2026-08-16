# Show available commands
default:
    just --list

# Format all Rust and Leptos code
fmt:
    cargo fmt --all
    leptosfmt "src/**/*.rs"

# Check formatting without modifying files
fmt-check:
    cargo fmt --all --check
    leptosfmt --check "src/**/*.rs"

# Run normal Rust checks (clippy subsumes `cargo check`).
# Targets wasm32 — the thing we actually ship. `build.rs` is still linted, since cargo
# always compiles build scripts for the host.
check:
    cargo clippy --workspace --all-targets --target wasm32-unknown-unknown -- -D warnings

# Full local validation before pushing
verify:
    just fmt-check
    just check