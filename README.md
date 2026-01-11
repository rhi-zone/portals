# Pith

Standard library interfaces.

Capability-based, async-first interfaces inspired by WASI, designed to be implementable across runtimes.

## Crates

| Crate | Description | WASI Equivalent |
|-------|-------------|-----------------|
| `pith-clocks` | Wall clock, monotonic clock | `wasi:clocks` |
| `pith-cli` | Args, environment, stdio | `wasi:cli` |
| `pith-crypto` | Hashing, HMAC, encryption, signatures | - |
| `pith-encoding` | Base64, hex, URL encoding | - |
| `pith-filesystem` | Files, directories | `wasi:filesystem` |
| `pith-http` | HTTP client/server | `wasi:http` |
| `pith-io` | Streams, polling | `wasi:io` |
| `pith-random` | Secure and insecure RNG | `wasi:random` |
| `pith-sockets` | TCP, UDP, DNS | `wasi:sockets` |
| `pith-sql` | Database connections, queries | - |

## Structure

```
crates/
├── interfaces/     # Trait definitions (pith-*)
└── backends/       # Implementations
    ├── native/     # Native OS implementations
    └── wasm/       # WASM implementations
```

## Design Principles

- **Capability-based**: Access is granted through capability objects, not ambient authority
- **Async-first**: Operations that may block return futures
- **Minimal**: Interfaces define traits, backends provide implementations
- **Portable**: Implementable on native, WASM, and embedded targets
