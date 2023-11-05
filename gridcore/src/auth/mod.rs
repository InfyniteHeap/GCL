//! # Microsoft OAuth2
//!
//! The module that is used to Minecraft genius verification.
//!
//! Because Mojang had already deprecated Mojang account verification method, this module only supports Microsoft OAuth2.
//!
//! ### Example
//!
//! This example will roughly show you how to complete the verification process.
//!
//! Before starting, it's recommended to add `tokio` crate in `Cargo.toml` with `rt-multi-thread` feture enabled:
//!
//! ```toml
//! [dependencies]
//! tokio = { version = "1.33.0", features = ["rt-multi-thread"] }
//! ```
//!
//! Then add structure `tokio::runtime::Runtime` in which you will use it:
//!
//! ```rust
//! use tokio::runtime::Runtime;
//! ```
//!
//! Now we can start the process of genius verification:

pub mod microsoft_oauth2;
