---
created_at: "2026-10-15T22:08:58.898598939+00:00"
id: "atelier-fxbg"
issue_type: "bug"
labels:
- "adoption"
workflow_branch:
  branch_base: master
  work_branch: bug/atelier-fxbg
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-40i6"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T22:56:03.678387663+00:00"
status: "done"
title: "Let land pass on branches that no item owns"
updated_at: "2026-10-15T22:56:03.678387663+00:00"
---

## Goal

The Claude Stop hook runs `atelier land` at every stop. On any branch not created by `atelier start` (a feature branch, a PR branch, a branch from before adoption), `land` exits 1 with `land_work_untraceable`, even when the branch changes no records. Once a repo installs the hooks, every session on such a branch errors at each stop.

## Done when

`land` on a branch with no owner and no record changes exits 0 with a one-line note; a branch with no owner that does change records still gets the current error with guidance. Tests cover both.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Reproduced in a scratch repo: `land` passes on the base branch and fails on any other unowned branch. Related: atelier-40i6 (the install commit itself).

## Close Reason

Unowned branches that change nothing under .atelier/ since their merge base land with a note; unowned branches that change tracker files keep land_work_untraceable with the paths and atelier start guidance. Covered by unowned_branch_lands_unless_it_changes_records, including the Stop hook.
