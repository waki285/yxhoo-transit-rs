# yxhoo-transit

[![crates.io](https://img.shields.io/crates/v/yxhoo-transit.svg)](https://crates.io/crates/yxhoo-transit)
[![docs.rs](https://img.shields.io/docsrs/yxhoo-transit.svg)](https://docs.rs/yxhoo-transit)
[![downloads](https://img.shields.io/crates/dv/yxhoo-transit.svg)](https://crates.io/crates/yxhoo-transit)
[![license](https://img.shields.io/crates/l/yxhoo-transit.svg)](https://crates.io/crates/yxhoo-transit)
[![CI](https://img.shields.io/github/actions/workflow/status/waki285/yxhoo-transit/ci.yml?branch=main)](https://github.com/waki285/yxhoo-transit/actions/workflows/ci.yml)

A Rust client for Yxhoo! Transit (Japan) unofficial API.
This crate provides functions to suggest places and search for transit routes using Yxhoo! Transit.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
yxhoo-transit = "0.1.0"
```

## Features

- `http-reqwest` (default): Use [reqwest](https://docs.rs/reqwest/latest/reqwest/) as the HTTP client.
- `http-wreq`: Use [wreq](https://docs.rs/wreq/latest/wreq/) as the HTTP client.

## Example

```rust
use yxhoo_transit::{suggest_places, transit, TransitArgs, DateKind};

#[tokio::main]
async fn main() {
    // Suggest places
    let suggestions = suggest_places("新宿").await.unwrap();
    println!("{:?}", suggestions);

    // Transit search
    let args = TransitArgs {
        from: "新宿".into(),
        to: "渋谷".into(),
        date: chrono::Local::now().into(),
        date_kind: DateKind::DepartureTime,
        criteria: None,
        rank: 1,
        options: None,
    };
    let result = transit(&args).await.unwrap();
    println!("{:?}", result);
}
```

## License

Apache-2.0
