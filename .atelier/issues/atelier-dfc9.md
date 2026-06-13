---
created_at: "2026-06-13T01:02:47.940443346+00:00"
id: "atelier-dfc9"
issue_type: "task"
labels:
- "export"
- "tests"
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
title: "Repair canonical export sectioned-body expectations"
updated_at: "2026-06-13T01:02:47.940443346+00:00"
---

## Description

Repair broad-suite failures in export unit tests that still expect issue records
to end with legacy unsectioned bodies. Current canonical issue Markdown renders
Description, Outcome, and Evidence sections, including default Outcome/Evidence
placeholders when a DB issue has only a plain description.

## Outcome

- Export unit tests assert the current sectioned issue body after canonical
  rewrite and deterministic projection builds.
- The tests still verify changed canonical records are rewritten from
  projection state and Markdown serialization remains stable.
- The default broad-suite fail-fast probe advances past the previous export
  sectioned-body expectation failures.

## Evidence

- `cargo nextest run test_canonical_changed_record_export_rewrites_issue test_canonical_markdown_serialization_stability` passes.
- `cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet` advances past the previous export sectioned-body expectation failures.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
