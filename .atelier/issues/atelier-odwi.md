---
created_at: "2026-06-29T20:12:16.280861616+00:00"
id: "atelier-odwi"
issue_type: "task"
labels:
- "cli"
- "review"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Make review open derive routine provider fields from issue context"
updated_at: "2026-06-29T20:12:16.280861616+00:00"
---

## Description

Routine `review open` should not require operators to manually provide provider plumbing that Atelier already knows from the issue, branch policy, workflow action, or configured provider.

## Outcome

`review open` has an issue-derived routine path. It derives or clearly reports missing issue, source branch, target branch, title, body, provider, and role context. Fully manual provider fields are hidden, advanced, or explicitly documented as recovery-only.
