---
created_at: "2026-06-13T03:07:33.292197602+00:00"
id: "atelier-oezx"
issue_type: "bug"
labels:
- "cli"
- "ux"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T16:04:04.851089621+00:00"
status: "done"
title: "Unify issue creation type and template options"
updated_at: "2026-06-13T16:04:04.851089621+00:00"
---

## Description

Issue creation exposes two overlapping classification mechanisms: `--template` (bug, feature, refactor, research) and `--issue-type` (bug, closeout, epic, feature, spike, task, validation). The implementation also changes `--template` behavior when `--parent` or `--issue-type` is present, which makes template defaults inconsistent. This should become one clear creation model where a human or agent can choose the work type once and get predictable defaults.

## Outcome

- `issue create --help` presents one coherent way to choose work type and optional body scaffolding; users do not have to understand both template names and issue types for ordinary work.
- Supplying a parent or explicit type does not silently disable expected template defaults such as label, priority, or description prefix.
- Bug, feature, task, validation, spike, epic, and closeout creation behavior is documented in one place with clear defaults.
- Invalid or conflicting creation options fail with actionable guidance instead of producing surprising records.

## Evidence

- Focused CLI tests or transcripts cover representative create commands with and without parents, including bug and validation work.
- Help/docs parity review shows the documented creation model matches `atelier issue create --help`.
- Tracker fixture or file review proves created canonical records have expected issue_type, labels, priority, parent, and body sections.
- `atelier lint`, `atelier export --check`, and relevant CLI tests pass.

## Notes

Observed while filing this issue: passing section headings such as `## Outcome`
through `issue create --description` can create duplicate canonical body
sections and leave projection refresh failed until the Markdown is repaired.
The simplified creation model should either accept a full sectioned body
deliberately or reject sectioned descriptions with actionable guidance.
