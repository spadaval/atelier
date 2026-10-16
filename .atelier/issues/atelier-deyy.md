---
created_at: "2026-10-16T23:04:58.562869688+00:00"
id: "atelier-deyy"
issue_type: "task"
labels:
- "orientation"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-7u4o"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Tighten work views to one dense line per item"
updated_at: "2026-10-16T23:43:14.107499665+00:00"
---

## Goal

Apply the ADR 0022 output principle to the `work` views and write it into the output contract: `work mission` prefixes every done row with "shown for context", and rows repeat category and status tokens that the section header already states.

## Done when

- `work mission`, `work parent`, `work all` and `status` drop per-row "shown for context" and tokens the section header already says, mark state with symbols that read without a legend (✓ done, ⚠ needs attention) next to the status word, and keep every fact they show today. Rendering tests are updated to the new output.
- `docs/product/human-cli-output.md` states the dense-output rule (one line per item, symbols with the status word, no tokens the header states, brief detail views with a full flag, no JSON or YAML modes for views) and the rule that routine piping of atelier output is filed as a missing view.

## Scope

Work-view rendering, its tests, and `human-cli-output.md`. No change to classification or counts.

## Context

ADR 0022; output observed on master 2026-10-16.
