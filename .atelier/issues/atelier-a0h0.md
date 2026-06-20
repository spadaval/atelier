---
created_at: "2026-06-20T15:55:48.533628698+00:00"
id: "atelier-a0h0"
issue_type: "bug"
labels:
- "cutting-pass"
- "fields"
- "schema"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Clarify or remove generic issue fields storage"
updated_at: "2026-06-20T15:56:45.847091628+00:00"
---

## Description

Issue records expose a generic `fields` map in code and canonical parsing, but schema version 3 only accepts the built-in `review` field. This is technically a generic custom-field mechanism with a single allowed key, which makes the product contract ambiguous.

## Outcome

The issue record contract is explicit: either the generic `fields` bag is removed/renamed in favor of first-class typed fields such as `review`, or the product intentionally defines a real user-facing custom field system. The target decision should avoid preserving a misleading half-measure.

## Evidence

- Command proof: `rg "fields" crates/atelier-*` shows either no generic issue fields surface or a documented custom-field system.
- Command proof: `target/debug/atelier rebuild` rejects unsupported issue metadata with a clear schema diagnostic.
- File proof: `docs/architecture/markdown-first-record-store.md` and `docs/product/workflow-configuration.md` agree on the final issue field contract.
