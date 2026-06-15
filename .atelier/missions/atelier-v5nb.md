---
created_at: "2026-06-15T05:11:17.608407137+00:00"
id: "atelier-v5nb"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-0fhv"
    type: "advances"
  - kind: "issue"
    id: "atelier-2q5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-4wor"
    type: "advances"
  - kind: "issue"
    id: "atelier-fchz"
    type: "advances"
  - kind: "issue"
    id: "atelier-kjj1"
    type: "advances"
  - kind: "issue"
    id: "atelier-lu10"
    type: "advances"
  - kind: "issue"
    id: "atelier-qsbe"
    type: "advances"
  - kind: "issue"
    id: "atelier-ycmp"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "In-place crate rewrite and SQLite projection redesign"
updated_at: "2026-06-15T08:28:23.591775786+00:00"
---

## Intent

Break Atelier's single Rust crate into a layered Cargo workspace while preserving Markdown canonical state and stable visible CLI behavior. The rewrite is contract-first: architecture artifacts define crate boundaries, SQLite projection/runtime ownership, and active-work removal before implementation epics start.

## Constraints

- Markdown records under .atelier/ remain canonical and stable unless a contract issue explicitly changes them.
- Visible CLI command intent, help surfaces, and product docs remain stable; exact transcript formatting may change deliberately with tests updated.
- Rust crate APIs are internal and may be replaced; repo tests and fuzz targets move to the new crates.
- SQLite schema compatibility is not required because rebuild and doctor recover local projection/runtime state from canonical Markdown.
- Temporary migration adapters are tracked internal implementation details only and must have removal owners.

## Risks

- Crate movement can hide product behavior regressions behind broad integration tests unless each crate owns focused invariants.
- Runtime active-work and claim removal conflicts with older work-association guidance and must be resolved before the SQLite rewrite lands.
- Oversized command and test files can be mechanically moved without improving boundaries unless application and CLI ownership are defined first.

## Validation

- Closeout maps every mission outcome to linked epic and issue evidence.
- Independent validation covers CLI/help/docs parity, Markdown round trips, projection rebuild, missing or stale DB recovery, active-work removal, and representative mission, issue, and evidence workflows.
- Repository checks pass: cargo fmt -- --check, cargo nextest run, relevant extended ignored tests, atelier lint, atelier export --check, and atelier doctor.

## Closeout Notes

- Close reason: All linked work is closed and validation evidence atelier-7aga passed the mission closeout checks.
