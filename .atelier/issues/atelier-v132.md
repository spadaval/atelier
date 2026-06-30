---
created_at: "2026-06-29T17:39:54.810945279+00:00"
id: "atelier-v132"
issue_type: "task"
labels:
- "cli"
- "mission"
- "planning"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Retire mission section authoring flags"
updated_at: "2026-06-29T17:39:54.810945279+00:00"
---

## Description

Mission create/update no longer promotes --constraint, --risk, or --validation as the normal authoring path. The command surface should guide users toward plain Outcome-centered mission Markdown instead of structured paperwork fields.

## Outcome

Mission authoring commands stop presenting `--constraint`, `--risk`, and `--validation` as the normal way to create mission plans. Help text and behavior guide users toward plain mission Markdown centered on `Outcome`.
