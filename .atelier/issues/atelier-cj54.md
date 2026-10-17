---
created_at: "2026-10-17T01:36:20.904463567+00:00"
id: "atelier-cj54"
issue_type: "feature"
labels:
- "issue-commands"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let issue update replace one body section"
updated_at: "2026-10-17T01:36:20.904463567+00:00"
---

## Goal

`atelier issue update --body` replaces every typed section, so agents rewrite a single section by script. On 2026-10-16 there were about 20 Python `str.replace` or section-splice edits of `.atelier/issues/*.md`, changing a Done when, a Design section, a Close Reason or one Scope line. One script split on the first `## Context` and duplicated sections, which `evidence record` then refused: "Duplicate issue body section 'Context'". Editing records directly stays supported for bulk work. This item covers the one-section case.

## Done when

- `atelier issue update <id> --section "Done when"` replaces only that section. The text comes from stdin or a file, so Markdown never passes through shell quoting. The result is validated like `atelier check`.
- An unknown or disallowed section is refused, and the error names the allowed sections for the record's type.
- Notes stay append-only through `issue note`. Done records follow the existing rules for editing a completion claim (atelier-jy9t).
- The confirmation follows atelier-7ebi. Tests cover a replace, an unknown section, and a done record.

## Context

From the 2026-10-16 transcript retrospective.
