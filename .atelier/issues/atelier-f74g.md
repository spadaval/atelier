---
created_at: "2026-06-15T05:13:25.635642858+00:00"
id: "atelier-f74g"
issue_type: "task"
labels:
- "adr"
- "architecture"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-15T05:55:23.278418170+00:00"
status: "done"
title: "Record workspace layering ADR and active-work decision"
updated_at: "2026-06-15T05:55:23.278418170+00:00"
---

## Description

Record the hard-to-reverse architecture decisions behind the crate rewrite and current-work model so future contributors do not collapse the workspace or preserve runtime active-work state accidentally.

## Outcome

- A new ADR records the workspace/crate layering decision, dependency direction, internal API policy, and why a layered workspace is preferred over the current monolith.
- ADR 0004 is amended, superseded, or clearly cross-referenced so runtime work associations are no longer the accepted current-work source of truth.
- The ADRs distinguish source-of-truth choices from temporary migration adapters.

## Evidence

- File change review of `docs/adr/` shows the new workspace decision and the active-work decision update.
- File change review of architecture docs shows references to the new ADR where workspace layering or active-work source of truth is described.
- `atelier lint atelier-f74g` and `atelier export --check` pass.
