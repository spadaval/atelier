---
created_at: "2026-06-29T20:12:45.490635701+00:00"
id: "atelier-yysm"
issue_type: "epic"
labels:
- "cli"
- "complexity"
- "evidence"
- "history"
review:
  kind: pull_request
  number: 46
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-durs
    integration_target: mission/atelier-durs
    merge_strategy: squash
    owner_issue_id: atelier-yysm
    owner_kind: epic
    review_target: mission/atelier-durs
    work_branch: epic/atelier-yysm
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
status: "review"
title: "Epic: Simplify evidence and history browse surfaces"
updated_at: "2026-07-06T18:01:36.572085977+00:00"
---

## Description

Evidence and history are first-class records, but their browse surfaces can exceed the human information budget or duplicate relationship/query behavior. Apply the complexity budget to keep proof and activity inspectable without transcript firehoses or scoped query sprawl.

## Outcome

Evidence and history surfaces stay bounded and purposeful. `evidence record`, `evidence show`, and `evidence list` have clear jobs; `evidence attach` is folded or justified; history remains a bounded activity reader instead of a second search/query language; docs and tests prove default output does not dump raw transcripts or unbounded timelines.
