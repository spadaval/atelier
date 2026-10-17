---
created_at: "2026-10-17T08:25:02.589462877+00:00"
id: "atelier-w4ef"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Make check and check <id> agree on missing required sections"
updated_at: "2026-10-17T08:25:02.589462877+00:00"
---

## Goal

Found while implementing atelier-pr4k: `atelier check` passes while `atelier check <id>` fails for an issue with no Goal or Done when sections, so the repository-wide check misses a finding the scoped one reports. Separately, `docs/architecture/quality/agent-factory-atelier-validation.md` mentions `atelier check --fix`, which does not exist.

## Done when

- `check` and `check <id>` report the same findings for the same record.
- The doc names only commands that exist.
- A test covers an issue missing its required sections under both forms.
