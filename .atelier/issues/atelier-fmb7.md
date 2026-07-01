---
created_at: "2026-06-29T20:16:19.141277804+00:00"
id: "atelier-fmb7"
issue_type: "task"
labels:
- "command-audit"
- "complexity"
- "docs"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-01T06:02:11.286090417+00:00"
status: "done"
title: "Collapse retired command pages to tombstones"
updated_at: "2026-07-01T06:02:11.286090417+00:00"
---

## Description

Retired command audit pages should not preserve enough detail to look like supported command manuals. They should explain that the surface is retired and point to the current replacement when one exists.

## Outcome

Retired command pages under docs/product/command-audit are reduced to compact tombstones or history notes. They identify the current replacement or removal decision, avoid full command walkthroughs, and no role guide treats retired commands as supported.
