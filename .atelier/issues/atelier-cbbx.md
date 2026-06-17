---
created_at: "2026-06-17T17:59:44.803704296+00:00"
id: "atelier-cbbx"
issue_type: "task"
labels:
- "adr"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-17T23:51:48.470181424+00:00"
status: "done"
title: "Write ADR for session-aware PR coordination boundaries"
updated_at: "2026-06-17T23:51:48.470181424+00:00"
---

## Description

Write an ADR that records the boundary decisions for this mission: durable
optional sessions, Forgejo PRs as review artifacts, typed fields in workflow
policy, and PR validators as read-only workflow gates.

## Outcome

- A new ADR explains why sessions do not replace current-work status.
- The ADR explains why PR commands do not transition Atelier workflow.
- The ADR explains why `forge_pr` is a typed field rather than an attachment.

## Evidence

- File content check names the new ADR and shows it covers sessions, PR
  artifacts, typed fields, and read-only PR validators.
- Command transcript shows `git diff --check -- docs/adr` passes.
