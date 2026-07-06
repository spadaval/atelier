---
created_at: "2026-06-29T20:12:45.490635701+00:00"
id: "atelier-yysm"
issue_type: "epic"
labels:
- "cli"
- "complexity"
- "evidence"
- "history"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-vqhi"
  children:
  - kind: "issue"
    id: "atelier-3g1y"
  - kind: "issue"
    id: "atelier-9evg"
  - kind: "issue"
    id: "atelier-ll1n"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-durs"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Simplify evidence and history browse surfaces"
updated_at: "2026-06-29T20:13:31.328877971+00:00"
---

## Description

Evidence and history are first-class records, but their browse surfaces can exceed the human information budget or duplicate relationship/query behavior. Apply the complexity budget to keep proof and activity inspectable without transcript firehoses or scoped query sprawl.

## Outcome

Evidence and history surfaces stay bounded and purposeful. `evidence record`, `evidence show`, and `evidence list` have clear jobs; `evidence attach` is folded or justified; history remains a bounded activity reader instead of a second search/query language; docs and tests prove default output does not dump raw transcripts or unbounded timelines.
