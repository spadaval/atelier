---
created_at: "2026-06-30T16:09:44.062306307+00:00"
id: "atelier-qu06"
issue_type: "epic"
labels:
- "git"
- "workflow"
review:
  kind: pull_request
  number: 36
  provider: forgejo
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
status: "validation"
title: "Epic: Implement Git workflow validators and actions for mission branches"
updated_at: "2026-06-30T18:43:06.209884234+00:00"
---

## Description

Implement workflow policy and CLI execution changes for Git validators and actions that support mission integration branches.

## Outcome

- Git-related workflow names use the `git.*` namespace: `git.on_base`, `git.on_mission_branch`, `git.worktree_clean`, `git.prepare_branch`, `git.push`, and `git.sync`.
- `git.on_mission_branch` verifies that the current branch is `mission/<id>`, that `<id>` is a mission issue, and that the transition target is scoped by that mission through `advances` links plus descendants.
- `git.prepare_branch` creates or checks out the target work branch using canonical `<issue_type>/<issue_id>` naming.
- `git.prepare_branch: current` creates the work branch from the current branch and records that branch as the branch base.
- Parser, planner, transition option output, and execution paths handle scalar action parameters without introducing a generic branch selector language.

## Evidence

- Focused workflow-policy parser tests cover accepted and rejected Git action/validator syntax.
- CLI transition tests cover mission start, epic start from mission branch, wrong-branch rejection, and action output.
- `target/debug/atelier check atelier-qu06` passes.
