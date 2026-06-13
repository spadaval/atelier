---
created_at: "2026-06-13T20:37:11.930202224+00:00"
id: "atelier-ihz0"
issue_type: "task"
labels:
- "architecture"
- "assignee:root"
- "data-model"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Replace generic DomainRecord data_json plumbing with typed records"
updated_at: "2026-06-13T23:50:34.383513364+00:00"
---

## Description

The generic DomainRecord model and records table preserve kind/body/data_json as a common payload shape even after missions gained readable sections. Audit whether first-class records should use typed Rust structs and projection metadata instead of generic JSON payload storage.

## Outcome

- Mission, plan, evidence, and milestone code use typed domain structs for canonical fields and projection metadata.
- SQLite projection stores query fields and links, not a parallel full payload copy.
- Generic record code remains only where it is a deliberate low-level projection primitive, not the public data model.

## Evidence

- `rg` command output residue search classifies `DomainRecord`, `data_json`, `records.body`, and `records.data_json` usages.
- Focused `atelier rebuild`, `atelier export --check`, show, and list tests prove typed records round-trip.
- Architecture documentation diff or review artifact records any intentionally retained generic projection primitive.
