# CLAUDE.md

Pith is a standard library of interfaces, inspired by WASI.

## Structure

```
crates/
├── interfaces/     # Trait definitions (pith-*)
└── backends/       # Implementations
    ├── native/     # Native OS implementations
    └── wasm/       # WASM implementations
```

### Interfaces

Each crate in `interfaces/` defines traits for a capability domain:
- `pith-clocks` - time
- `pith-cli` - command-line environment
- `pith-filesystem` - file I/O
- `pith-http` - HTTP
- `pith-io` - streams and polling
- `pith-random` - randomness
- `pith-sockets` - networking

### Backends

Implementations go in `backends/<target>/`. For example:
- `backends/native/pith-clocks-native` - native clock implementation
- `backends/wasm/pith-clocks-wasm` - WASM clock implementation

## Design

- Interfaces define traits, backends provide implementations
- Capability-based (no global/ambient access)
- Async-first where blocking is possible
- Mirror WASI structure but diverge for ergonomics where sensible
