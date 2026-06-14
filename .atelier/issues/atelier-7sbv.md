---
created_at: "2026-06-14T05:58:14.483925218+00:00"
id: "atelier-7sbv"
issue_type: "task"
labels:
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Move blocker commands under issue"
updated_at: "2026-06-14T05:58:14.483925218+00:00"
---

## Description

Replace the top-level dep command with issue-owned blocker commands so blocking relationships remain conceptually attached to issues.

## Outcome

Blocking relationships are managed through issue-owned commands rather than
the top-level `dep` command. Help, docs, and command errors make blockers feel
like issue readiness relationships, not a separate dependency domain.

## Evidence

- `atelier issue --help` shows blocker add/remove or equivalent issue-owned
  blocker operations.
- Root help no longer exposes `atelier dep` as the normal blocker surface.
- Focused CLI tests cover adding, removing, listing, and rejecting invalid
  issue blockers through the issue command family.
- Search transcript shows normal docs and Agent Factory guidance do not
  recommend `atelier dep`.
- `git diff --check`, `atelier lint`, and focused CLI tests pass.
