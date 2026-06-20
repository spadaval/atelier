---
created_at: "2026-06-20T15:11:24.269278665+00:00"
id: "atelier-6hcl"
issue_type: "task"
labels:
- "cutting-pass"
- "removal"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-p1yz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Delete retired command modules and stale implementation owners"
updated_at: "2026-06-20T21:51:36.799068614+00:00"
---

## Description

Delete active implementation modules and names that only exist to preserve removed or renamed product concepts. Current audit examples include bundle code delegating to plan-owned modules and compiled-but-unused command modules such as legacy label/tested surfaces.

## Outcome

Removed product names do not remain as active implementation owners. Bundle behavior is owned by bundle-named code, unused modules are deleted, and command dispatch has no stale module declarations.

## Evidence

- `rg "commands::plan|pub mod tested|pub mod label|Commands::"` confirms no stale implementation owner remains for deleted surfaces.
- `cargo build -p atelier-cli` passes after module deletion.
- Command-audit retired notes match the final code surface.
