---
created_at: "2026-06-19T03:59:03.088244618+00:00"
id: "atelier-q199"
issue_type: "validation"
labels:
- "review"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Run review mission closeout checks"
updated_at: "2026-06-19T03:59:03.088244618+00:00"
---

## Description

Run mission closeout validation after the docs, schema, config, room backend,
and provider rename epics have their own evidence.

## Outcome

- Final repository checks cover formatting, focused regression tests, full test
  suite, tracker lint, doctor, and Markdown whitespace.
- Docs/help parity is checked for `review` commands, wrong-mode guidance,
  `atelier pr` removal, and review field terminology.
- Mission status shows linked work complete, blockers clear, evidence attached,
  and terminal checks passing or explicitly justified.

## Evidence

- Recorded command output for `cargo fmt -- --check`, focused tests,
  `cargo nextest run`, extended ignored tests when applicable,
  `git diff --check`, `atelier lint`, and `atelier doctor`.
- Search output for active `atelier pr`, legacy `pull_request`, and review mode
  terminology is attached or summarized.
- `atelier mission status atelier-zief --verbose` output is attached to the
  validation evidence.
