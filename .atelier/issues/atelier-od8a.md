---
created_at: "2026-06-14T03:47:12.542064132+00:00"
id: "atelier-od8a"
issue_type: "task"
labels:
- "assignee:root"
- "cli"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Add mission unlink repair command"
updated_at: "2026-06-14T07:29:37.604492759+00:00"
---

## Description

There is no normal command to remove an issue from mission linked work after an
accidental `mission add-work`. Add `atelier mission unlink <mission-id>
<issue-id>` as the record-specific repair path.

## Outcome

Operators can repair accidental mission work links with `atelier mission
unlink` without manual canonical Markdown edits or generic `atelier link`
dead ends.

## Evidence

- CLI help shows `atelier mission unlink <mission-id> <issue-id>`.
- Focused test covers `mission add-work` followed by `mission unlink`.
- `atelier mission show` no longer lists the unlinked issue.
- `atelier link` is not used as the mission relationship repair path.
- `git diff --check` passes.
