---
created_at: "2026-06-23T16:21:20.082763958+00:00"
id: "atelier-xa9s"
issue_type: "epic"
labels: []
review:
  kind: pull_request
  number: 53
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-mska
    integration_target: mission/atelier-mska
    merge_strategy: squash
    owner_issue_id: atelier-xa9s
    owner_kind: epic
    review_target: mission/atelier-mska
    work_branch: epic/atelier-xa9s
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-m7za"
  - kind: "issue"
    id: "atelier-qqfe"
  - kind: "issue"
    id: "atelier-u7wi"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mska"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Epic: Validate and document cache architecture rewrite"
updated_at: "2026-07-06T21:03:32.061685175+00:00"
---

## Description

Prove the persistence rewrite end to end and update docs, command audit, and terminology after implementation.

## Outcome

- The mission has end-to-end validation evidence and user-facing docs match the new architecture.

## Evidence

- Independent validation evidence `atelier-x1d1` proves a relationship-heavy
  batch left `state.db` unchanged, became detectably stale, then performed
  exactly one lazy rebuild; focused cache evidence `atelier-vj0b` passes 67/67
  tests and the repaired mission/blocker graph matched the record files.
- Independent command evidence `atelier-mfxj` covers list/show/status,
  ready/blocked, transitions, evidence, native review rooms, check/lint,
  rebuild, doctor repair, and lazy query repair; evidence `atelier-oy2z`
  records all 448 Atelier CLI tests passing.
- Evidence records `atelier-j5aa` and `atelier-y1zi` capture the targeted
  terminology audit and `target/debug/atelier check --help` transcript: retained
  projection references are historical or Mission Control-specific, while
  active repair help uses disposable runtime/domain-cache terminology.
