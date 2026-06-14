---
created_at: "2026-06-12T04:51:47.082080848+00:00"
id: "atelier-uibk"
issue_type: "task"
labels:
- "assignee:root"
- "markdown"
- "migration"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-n1ys"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-nw8j"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T05:22:45.271218408+00:00"
status: "done"
title: "Migrate existing issue records to sectioned Markdown"
updated_at: "2026-06-12T05:22:45.271218408+00:00"
---

## Description

Repair existing canonical issue records so the repository can enable strict
section lint without immediately failing on historical tracker shape.
- Existing issue bodies are migrated or repaired into the sectioned Markdown
  format selected by the contract.
- Generated edits are deterministic, reviewable, and avoid inventing false
  outcomes for issues whose intent is unclear.
- Issues that cannot be safely migrated are explicitly identified for human
  cleanup rather than silently padded with meaningless text.
- The migration preserves titles, statuses, labels, relationships, notes,
  activity sidecars, and evidence links.
- The migration removes issue-level YAML `acceptance` and `evidence_required`
  arrays once the parser/schema supports sectioned Markdown.
- After migration, strict issue-section lint can run across the repository.
- Add migration tests or fixture-based checks for legacy empty bodies, plain
  description bodies, existing Acceptance Criteria prose, and already-sectioned
  issues.
- Run the migration or repair path on this repository and inspect the diff.
- Run `atelier lint` after migration enforcement is enabled.
- Run `atelier export --check`.
This work may need a dry-run/report mode before making broad edits. The outcome
should avoid low-value generated boilerplate that lets agents close work without
understanding it.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
