---
created_at: "2026-06-17T19:37:33.077782733+00:00"
id: "atelier-wpht"
issue_type: "task"
labels:
- "app-layer"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:44:16.357365357+00:00"
status: "done"
title: "Reduce CLI command modules to renderers and adapters"
updated_at: "2026-06-17T23:44:16.357365357+00:00"
---

## Description

After app APIs own orchestration, reduce leftover CLI command modules to
terminal rendering and argument adapters. Delete modules or functions that have
no remaining CLI-only responsibility.

## Outcome

- CLI command modules for migrated workflows contain rendering, formatting, or
  argument adaptation only.
- Shared workflow state, storage decisions, and mutation sequencing are not
  implemented in `crates/atelier-cli/src/commands`.
- Removed adapters do not create public compatibility aliases, fallback shims,
  or old command behavior.

## Evidence

- File review or search transcript shows migrated CLI command modules no longer
  own app orchestration.
- Representative CLI integration tests still pass for migrated command
  surfaces.
- `atelier lint` and `git diff --check` pass.
