---
created_at: "2026-06-15T15:16:27.170889790+00:00"
id: "atelier-3kap"
issue_type: "epic"
labels:
- "closeout"
- "rewrite"
- "workspace"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fchz"
  children:
  - kind: "issue"
    id: "atelier-4j3k"
  - kind: "issue"
    id: "atelier-cwgx"
  - kind: "issue"
    id: "atelier-epzs"
  - kind: "issue"
    id: "atelier-qsib"
  - kind: "issue"
    id: "atelier-vu2b"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:59:02.074088774+00:00"
status: "done"
title: "Epic: Root package deletion and warning-free closeout"
updated_at: "2026-06-15T18:59:02.074088774+00:00"
---

## Description

Remove the old root crate after the layered workspace is authoritative, and prove the repository has no compatibility imports, root modules, removed-command guidance, or warning debt at closeout.

## Outcome

- The repository root is a virtual Cargo workspace with no root package, lib target, bin target, or root-owned integration test target.
- The `atelier` executable is owned by `crates/atelier-cli` and still builds to `target/debug/atelier`.
- Old root source modules and compatibility import paths are deleted rather than preserved as shims.
- Closeout checks prove the workspace is warning-free and root-package regressions are guarded.

## Evidence

- Child proof covers virtual workspace conversion, root source deletion, test/fuzz relocation, and compatibility-path validation.
- `cargo metadata --no-deps --format-version 1` transcript shows no root package and lists the expected workspace members.
- `RUSTFLAGS=-Dwarnings cargo check --workspace --all-targets`, `cargo fmt -- --check`, `cargo nextest run`, `atelier lint`, `atelier export --check`, and `atelier doctor` pass before epic closeout.
- Search transcript proves no `atelier::...` root compatibility imports, `crate::commands`, `crate::db`, or deleted root module references remain.

## Notes

- Root package deletion closeout must inventory temporary adapters named by
  implementation epics under `docs/architecture/source-layout.md` and either
  remove them, link open removal work that blocks closeout, or classify them as
  target-state code.
