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

# Run normal Rust checks
check:
    cargo check --workspace
    cargo clippy --workspace --all-targets -- -D warnings

# Full local validation before pushing
verify:
    just fmt-check
    just check