---
created_at: "2026-06-15T15:17:42.061160516+00:00"
id: "atelier-cwgx"
issue_type: "task"
labels:
- "fuzz"
- "tests"
- "workspace"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vu2b"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T18:55:08.925579575+00:00"
status: "done"
title: "Move root integration smoke and fuzz imports into owning crates"
updated_at: "2026-06-15T18:55:08.925579575+00:00"
---

## Description

Move remaining root integration and smoke tests under crates/atelier-cli/tests or crate-specific test directories, and retarget fuzz imports so the virtual workspace has no root package test targets.

## Outcome

- Root integration and smoke tests move under `crates/atelier-cli/tests` or crate-specific test directories before the root package is removed.
- Fuzz targets import the owning internal crates rather than old single-crate `atelier::...` paths.
- Test helpers remain shared only through crate-appropriate support modules, not root package test infrastructure.

## Evidence

- File inventory proves no tests depend on a root package test target.
- `cargo nextest run --test cli_integration` or the renamed CLI integration suite passes from the CLI crate.
- `cargo check --manifest-path fuzz/Cargo.toml --bins` passes and search proves fuzz targets no longer import old root crate APIs.
