---
created_at: "2026-10-15T22:08:58.898598939+00:00"
id: "atelier-fxbg"
issue_type: "bug"
labels:
- "adoption"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Let land pass on branches that no item owns"
updated_at: "2026-10-15T22:08:58.898598939+00:00"
---

## Goal

The Claude Stop hook runs `atelier land` at every stop. On any branch not created by `atelier start` (a feature branch, a PR branch, a branch from before adoption), `land` exits 1 with `land_work_untraceable`, even when the branch changes no records. Once a repo installs the hooks, every session on such a branch errors at each stop.

## Done when

`land` on a branch with no owner and no record changes exits 0 with a one-line note; a branch with no owner that does change records still gets the current error with guidance. Tests cover both.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Reproduced in a scratch repo: `land` passes on the base branch and fails on any other unowned branch. Related: atelier-40i6 (the install commit itself).
