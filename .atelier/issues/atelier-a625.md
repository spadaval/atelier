---
created_at: "2026-06-14T02:48:38.708529049+00:00"
id: "atelier-a625"
issue_type: "epic"
labels:
- "reliability"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-isd5"
  - kind: "issue"
    id: "atelier-r5tr"
  - kind: "issue"
    id: "atelier-zah8"
  children:
  - kind: "issue"
    id: "atelier-51iv"
  - kind: "issue"
    id: "atelier-613f"
  - kind: "issue"
    id: "atelier-h184"
  - kind: "issue"
    id: "atelier-j01c"
  - kind: "issue"
    id: "atelier-mxug"
  - kind: "issue"
    id: "atelier-oqtz"
  - kind: "issue"
    id: "atelier-ovv0"
  - kind: "issue"
    id: "atelier-qdaw"
  - kind: "issue"
    id: "atelier-srvz"
  - kind: "issue"
    id: "atelier-uy4o"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T08:24:05.578265724+00:00"
status: "done"
title: "Epic: Make tracker state and worktree recovery unambiguous"
updated_at: "2026-06-14T08:24:05.578265724+00:00"
---

## Description

Projection, canonical-state, worktree, and active-work failures become recoverable operator flows instead of mission-blocking ambiguity.

## Outcome

Stale or invalid local state reports one repair path, runtime/cache artifacts stay out of canonical diagnostics, and worktree/active-work setup is atomic or repairable.

## Evidence

Focused tests or validation transcripts cover stale projection recovery, invalid canonical Markdown messaging, worktree setup, and active-work reconciliation.
