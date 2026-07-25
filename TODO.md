# TODO

## Roadmap

1. [x] Diverge from WASI where ergonomics warrant
2. [x] Add more interfaces (crypto, encoding, sql)
3. [x] Reference implementations for native targets
4. [x] Unified streams (filesystem returns `portals-io` streams)
5. [x] WebSocket and DNS interfaces + native backends
   - [x] portals-websocket interface + portals-websocket-native (via tungstenite)
   - [x] portals-dns interface + portals-dns-native (via hickory-resolver)
6. [x] Protocol implementations (`crates/protocols/`)
   - [x] portals-http1 (HTTP/1.1 wire format parsing/serialization)
7. [x] Mock backends for testing (`crates/backends/mock/`)
   - [x] portals-clocks-mock (controllable wall/monotonic clocks)
   - [x] portals-random-mock (deterministic secure/insecure random)
   - [x] portals-http-mock (request recording, response queuing)
8. [x] Crypto AAD support (AES-GCM, ChaCha20-Poly1305)


### [x] Update CLAUDE.md — corrections as documentation lag (2026-03-29)

Add to the corrections section:
> **Corrections are documentation lag, not model failure.** When the same mistake recurs, the fix is writing the invariant down — not repeating the correction. Every correction that doesn't produce a CLAUDE.md edit will happen again. Exception: during active design, corrections are the work itself — don't prematurely document a design that hasn't settled yet.

Add to the Session Handoff section:
> **Initiate a handoff after a significant mid-session correction.** When a correction happens after substantial wrong-path work, the wrong reasoning is still in context and keeps pulling. Writing down the invariant and starting fresh beats continuing with poisoned context — the next session loads the invariant from turn 1 before any wrong reasoning exists.

Conventional commit: `docs: add corrections-as-documentation-lag + context-poisoning handoff rule`

## Potential Interfaces

Application-level interfaces to consider (beyond WASI):

### Identity / Auth
- **portals-jwt** - JWT parsing/validation/creation
- **portals-oauth** - OAuth flow abstractions
- **portals-session** - session management

### Data / Validation
- **portals-cache** - caching with TTL/LRU policies
- **portals-validation** - schema validation
- **portals-serialization** - JSON/TOML/YAML/etc (or per-format crates)

### Text / Formatting
- **portals-i18n** - internationalization/localization
- **portals-markdown** - markdown parsing/rendering
- **portals-template** - templating

### Media
- **portals-image** - image metadata/transforms
- **portals-audio** - audio metadata/processing
- **portals-video** - video metadata

### Communication
- **portals-email** - email sending/parsing
- **portals-notification** - push notifications

### Identifiers
- **portals-uuid** - UUID generation/parsing
- **portals-nanoid** - nanoid generation
- **portals-snowflake** - snowflake IDs

### Scheduling
- **portals-cron** - cron expressions/scheduling
- **portals-delay** - delayed/scheduled tasks

### no_std Primitives
- **portals-collections** - portable collections
- **portals-sync** - sync primitives (mutex, rwlock, etc)
- **portals-alloc** - allocator interfaces

## Future Considerations

- **`spore-portals`**: Lua bindings (belongs in Spore, not here)

Potential interface improvements to consider later:

- [x] **Filesystem seek**: Add `Seek` trait for random access file operations
- [x] **Zero-copy reads**: Add `read_into(&mut self, buf: &mut [u8])` to `InputStream`

## ADRs

- 0001: `InsecureRandom` uses `&mut self` (PRNGs need state)
- 0002: Async runtime via tokio feature flag
- 0003: Stdio uses `&mut self` (matches std::io)
- 0004: Capability audit (4 violations fixed)
