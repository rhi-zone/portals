# ADR-0001: Random trait mutability

## Status

Proposed

## Context

WASI's `wasi:random` interface provides:
- `get-random-bytes(len: u64) -> list<u8>` - cryptographically secure
- `get-random-u64() -> u64` - cryptographically secure
- `get-insecure-random-bytes(len: u64) -> list<u8>` - fast, non-crypto
- `get-insecure-random-u64() -> u64` - fast, non-crypto

These are free functions, not methods on objects. Our initial interface mirrored this with traits:

```rust
pub trait SecureRandom {
    fn fill(&self, buf: &mut [u8]);
}

pub trait InsecureRandom {
    fn fill(&self, buf: &mut [u8]);
}
```

## Problem

`SecureRandom` works fine with `&self` - OS entropy sources are stateless from the caller's perspective.

`InsecureRandom` doesn't work with `&self` - PRNGs must mutate internal state. Options:
1. Interior mutability (`RefCell`, `Mutex`, atomics)
2. Change signature to `&mut self`
3. Use free functions like WASI

## Decision

**Change `InsecureRandom` to use `&mut self`.**

Rationale:
- Explicit mutability is idiomatic Rust
- Interior mutability adds overhead and API complexity
- Free functions don't compose well (can't pass around "a source of randomness")
- `SecureRandom` keeps `&self` since OS entropy is conceptually stateless

## Consequences

```rust
pub trait SecureRandom {
    fn fill(&self, buf: &mut [u8]);  // &self - stateless
}

pub trait InsecureRandom {
    fn fill(&mut self, buf: &mut [u8]);  // &mut self - stateful
}
```

This diverges from WASI's uniform interface but better fits Rust's ownership model.
