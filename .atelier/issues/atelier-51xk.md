---
created_at: "2026-06-12T01:55:00.088845078+00:00"
id: "atelier-51xk"
issue_type: "task"
labels:
- "cli"
- "ergonomics"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T03:35:43.788915385+00:00"
status: "done"
title: "Remove large text-field editing from issue update"
updated_at: "2026-06-12T03:35:43.788915385+00:00"
---

## Description

Remove large body-field editing from `atelier issue update`. Long descriptions,
rich acceptance criteria, notes, and similar fields should be edited directly in
the canonical Markdown issue file so agents can use robust file-editing tools
instead of fragile shell quoting.

## Outcome

- `issue update` no longer accepts description/body-style updates for rich
  issue text.
- `issue update --help` explains that long-form issue text should be edited in
  `.atelier/issues/<id>.md` and validated with `atelier lint <id>`.
- Small metadata and state changes remain available through `issue update`
  where appropriate, such as title, status, priority, type, labels, parent, and
  claim.
- Documentation and Agent Factory guidance stop recommending shell-quoted
  description updates.
- Transcript tests cover the removed option, the help guidance, and the
  retained metadata-update path.

## Evidence

Evidence was not specified in the legacy issue record.
