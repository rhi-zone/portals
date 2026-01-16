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

## Why Not Wrap These?

1. **No decision fatigue to solve** - everyone already uses these
2. **APIs are well-known** - wrapping adds learning cost
3. **No portability benefit** - same impl everywhere
4. **Maintenance burden** - tracking upstream changes

## Pith's Focus

Pith focuses on domains where:
- Multiple viable options exist (contested)
- Abstraction enables portability (primitives)
- A consistent API across pith crates adds value

See [DESIGN.md](../DESIGN.md) for the full categorization.
