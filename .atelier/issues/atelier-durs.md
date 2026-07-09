---
created_at: "2026-06-29T20:11:38.060404364+00:00"
id: "atelier-durs"
issue_type: "mission"
labels:
- "cli"
- "command-audit"
- "complexity"
review:
  kind: pull_request
  number: 51
  provider: forgejo
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-durs
    owner_kind: mission
    review_target: master
    work_branch: mission/atelier-durs
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-411x"
    type: "advances"
  - kind: "issue"
    id: "atelier-eqq6"
    type: "advances"
  - kind: "issue"
    id: "atelier-vqhi"
    type: "advances"
  - kind: "issue"
    id: "atelier-ye11"
    type: "advances"
  - kind: "issue"
    id: "atelier-yysm"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-06T19:12:13.111761133+00:00"
status: "publish_review"
title: "Mission: Apply command complexity budget to remaining CLI surfaces"
updated_at: "2026-07-06T19:12:13.111761133+00:00"
---

## Description

Atelier applies the command complexity budget to the remaining CLI surfaces that are outside the mission planning/dashboard workflow. Commands that mirror provider operations, duplicate relationship mutation, expose hidden recovery machinery, preserve retired vocabulary, or grow into query languages are simplified, folded, hidden, or removed.

## Outcome

The remaining command surface has explicit Keep, Simplify, Fold, Hide, or Remove decisions backed by executable behavior, docs, help, and tests. Review commands no longer mirror provider plumbing by default; evidence/history browsing stays bounded; provider, branch, maintenance, and hidden diagnostic escape hatches are routed only by explicit recovery paths; retired command docs are tombstones rather than legacy manuals; and validation proves surviving commands answer distinct operator questions within the complexity budget.
