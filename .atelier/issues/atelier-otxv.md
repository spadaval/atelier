---
created_at: "2026-06-30T16:09:55.493794454+00:00"
id: "atelier-otxv"
issue_type: "epic"
labels:
- "git"
- "review"
review:
  kind: pull_request
  number: 38
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-sszj
    integration_target: mission/atelier-sszj
    merge_strategy: squash
    owner_issue_id: atelier-otxv
    owner_kind: epic
    review_target: mission/atelier-sszj
    work_branch: epic/atelier-otxv
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fs79"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T19:15:33.089267491+00:00"
status: "done"
title: "Epic: Retarget review and sync to recorded branch bases"
updated_at: "2026-06-30T19:15:33.089267491+00:00"
---

## Description

Retarget provider-backed review, merge, and sync behavior so review artifacts can target the recorded branch base instead of always targeting the configured base branch.

## Outcome

- Review-open/provider code can create or reuse an epic review artifact targeting the recorded mission branch.
- Review merge validates source and target against recorded branch context.
- `git.sync` fast-forwards the resolved review or integration target from the remote without switching worktrees when Git can do that safely.
- If the target branch is checked out in another worktree or cannot be safely fast-forwarded, the action fails with clear recovery guidance rather than mutating refs unsafely.

## Evidence

- Provider or local-room tests prove epic review targets `mission/<mission-id>` when branch base is a mission branch.
- Sync tests cover no-checkout fast-forward and checked-out-target failure guidance.
- `target/debug/atelier check atelier-otxv` passes.
