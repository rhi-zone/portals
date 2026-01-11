# CLAUDE.md

Pith is a standard library of interfaces, inspired by WASI.

## Structure

Each crate defines traits for a capability domain:
- `pith-clocks` - time
- `pith-cli` - command-line environment
- `pith-filesystem` - file I/O
- `pith-http` - HTTP
- `pith-io` - streams and polling
- `pith-random` - randomness
- `pith-sockets` - networking

## Design

- Traits, not implementations (implementations live elsewhere)
- Capability-based (no global/ambient access)
- Async-first where blocking is possible
- Mirror WASI structure but diverge for ergonomics where sensible
