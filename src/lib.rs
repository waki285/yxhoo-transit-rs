//! # yxhoo-transit
//!
//! A Rust client for the unofficial Yxhoo! Transit (Japan) API.
//!
//! ## Feature flags
//! Exactly one HTTP client feature must be enabled.
//!
//! - `http-reqwest` (default)
//! - `http-wreq`
//! - `schemars`: Enable `JsonSchema` derives for public types.
//!
//! ```bash
//! # default (reqwest)
//! cargo test
//!
//! # wreq
//! cargo test --no-default-features --features http-wreq,schemars
//! ```
//!
//! ## Example
//! ```no_run
//! use yxhoo_transit::{suggest_places, transit, args::{TransitArgs, DateKind}};
//!
//! # #[tokio::main]
//! # async fn main() -> anyhow::Result<()> {
//! let suggestions = suggest_places("新宿").await?;
//! println!("{:?}", suggestions);
//!
//! let args = TransitArgs {
//!     from: "新宿".into(),
//!     to: "渋谷".into(),
//!     date: chrono::Local::now().into(),
//!     date_kind: DateKind::DepartureTime,
//!     criteria: None,
//!     rank: 1,
//!     options: None,
//! };
//! let result = transit(&args).await?;
//! println!("{:?}", result);
//! # Ok(())
//! # }
//! ```
//!
//! ## Notes
//! This crate uses an unofficial API and may break without notice.
pub mod args;
mod dt_minute_tz;
mod http;
pub mod transit;
mod yxhoo;

pub use yxhoo::{suggest_places, transit};
