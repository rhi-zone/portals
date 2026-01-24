//! # Pith
//!
//! A standard library of interfaces for Rust.
//!
//! Pith provides capability-based interfaces that enable portability across
//! platforms (native, WASM, embedded) while reducing decision fatigue in
//! contested ecosystem domains.
//!
//! ## Philosophy
//!
//! - **Portability over power** - simpler interfaces that work everywhere
//! - **Reduce decision fatigue** - blessed choices in contested domains
//! - **Consistent APIs** - uniform patterns across all portals crates
//!
//! ## Interface Categories
//!
//! ### Primitives (High Value)
//!
//! Fundamental capabilities with genuinely different implementations per platform:
//!
//! | Crate | Domain |
//! |-------|--------|
//! | [`portals-clocks`](https://docs.rs/portals-clocks) | Time and timestamps |
//! | [`portals-filesystem`](https://docs.rs/portals-filesystem) | File I/O |
//! | [`portals-io`](https://docs.rs/portals-io) | Streams and polling |
//! | [`portals-random`](https://docs.rs/portals-random) | Randomness |
//! | [`portals-sockets`](https://docs.rs/portals-sockets) | Raw networking |
//!
//! ### Contested Domains (Medium Value)
//!
//! Areas where the ecosystem has multiple viable options:
//!
//! | Crate | Domain | Ecosystem Alternatives |
//! |-------|--------|----------------------|
//! | [`portals-http`](https://docs.rs/portals-http) | HTTP client/server | reqwest, ureq, hyper |
//! | [`portals-sql`](https://docs.rs/portals-sql) | SQL databases | rusqlite, sqlx, diesel |
//! | [`portals-cache`](https://docs.rs/portals-cache) | Caching with TTL | moka, cached, etc. |
//! | [`portals-crypto`](https://docs.rs/portals-crypto) | Cryptography | ring, rustcrypto |
//! | [`portals-logging`](https://docs.rs/portals-logging) | Logging | log, tracing |
//! | [`portals-markdown`](https://docs.rs/portals-markdown) | Markdown | pulldown-cmark, comrak |
//! | [`portals-config`](https://docs.rs/portals-config) | Configuration | figment, config |
//! | [`portals-websocket`](https://docs.rs/portals-websocket) | WebSocket | tungstenite, etc. |
//!
//! ## Solved Domains (Use Directly)
//!
//! For these domains, the Rust ecosystem has clear winners.
//! **Don't use portals wrappers - use these directly:**
//!
//! | Domain | Recommended Crate | Why |
//! |--------|------------------|-----|
//! | Serialization | [`serde`](https://docs.rs/serde) | Universal standard |
//! | JSON | [`serde_json`](https://docs.rs/serde_json) | De facto standard |
//! | CLI parsing | [`clap`](https://docs.rs/clap) | Dominant, excellent DX |
//! | URL parsing | [`url`](https://docs.rs/url) | WHATWG compliant |
//! | UUID | [`uuid`](https://docs.rs/uuid) | Full RFC 4122 support |
//! | Regex | [`regex`](https://docs.rs/regex) | Fast, safe, dominant |
//! | Async runtime | [`tokio`](https://docs.rs/tokio) | Ecosystem standard |
//! | Multi-pattern matching | [`aho-corasick`](https://docs.rs/aho-corasick) | Used by regex internally |
//! | Library error types | [`thiserror`](https://docs.rs/thiserror) | Dominant derive macro |
//! | App error handling | [`anyhow`](https://docs.rs/anyhow) | Dominant for apps |
//! | Incremental parsing | [`tree-sitter`](https://docs.rs/tree-sitter) | Dominant in tooling |
//!
//! Creating portals wrappers for these would add friction without benefit.
//!
//! ## What Pith Is Not
//!
//! Pith does not try to:
//! - **Wrap solved domains** - use serde, clap, regex directly
//! - **Abstract stylistic choices** - error handling, parser combinators
//! - **Replace the ecosystem** - we complement it, not compete
//! - **Be a framework** - portals is à la carte
//!
//! ## Usage
//!
//! Add the specific portals crates you need:
//!
//! ```toml
//! [dependencies]
//! portals-filesystem = "0.1"
//! portals-http = "0.1"
//! portals-clocks = "0.1"
//!
//! # Native backends
//! portals-filesystem-native = "0.1"
//! portals-http-native = "0.1"
//! portals-clocks-native = "0.1"
//! ```
//!
//! ## Backends
//!
//! Each interface has one or more backend implementations:
//!
//! - `*-native` - Native OS implementation
//! - `*-wasm` - WebAssembly implementation (partial)
//! - `*-mock` - Testing mocks
//!
//! ## Links
//!
//! - [GitHub Repository](https://github.com/rhi-zone/portals)
//! - [Design Guidelines](https://github.com/rhi-zone/portals/blob/main/DESIGN.md)

#![no_std]

// This crate is documentation-only.
// Use the specific portals-* crates for functionality.
