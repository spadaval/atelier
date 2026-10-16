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
updated_at: "2026-10-16T23:04:58.562869688+00:00"
---

## Goal

Apply the ADR 0022 output principle to the `work` views: `work mission` prefixes every done row with "shown for context", and rows repeat category and status tokens that the section header already states.

## Done when

`work mission`, `work parent`, `work all` and `status` drop per-row "shown for context" and tokens the section header already says, mark state with symbols that read without a legend (✓ done, ⚠ needs attention) next to the status word, and keep every fact they show today. Snapshot or rendering tests are updated to the new output.

## Scope

Work-view rendering and tests. No change to classification or counts.

## Context

`docs/product/human-cli-output.md`; output observed on master 2026-10-16.
