# Contributing

Thanks for contributing to yxhoo-transit. Even small fixes are welcome.

## Getting Started

- For breaking changes or large feature additions, please discuss in an issue first.
- Keep PRs small and focused to speed up review.
- Because this crate targets an unofficial API, try to keep changes backward-compatible.

## Development Environment

- Rust stable
- rustfmt / clippy (`rustup component add rustfmt clippy`)

## Setup

```bash
# Clone
git clone <your-fork-url>
cd yxhoo-transit

# Build
cargo build
```

## Tests and Quality Checks

```bash
# Tests
cargo test

# Format
cargo fmt

# Lint (treat warnings as errors)
cargo clippy --all-targets -- -D warnings
cargo clippy --all-targets --no-default-features --features http-wreq -- -D warnings
```

## Feature Flags

This crate lets you switch the HTTP client via features. Exactly one must be enabled.

- Default: `http-reqwest`
- Alternative: `http-wreq`

```bash
# Default (reqwest)
cargo test

# Use wreq
cargo test --no-default-features --features http-wreq
```

## How to Contribute

1. Open an issue (if needed)
2. Create a branch
3. Make changes, run tests, update docs
4. Open a PR (include background, intent, and how you verified it)

## Documentation

- If you change the public API, update the usage examples in `README.md`.
- If behavior changes for exceptions or error types, add an explanation.

## License

Contributions follow the license in `Cargo.toml` (Apache-2.0).
