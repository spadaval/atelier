---
created_at: "2026-07-06T17:20:25.693166412+00:00"
id: "atelier-dy3u"
issue_type: "feature"
labels:
- "cli"
- "color"
- "human-output"
- "mission-dashboard"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-sjsz"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-07-07T06:09:55.609121312+00:00"
status: "done"
title: "Render the Mission Overview with shared formatting and color"
updated_at: "2026-07-07T06:09:55.609121312+00:00"
---

## Description

Render the Mission Overview through the shared page, panel, row, and style abstractions instead of the generic issue-list path. Show clear mission headings, indented epic rows, textual state and progress, blocker and omission cues, and focused drill-down commands. Apply the existing automatic terminal color policy and semantic display roles; do not add command-local ANSI logic.

## Outcome

- Interactive `atelier work missions` output uses the same semantic colors and structural grammar as other work panels, while noninteractive and `NO_COLOR` output contains no ANSI escapes and loses no meaning.
- Mission and epic hierarchy, status, priority, progress, blockers, hidden-child counts, omitted-row counts, and next commands remain legible at normal and narrow terminal widths.

## Evidence

- Renderer snapshot and color/plain semantic-parity test transcript: `atelier-9mto`.
- Temporary-repository `atelier work missions` CLI transcript covering hierarchy, blocker-aware progress, direct and unassigned accounting, `NO_COLOR`, and quiet output: `atelier-w2py`.
- Review-response transcript covering shared width-aware rows and `COLUMNS=40` rendering: `atelier-ud6t`.
- Adversarial review-response transcript covering a 100-character unbroken title at `COLUMNS=40`: `atelier-7v8q`.
