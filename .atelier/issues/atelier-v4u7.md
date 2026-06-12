---
acceptance: []
created_at: "2026-06-12T04:51:36.967886831+00:00"
evidence_required: []
id: "atelier-v4u7"
issue_type: "task"
labels:
- "assignee:root"
- "markdown"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0j6e"
  - kind: "issue"
    id: "atelier-igzl"
  - kind: "issue"
    id: "atelier-n1ys"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement issue body section parser and rendering"
updated_at: "2026-06-12T05:03:46.768403215+00:00"
---

## Description

Implement the parser/model changes that turn known issue Markdown headings into
named sections instead of treating the whole body as one description string.

## Outcome

- Canonical issue parsing exposes recognized body sections as structured issue
  data while preserving the original Markdown body for round-trip writes.
- `atelier issue show <id>` renders recognized sections in a stable,
  scan-friendly order instead of dumping the entire body under Description.
- Unknown non-required sections are preserved and displayed without corrupting
  recognized sections.
- Existing frontmatter metadata remains compact and does not absorb large
  section bodies.
- The previous ad hoc split on the literal Acceptance Criteria heading is
  removed or replaced by the general section parser.

## Evidence

- Parser unit tests cover normal sections, unknown sections, duplicate
  recognized headings, empty sections, content before the first heading, and
  round-trip rendering.
- CLI transcript tests cover `atelier issue show <id>` for a sectioned issue.
- Run focused parser and CLI show tests.

## Notes

This task depends on the section contract. It should not enforce lint/start
failures; enforcement belongs to the lint and workflow-gate child.
