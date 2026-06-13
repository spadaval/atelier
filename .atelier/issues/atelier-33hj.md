---
created_at: "2026-06-13T00:40:58.232341947+00:00"
id: "atelier-33hj"
issue_type: "task"
labels:
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Repair close-all proof test helper"
updated_at: "2026-06-13T00:40:58.232341947+00:00"
---

## Description

Repair the broad-suite failure in `test_close_all_is_durable_without_manual_export`.
The CLI integration helper currently registers every `atelier-...` token printed
by commands, including evidence record IDs, so numeric issue references can
resolve to non-issue records before proof attachment.

## Outcome

- CLI integration issue-reference registration only records IDs that correspond
  to canonical issue records under `.atelier/issues`.
- Evidence IDs and other non-issue record IDs printed by commands no longer
  pollute numeric issue-reference resolution.
- `test_close_all_is_durable_without_manual_export` passes alone and in the
  broad fail-fast default suite.

## Evidence

- Focused CLI integration test for close-all durable export/rebuild behavior.
- Broad default nextest fail-fast probe advances past the previous close-all
  helper failure.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
