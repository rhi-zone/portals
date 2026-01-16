# Ecosystem Recommendations

For some domains, the Rust ecosystem already has clear winners. Rather than wrap these with marginal value, we recommend using them directly.

## Solved Domains (Just Use These)

| Domain | Recommended Crate | Why |
|--------|------------------|-----|
| **Serialization** | [serde](https://crates.io/crates/serde) | Universal standard, everything supports it |
| **JSON** | [serde_json](https://crates.io/crates/serde_json) | De facto standard with serde |
| **CLI parsing** | [clap](https://crates.io/crates/clap) | Dominant, excellent derive macros |
| **URL parsing** | [url](https://crates.io/crates/url) | WHATWG spec compliant, widely used |
| **UUID** | [uuid](https://crates.io/crates/uuid) | Dominant, full RFC 4122 support |
| **Regex** | [regex](https://crates.io/crates/regex) | Fast, safe, dominant |
| **Async runtime** | [tokio](https://crates.io/crates/tokio) | Ecosystem standard for async |
| **Multi-pattern matching** | [aho-corasick](https://crates.io/crates/aho-corasick) | Used by regex internally, dominant |
| **Library error types** | [thiserror](https://crates.io/crates/thiserror) | Dominant derive macro for Error |
| **App error handling** | [anyhow](https://crates.io/crates/anyhow) | Dominant for application errors |
| **Incremental parsing** | [tree-sitter](https://crates.io/crates/tree-sitter) | Dominant in editor/tooling space |

## Contested Domains (Watching)

These have multiple viable options. We're not picking winners yet.

| Domain | Options | Notes |
|--------|---------|-------|
| **Parser combinators** | nom, winnow, chumsky, pest | winnow is nom's successor, chumsky has nice errors |
| **Zero-copy serialization** | rkyv, bincode, postcard | Different tradeoffs (speed vs size vs features) |
| **Datetime** | chrono, time | Both mature, `time` is lighter |

## Why Not Wrap These?

1. **No decision fatigue to solve** - everyone already uses these
2. **APIs are well-known** - wrapping adds learning cost
3. **No portability benefit** - same impl everywhere
4. **Maintenance burden** - tracking upstream changes

## What Pith Is

Pith provides:
- **Capability abstractions** - traits for fs, io, sockets, clocks, random
- **Contested infrastructure** - blessed choices for http, sql, caching where ecosystem is fragmented
- **Portability** - same interface across native, WASM, embedded

## What Pith Is Not

Pith does not try to:
- **Wrap solved domains** - use serde, clap, regex directly
- **Abstract stylistic choices** - error handling style, parser combinator preference
- **Replace the ecosystem** - we complement it, not compete with it
- **Be a framework** - pith is à la carte, pick what you need

The goal is reducing decision fatigue for *capabilities and infrastructure*, not becoming "the one true Rust stack."

See [DESIGN.md](../DESIGN.md) for the full design philosophy.
