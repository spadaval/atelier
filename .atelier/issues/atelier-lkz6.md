---
created_at: "2026-06-21T17:40:45.558707306+00:00"
id: "atelier-lkz6"
issue_type: "validation"
labels:
- "evidence"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-21T19:59:20.886578178+00:00"
status: "done"
title: "Validate workflow-driven evidence behavior end to end"
updated_at: "2026-06-21T19:59:20.886578178+00:00"
---

## Description

Independently validate that evidence is optional by default, required only by configured validators, and still useful as an optional proof artifact after the hardcoded evidence requirements are removed.

## Outcome

- A workflow without evidence.attached can move ordinary work through completion without attached evidence.
- A workflow with evidence.attached blocks the configured transition until matching evidence is attached.
- status, issue status, issue show, transition options, and blocked transitions all communicate the configured validator failure with the simple help hint.
- Evidence record, capture, attach, show, and list commands still work as optional capabilities.

## Evidence

- End-to-end command transcript covers workflows with and without evidence.attached.
- Focused test run covers validator, status/readiness, parser/lint, and evidence command behavior.
- Command-surface search shows no universal evidence requirement remains in help/docs/status text.
- atelier lint and git diff --check pass.
