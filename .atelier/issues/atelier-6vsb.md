---
created_at: "2026-06-13T00:56:48.121711594+00:00"
id: "atelier-6vsb"
issue_type: "task"
labels:
- "lint"
- "reliability"
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
title: "Repair rebuild temp-file lint fixture"
updated_at: "2026-06-13T00:56:48.121711594+00:00"
---

## Description

Repair the broad-suite failure in
`test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor`. The
fixture intended to prove that query, lint, export, and doctor ignore rebuild
temp files, but its temporary issue Evidence section used placeholder-style
wording that was correctly rejected by lint before the temp-file assertions
could complete.

## Outcome

- The fixture issue uses concrete Evidence wording that names the temp-file
  regression test and command checks.
- The regression test confirms query, lint, export, and doctor ignore rebuild
  temp files without reporting the rebuild temp path.
- The default broad-suite fail-fast probe advances past the previous temp-file
  lint fixture failure.

## Evidence

- `cargo nextest run test_rebuild_temp_files_are_ignored_by_query_lint_export_and_doctor` passes.
- `cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet` advances past the previous temp-file lint failure.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
