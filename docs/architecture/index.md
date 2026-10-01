# Architecture Guide

Atelier is a Rust workspace that parses repository records directly and applies
invariant checks before mutation. [ADR 0019](../adr/0019-verified-repo-native-tracker.md)
is the governing architecture decision.

- [Markdown record store](markdown-first-record-store.md): durable state,
  direct reads, mutation boundaries, and Git authority.
- [Source layout](source-layout.md): crate responsibilities and dependency direction.
- [Quality standards](quality/standards.md): engineering requirements.
- [Validation](quality/validation.md): proof and review boundaries.
- [Quality vocabulary](quality/architecture-quality.md) and
  [Rust hazard scans](quality/rust-quality-hazard-scans.md): focused review tools.
- [Provenance](provenance.md): historical Chainlink origin.

Product contracts live in the [product guide](../product/index.md). Agent
Factory owns process and templates; architecture should not duplicate its
assignment choreography. [Dated audits](quality/index.md) describe the system
as it was when examined.
