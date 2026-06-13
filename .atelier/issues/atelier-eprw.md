---
created_at: "2026-06-11T20:10:54.799132612+00:00"
id: "atelier-eprw"
issue_type: "task"
labels:
- "errors"
- "lint"
- "projection"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-g9fd"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-ybcj"
    role: "validates"
  relates:
  - kind: "issue"
    id: "atelier-fx9r"
    type: "related"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-11T23:51:07.978213441+00:00"
status: "done"
title: "Fail projection queries clearly when canonical Markdown is invalid"
updated_at: "2026-06-11T23:51:07.978213441+00:00"
---

## Description

Ensure invalid canonical Markdown blocks query refresh and produces actionable guidance to run atelier lint, without falling back to stale projection rows. Acceptance: tests cover invalid records with existing stale state.db.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.
