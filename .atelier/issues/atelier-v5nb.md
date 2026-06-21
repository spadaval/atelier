---
created_at: '2026-06-15T05:11:17.608407137+00:00'
id: atelier-v5nb
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-0fhv
    type: advances
  - kind: issue
    id: atelier-2q5s
    type: advances
  - kind: issue
    id: atelier-3kap
    type: advances
  - kind: issue
    id: atelier-4wor
    type: advances
  - kind: issue
    id: atelier-fchz
    type: advances
  - kind: issue
    id: atelier-kjj1
    type: advances
  - kind: issue
    id: atelier-lu10
    type: advances
  - kind: issue
    id: atelier-qsbe
    type: advances
  - kind: issue
    id: atelier-ycmp
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-15T19:02:20.988195150+00:00'
status: closed
title: Complete The Atelier Crate Migration
updated_at: '2026-06-15T19:02:20.988195150+00:00'
---

## Description

Finish the migration by making the layered workspace the only architecture, not a parallel scaffold. The root package becomes a virtual workspace, crates/atelier-cli owns the atelier binary, atelier-sqlite owns SQLite projection and runtime schema/query code, and atelier-app owns use-case orchestration through request/outcome/view APIs that atelier-cli renders.

## Outcome

### Constraints

- Markdown records under .atelier/ remain canonical and stable unless a contract issue explicitly changes them.
- Visible CLI command intent, help surfaces, and product docs remain stable; exact transcript formatting may change deliberately with tests updated.
- Rust crate APIs are internal and may be replaced; repo tests and fuzz targets move to the new crates.
- SQLite schema compatibility is not required because rebuild and doctor recover local projection/runtime state from canonical Markdown.
- Closeout is deletion-driven: no root crate, no old `atelier::...` re-export paths, no removed-command runtime guidance, no SQLite comments/sessions/work-association hints, and no warnings under `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`.
- Temporary migration adapters are tracked internal implementation details only and must have removal owners before the root-deletion epic can close.

### Risks

- Crate movement can hide product behavior regressions behind broad integration tests unless each crate owns focused invariants.
- Runtime active-work and claim removal conflicts with older work-association guidance and must be resolved before the SQLite rewrite lands.
- Oversized command and test files can be mechanically moved without improving boundaries unless application and CLI ownership are defined first.

## Evidence

- Manual check: Closeout maps every mission outcome to linked epic and issue evidence.
- Manual check: Independent validation covers CLI/help/docs parity, Markdown round trips, projection rebuild, missing or stale DB recovery, active-work removal, root crate deletion, ordinary Clap rejection for removed commands, and representative mission, issue, and evidence workflows.
- Manual check: Repository checks pass: `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`, `cargo fmt -- --check`, `cargo nextest run`, relevant extended ignored tests, `cargo check --manifest-path fuzz/Cargo.toml --bins`, `target/debug/atelier lint`, `target/debug/atelier export --check`, `target/debug/atelier doctor`, and `git diff --check`.

## Notes

### Terminal Notes

- Close reason: All crate rewrite mission work, epics, validations, and closeout gates are complete.

Migrated from `.atelier/missions/atelier-v5nb.md` as a declared mission objective issue.
