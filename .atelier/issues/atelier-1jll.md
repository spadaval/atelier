---
created_at: "2026-10-16T18:26:41.060859501+00:00"
id: "atelier-1jll"
issue_type: "feature"
labels:
- "adoption"
workflow_branch:
  branch_base: task/atelier-1jll
  work_branch: feature/atelier-1jll
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T19:42:17.760171581+00:00"
status: "done"
title: "Human-only checks under a parent hold its branch off the base"
updated_at: "2026-10-16T19:42:17.760171581+00:00"
---

## Goal

A parent owns one branch, and `atelier merge` needs the parent and every scoped child terminal. When a child is a `needs-human` validation (a game, headset or Windows check that runs on the released build), the agent-doable children can all be done and reviewed while the branch still cannot merge until the human acts. The natural record shape (feature parent with its game check as a child) deadlocks integration.

## Done when

Either a parent can integrate while its `needs-human` validation children are open (they stay open and keep the parent from closing, or they move to a post-merge state), or `atelier check`/`start` warns when a `needs-human` item is a child of a branch owner and the docs say to model it as a standalone validation `blocked_by` the parent. Documented in the work-model doc.

## Context

Found 2026-10-16 orchestrating the astromech backlog. 15 parents (EDDN sender, Ships page, v1.0 UI refinement, onboarding and others) each had a needs-human game check as a child. Worked around in astromech b361b4a by detaching each check, linking it `blocked_by` its parent and `advances` the parent mission, and rewording the parent Done when to say the check runs after the merge.

## Close Reason

Took the warn-and-document option: atelier start and atelier prime warn when a branch owner has an open needs-human child, explain that atelier merge needs every child terminal, and give the commands to detach it as a standalone validation blocked_by the parent (issue update --no-parent, issue link --role blocked_by). The work-model doc says to model post-merge human checks that way. Test: start_and_prime_warn_when_a_branch_owner_has_a_needs_human_child, which also follows the advice.
