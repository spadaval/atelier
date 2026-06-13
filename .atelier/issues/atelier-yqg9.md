---
created_at: "2026-06-13T20:35:27.069049672+00:00"
id: "atelier-yqg9"
issue_type: "epic"
labels:
- "architecture"
- "cleanup"
- "stabilization"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-10qm"
  - kind: "issue"
    id: "atelier-2ehd"
  - kind: "issue"
    id: "atelier-4u5h"
  - kind: "issue"
    id: "atelier-d7lw"
  - kind: "issue"
    id: "atelier-ggls"
  - kind: "issue"
    id: "atelier-ihz0"
  - kind: "issue"
    id: "atelier-ja3o"
  - kind: "issue"
    id: "atelier-k3vs"
  - kind: "issue"
    id: "atelier-kpm8"
  - kind: "issue"
    id: "atelier-wj05"
  - kind: "issue"
    id: "atelier-yqj6"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Reduce code complexity and module boundary drift"
updated_at: "2026-06-13T23:10:38.230151438+00:00"
---

## Description

Audit and simplify implementation boundaries that make the CLI and data model harder to change: oversized modules, generic record plumbing, inherited lock/sync residue, and duplicate SQLite-first paths.

## Outcome

- Command routing stays thin and command handlers own focused user intent.
- Record parsing/rendering/validation has cohesive ownership by record kind or shared primitive.
- Inherited or dead modules are deleted, isolated, or explicitly assigned to a future product contract.
- SQLite projection code contains metadata/query behavior, not competing canonical payload ownership.

## Evidence

- Architecture review artifact classifies each large or suspect module by keep/split/delete/reconnect.
- Residue searches for compatibility, locks/sync, data_json/body payloads, and SQLite-first mutation paths are attached.
- Focused tests plus cargo fmt -- --check and relevant cargo nextest slices pass.
