---
created_at: "2026-06-23T15:22:44.214055809+00:00"
id: "atelier-7fof"
issue_type: "feature"
labels:
- "cli"
- "evidence"
- "history"
- "human-output"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-23T23:32:50.686219553+00:00"
status: "done"
title: "Refresh evidence and history browse output"
updated_at: "2026-06-23T23:32:50.686219553+00:00"
---

## Description

Refresh proof and activity browsing surfaces so they remain useful at repository
scale.

## Outcome

- `evidence list` has a bounded default view, omitted-count reporting, useful
  grouping or filtering hints, and elided command transcripts.
- `evidence show` remains the focused proof detail surface.
- `history` rows are grouped or wrapped so the event sentence is primary and
  repeated metadata is de-emphasized.

## Evidence

- Before/after transcripts cover a repository with hundreds of evidence records
  and issue history with repeated evidence events.
- Focused tests cover default evidence list limits, transcript elision, history
  wrapping or grouping, and omitted-count output.
- `target/debug/atelier lint`, focused CLI tests, and `git diff --check` pass.
