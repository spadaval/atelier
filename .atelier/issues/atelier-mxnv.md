---
created_at: "2026-06-29T20:15:16.003270031+00:00"
id: "atelier-mxnv"
issue_type: "task"
labels:
- "branch"
- "cli"
- "complexity"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Route branch recovery only from failed workflow guidance"
updated_at: "2026-06-29T20:15:16.003270031+00:00"
---

## Description

The branch command family exposes recovery machinery as if it were ordinary workflow. The complexity budget says this should be available only when a branch or transition failure needs it.

## Outcome

Branch recovery commands are hidden, folded, or removed from normal help and role guides. Failed transition, checkout, or check guidance still gives a concrete recovery path when branch state blocks work, and regression coverage proves normal command listings do not promote recovery plumbing.
