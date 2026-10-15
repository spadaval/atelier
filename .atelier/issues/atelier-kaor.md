---
created_at: "2026-10-15T22:08:58.911071017+00:00"
id: "atelier-kaor"
issue_type: "task"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make the issue CLI's options say what they do"
updated_at: "2026-10-15T22:08:58.911071017+00:00"
---

## Goal

Several `atelier issue` options mislead or are missing.

## Done when

Each item is fixed or deliberately removed, with help text matching behaviour: (1) `issue update --status` is documented for missions but refuses every status change in favour of `issue transition`; (2) `--parent` mentions an "imported source ID", left over from the old import model; (3) `issue create` has both `--description` and `--body` with overlapping meanings; (4) `issue update --body` works only on missions, so epic and issue bodies can only be edited by hand; (5) bundles say `depends_on`/`blocks` while the CLI says `--role blocked_by`; pick one vocabulary; (6) there are two ways to start work: `atelier start` creates the owner branch, while `issue transition <id> start` moves the record to in_progress on any branch without one. Either document when to use each or make the transition refuse when a branch is needed; (7) the link role is `related` but the record field is `relates`, and `issue link` offers 14 roles with no help text saying which ones affect readiness.

## Context

Found 2026-10-15 while adopting Atelier in astromech (branch chore/atelier-tracker). Item 6 reproduced in a scratch repo: a spike went todo → in_progress → done on the base branch with `issue transition`. Smaller edges already listed in atelier-v44v.
