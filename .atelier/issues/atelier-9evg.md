---
created_at: "2026-06-29T20:13:19.532733179+00:00"
id: "atelier-9evg"
issue_type: "task"
labels:
- "cli"
- "evidence"
- "output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Bound evidence list and elide transcript firehoses"
updated_at: "2026-06-29T20:13:19.532733179+00:00"
---

## Description

`evidence list` should help reviewers find proof records without dumping hundreds of records or command transcripts by default.

## Outcome

Default `evidence list` output is bounded, states omitted counts, keeps evidence IDs visible, summarizes command-backed evidence without inline transcript firehoses, and routes full proof inspection to `evidence show <id>`. Quiet output remains useful for ID composition.
