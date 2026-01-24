# portals

A standard library of interfaces for Rust.

Pith provides capability-based interfaces that enable portability across platforms (native, WASM, embedded) while reducing decision fatigue in contested ecosystem domains.

## Philosophy

- **Portability over power** - simpler interfaces that work everywhere
- **Reduce decision fatigue** - blessed choices in contested domains
- **Consistent APIs** - uniform patterns across all portals crates

## Quick Start

Add the specific portals crates you need:

```toml
[dependencies]
portals-filesystem = "0.1"
portals-http = "0.1"
portals-clocks = "0.1"

# Native backends
portals-filesystem-native = "0.1"
portals-http-native = "0.1"
portals-clocks-native = "0.1"
```

## Solved Domains (Use Directly)

For these domains, just use the ecosystem standard directly:

| Domain | Use This |
|--------|----------|
| Serialization | [serde](https://crates.io/crates/serde) |
| JSON | [serde_json](https://crates.io/crates/serde_json) |
| CLI parsing | [clap](https://crates.io/crates/clap) |
| URL parsing | [url](https://crates.io/crates/url) |
| UUID | [uuid](https://crates.io/crates/uuid) |
| Regex | [regex](https://crates.io/crates/regex) |
| Async runtime | [tokio](https://crates.io/crates/tokio) |

See [docs.rs/portals](https://docs.rs/portals) for the full interface catalog and recommendations.

## License

MIT OR Apache-2.0
