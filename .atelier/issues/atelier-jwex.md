---
created_at: "2026-08-05T18:46:52.084817342+00:00"
id: "atelier-jwex"
issue_type: "task"
labels:
- "review-finding"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Unify readiness and retire superseded descendants"
updated_at: "2026-08-05T18:46:52.084817342+00:00"
---

## Description

Repair the blocking PR 77 findings that normal views disagree about blocker-free `blocked` work and that descendants of a superseded epic remain executable. Put the classification behind one domain rule and preserve completed descendant history while suppressing open speculative descendants from ready selection.

## Outcome

- Status, global work views, mission views, and epic views classify explicit blocked states consistently.
- Open descendants of a terminal or superseded ancestor cannot enter ready projections; completed descendants and evidence remain inspectable.
- Cross-surface CLI regressions cover both cases.

## Evidence

- Fresh global, mission, and epic view scenarios agree on blocked and superseded-descendant classifications.
