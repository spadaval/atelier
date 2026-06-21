---
created_at: "2026-06-21T16:37:30.772599801+00:00"
id: "atelier-y3fj"
issue_type: "validation"
labels:
- "mission-rework"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "validation"
title: "Validate docs help and Agent Factory guidance after mission rework"
updated_at: "2026-06-21T20:11:02.837379086+00:00"
---

## Description

Check public docs, root help, role guides, command audits, and Agent Factory bindings after mission rework lands.

## Outcome

- No normal guidance teaches removed mission command forms.
- Role guides point managers and orchestrators to bundle, issue, status, evidence, history, and lint surfaces.
- Command-surface lint or targeted searches catch stale examples.

## Evidence

- Evidence record includes root help/man excerpts or bounded summaries, targeted `rg` output, and `atelier lint` result.
- Any historical references are explicitly classified as migration history or removed-command tests.
