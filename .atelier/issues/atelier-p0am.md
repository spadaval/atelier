---
created_at: "2026-06-29T20:16:30.938172765+00:00"
id: "atelier-p0am"
issue_type: "task"
labels:
- "cli"
- "complexity"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove stale command references from product and role docs"
updated_at: "2026-06-29T20:16:30.938172765+00:00"
---

## Description

The audit found stale references such as old mission status/list wording, old issue status verbose forms, and old work queue filters. These references undermine the current command model and make workers follow dead paths.

## Outcome

Product docs, role guides, command audit pages, and quality docs use the current command vocabulary. Stale references are removed, rewritten to current commands, or explicitly marked retired, and a search-based check proves the known stale forms no longer appear as live guidance.
