---
created_at: "2026-06-30T16:10:07.073285491+00:00"
id: "atelier-fs79"
issue_type: "validation"
labels:
- "git"
- "validation"
- "workflow"
review:
  kind: pull_request
  number: 41
  provider: forgejo
fields:
  workflow_branch:
    branch_base: master
    integration_target: master
    merge_strategy: squash
    owner_issue_id: atelier-fs79
    owner_kind: issue
    review_target: master
    work_branch: validation/atelier-fs79
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
  - kind: "issue"
    id: "atelier-sszj"
    type: "validates"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T20:18:12.942829369+00:00"
status: "done"
title: "Validate mission integration branch workflow end to end"
updated_at: "2026-06-30T20:18:12.942829369+00:00"
---

## Description

Independently validate the mission integration branch workflow after implementation.

## Outcome

- An independent validator maps the mission Outcome to concrete behavior and records pass, fail, blocked, or deferred for each claim.
- The validation covers opt-in behavior, canonical branch naming, mission-scope validation through `advances`, durable branch base state, provider or local review target behavior, Git sync safety, docs/help parity, and non-mission workflow compatibility.
- End-to-end scenario proof covers mission start, epic start from mission branch, wrong-branch rejection, epic close to mission branch, and mission close to configured base branch.

## Evidence

- Evidence records include command transcripts or file references for each validated claim.
- Baseline checks include focused tests, `cargo fmt -- --check`, `git diff --check`, and `target/debug/atelier check`.
