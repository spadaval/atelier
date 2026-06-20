---
created_at: "2026-06-19T19:39:37.210575708+00:00"
id: "atelier-f55w"
issue_type: "feature"
labels:
- "local-state"
- "prune"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-iq7f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Prune ignored local runtime, cache, and diagnostics"
updated_at: "2026-06-19T19:39:37.210575708+00:00"
---

## Description

Implement pruning for ignored local state that is explicitly outside canonical
project records: runtime cache, command diagnostics, stale projection artifacts,
and other local-only files allowed by the contract.

## Outcome

- Apply mode can remove eligible ignored local artifacts selected by the dry-run
  candidate engine.
- Cleanup failure for non-critical diagnostics is reported without corrupting
  canonical records or blocking unrelated normal commands.
- Projection rebuild or doctor guidance is shown when cleanup affects local
  state needed by subsequent commands.

## Evidence

- Focused tests create stale local artifacts, show dry-run candidates, apply
  cleanup, and prove canonical `.atelier/` records are unchanged.
- Transcript includes `atelier lint` or a targeted post-cleanup health check.
