---
created_at: "2026-06-17T19:37:13.145493659+00:00"
id: "atelier-6jap"
issue_type: "epic"
labels:
- "architecture"
- "audit"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-98mo"
  children:
  - kind: "issue"
    id: "atelier-2573"
  - kind: "issue"
    id: "atelier-3h90"
  - kind: "issue"
    id: "atelier-adub"
  - kind: "issue"
    id: "atelier-m6zl"
  - kind: "issue"
    id: "atelier-ynks"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Finish storage boundary cleanup"
updated_at: "2026-06-17T23:05:06.484242283+00:00"
---

## Description

Finish storage-boundary cleanup uncovered by the architecture audit. RecordStore
and SQLite have moved toward the target architecture, but live code still mixes
large record-store responsibilities, duplicated priority vocabulary, raw SQLite
access, compatibility-style session/work APIs, and unclear comment sidecar
ownership.

This epic owns interface cleanup and boundary proof. It should not change public
CLI behavior unless a child issue explicitly names that behavior.

## Outcome

- RecordStore code is split by durable record ownership and parser/rendering
  responsibilities are easy to locate.
- Canonical priority, human priority labels, projection filters, and work-order
  sorting use one shared conversion contract.
- `atelier-sqlite::Database` exposes narrow projection/query APIs; raw
  connection access and compatibility helpers are private, deleted, or isolated
  behind clearly named test/import adapters.
- Comment sidecar ownership is documented and implemented in the boundary that
  owns the canonical activity files.

## Evidence

- Focused records and SQLite tests cover record round trips, rebuild/projection
  behavior, priority conversion/filtering, and database interface encapsulation.
- Search transcript proves removed or isolated compatibility APIs are no longer
  used by production code.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and
  `git diff --check` pass.
