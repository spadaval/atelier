---
created_at: "2026-06-11T18:53:45.561669852+00:00"
id: "atelier-wznt"
issue_type: "epic"
labels:
- "config"
- "migration"
- "storage"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-dinu"
  - kind: "issue"
    id: "atelier-dydv"
  - kind: "issue"
    id: "atelier-lcgi"
  - kind: "issue"
    id: "atelier-nwlx"
  - kind: "issue"
    id: "atelier-vme7"
  - kind: "issue"
    id: "atelier-xcy9"
  attachments:
  - kind: "evidence"
    id: "atelier-k2mg"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Collapse Atelier project state into one .atelier directory"
updated_at: "2026-06-12T00:18:34.520571163+00:00"
---

## Description

Problem: Atelier currently splits durable tracker records into .atelier-state while .atelier mixes local SQLite runtime state, generated rule files, hook config, locks, and local overrides. This makes the product model hard to explain and leaves config untracked.

Target direction: use one .atelier/ root. Canonical project records and project config are tracked under .atelier/. Runtime files such as state.db, WAL/SHM, cache, locks, and local identity remain ignored. Remove copied .atelier/rules. Treat Claude hooks as an optional integration, not core tracker init.

## Outcome

- The target filesystem contract is documented before implementation slices land.
- Child implementation issues cover layout migration, ignore policy, init behavior, config, optional hooks, and validation.
- The epic is closed only after export/rebuild, init, and docs agree on the one-directory model.

## Evidence

Evidence was not specified in the legacy issue record.
