---
created_at: "2026-07-06T20:37:49.327467717+00:00"
id: "atelier-h3wg"
issue_type: "epic"
labels:
- "agent-factory"
- "mission-review"
review:
  kind: pull_request
  number: 74
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-p4z2
    integration_target: mission/atelier-p4z2
    merge_strategy: squash
    owner_issue_id: atelier-h3wg
    owner_kind: epic
    review_target: mission/atelier-p4z2
    work_branch: epic/atelier-h3wg
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-t876"
  children:
  - kind: "issue"
    id: "atelier-l5mw"
  - kind: "issue"
    id: "atelier-qi40"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Epic: Add independent mission review to Agent Factory"
updated_at: "2026-07-16T23:47:32.217970775+00:00"
---

## Description

Give independent mission issue-set review its own bounded Agent Factory procedure, then update planning and orchestration handoffs to use the repository-owned lifecycle and command surfaces.

## Outcome

- Agent Factory routes mission-plan readiness review to a distinct mission-review subskill rather than code review, plan authorship, or outcome validation.
- Planner and orchestrator guidance requires an independent reviewer and cannot promote an authored draft without current repository-enforced approval.

## Evidence

- Agent Factory routing and dogfood transcripts show the same draft assigned separately to `plan`, `mission-review`, and `orchestrate`: the planner hands off without approval authority, the mission reviewer returns revision-bound findings or approval, and orchestration refuses state not reported ready by Atelier.
- Focused guidance inspection confirms mission-plan review is not routed through code `review` or outcome `validate`, and tactical lifecycle commands remain owned by Atelier surfaces.
- atelier-t876 independently follows the documented handoff from a fresh context and records the inspected skill locations, commands, decisions, and evidence IDs.
