---
created_at: "2026-10-17T01:10:57.987452965+00:00"
id: "atelier-cy58"
issue_type: "feature"
labels:
- "cli-output"
- "work-views"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show each row's mission and parent in repo-wide work views"
updated_at: "2026-10-17T01:10:57.987452965+00:00"
---

## Goal

Repo-wide rows read `ready atelier-ubti [todo] low - Audit legacy widget names`, with nothing saying which mission it advances or what parent it sits under, so mission work and stray work look alike. On master, atelier-bw0b blocks three items of mission atelier-1w8c yet advances no mission, and the row cannot say so. An agent rebuilt it by hand: `atelier work ready | grep -E "^\s+ready" | tail -8; for i in 6qk1 aonb …; do p=$(atelier issue show astromech-$i | sed -n 's/^Parent: //p'); …` (astromech-atelier, 2 calls).

## Done when

- Rows in `work ready`, `work blocked`, `work human` and `work active` end with `· mission atelier-j76w`, `· parent atelier-qhnh`, both, or `· no mission`.
- Tests cover all four cases.

## Context

atelier-7n5u adds an in-flight mark to the same rows; coordinate the row format.
