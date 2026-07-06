---
created_at: "2026-06-29T18:20:16.722514121+00:00"
id: "atelier-qo3w"
issue_type: "task"
labels:
- "cli"
- "issues"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T04:07:36.832723749+00:00"
status: "done"
title: "Add generic issue list inventory command"
updated_at: "2026-06-30T04:07:36.832723749+00:00"
---

## Description

Add `atelier issue list` as the generic issue inventory command. It is an inventory/listing surface for issue records, not the complex mission dashboard and not a replacement for `issue show` record detail.

## Outcome

`atelier issue list` lists all issue records across statuses by default and supports `--status <status>`, `--category <category>`, `--issue-type <type>`, `--label <label>`, `--priority <priority>`, `--ready`, `--blocked`, and `--quiet`. The implementation reuses the existing queue read model and renderer instead of introducing another storage path, and shared filtering supports issue-type filtering without post-render hacks.
