---
created_at: "2026-10-16T00:01:32.982045465+00:00"
id: "atelier-bibd"
issue_type: "bug"
labels:
- "adoption"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Decide an install branch's base from outside the branch"
updated_at: "2026-10-16T00:01:32.982045465+00:00"
---

## Goal

`land` treats an unowned branch as installing Atelier when no copy of its base has a workflow policy. The base name comes from the branch's own working-tree policy, which the branch can edit, so a branch that points `base_branch` at a pre-adoption branch passes `land` with install guidance instead of `land_work_untraceable`.

## Done when

A branch counts as installing Atelier only when the policy it reads was introduced by commits unique to that branch (or another rule that the branch cannot satisfy by editing its own policy), with a test for the edited-base case.

## Context

Review finding atelier-jsd4 on atelier-40i6, deferred: it needs a deliberate policy edit, and `atelier merge` still needs an owner, so no code reaches the base this way.
