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
//! - **Consistent APIs** - uniform patterns across all pith crates
//!
//! ## Interface Categories
//!
//! ### Primitives (High Value)
//!
//! Fundamental capabilities with genuinely different implementations per platform:
//!
//! | Crate | Domain |
//! |-------|--------|
//! | [`pith-clocks`](https://docs.rs/pith-clocks) | Time and timestamps |
//! | [`pith-filesystem`](https://docs.rs/pith-filesystem) | File I/O |
//! | [`pith-io`](https://docs.rs/pith-io) | Streams and polling |
//! | [`pith-random`](https://docs.rs/pith-random) | Randomness |
//! | [`pith-sockets`](https://docs.rs/pith-sockets) | Raw networking |
//!
//! ### Contested Domains (Medium Value)
//!
//! Areas where the ecosystem has multiple viable options:
//!
//! | Crate | Domain | Ecosystem Alternatives |
//! |-------|--------|----------------------|
//! | [`pith-http`](https://docs.rs/pith-http) | HTTP client/server | reqwest, ureq, hyper |
//! | [`pith-sql`](https://docs.rs/pith-sql) | SQL databases | rusqlite, sqlx, diesel |
//! | [`pith-cache`](https://docs.rs/pith-cache) | Caching with TTL | moka, cached, etc. |
//! | [`pith-crypto`](https://docs.rs/pith-crypto) | Cryptography | ring, rustcrypto |
//! | [`pith-logging`](https://docs.rs/pith-logging) | Logging | log, tracing |
//! | [`pith-markdown`](https://docs.rs/pith-markdown) | Markdown | pulldown-cmark, comrak |
//! | [`pith-config`](https://docs.rs/pith-config) | Configuration | figment, config |
//! | [`pith-websocket`](https://docs.rs/pith-websocket) | WebSocket | tungstenite, etc. |
//!
//! ## Solved Domains (Use Directly)
//!
//! For these domains, the Rust ecosystem has clear winners.
//! **Don't use pith wrappers - use these directly:**
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
//!
//! Creating pith wrappers for these would add friction without benefit.
//!
//! ## Usage
//!
//! Add the specific pith crates you need:
//!
//! ```toml
//! [dependencies]
//! pith-filesystem = "0.1"
//! pith-http = "0.1"
//! pith-clocks = "0.1"
//!
//! # Native backends
//! pith-filesystem-native = "0.1"
//! pith-http-native = "0.1"
//! pith-clocks-native = "0.1"
//! ```
//!
//! ## Backends
//!
//! Each interface has one or more backend implementations:
//!
//! - `*-native` - Native OS implementation
//! - `*-wasm` - WebAssembly implementation (planned)
//! - `*-mock` - Testing mocks (planned)
//!
//! ## Links
//!
//! - [GitHub Repository](https://github.com/rhizome-lab/pith)
//! - [Design Guidelines](https://github.com/rhizome-lab/pith/blob/main/DESIGN.md)

#![no_std]

// This crate is documentation-only.
// Use the specific pith-* crates for functionality.
