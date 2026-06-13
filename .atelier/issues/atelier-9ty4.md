---
created_at: "2026-06-13T00:42:42.717322890+00:00"
id: "atelier-9ty4"
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
closed_at: "2026-06-13T01:17:13.275356336+00:00"
status: "done"
title: "Repair command-result lint fixture"
updated_at: "2026-06-13T01:17:13.275356336+00:00"
---

## Description

Repair the broad-suite failure in
`test_command_result_json_mode_is_rejected_and_human_subset_works`. The test
creates temporary fixture issues and then expects `atelier lint` to pass, but
the fixture issues use placeholder body sections that are correctly rejected by
the current proof discipline.

## Outcome

- The command-result fixture creates issue bodies with concrete Description,
  Outcome, and Evidence sections.
- `atelier lint` passes inside the fixture while the test still verifies that
  human command-result surfaces do not emit JSON payloads.
- The default broad-suite fail-fast probe advances past the previous
  command-result fixture failure.

## Evidence

- `cargo nextest run test_command_result_json_mode_is_rejected_and_human_subset_works` passes.
- `cargo nextest run --status-level fail --final-status-level fail --failure-output final --max-fail 1:immediate --cargo-quiet` advances past the previous command-result fixture failure.
- `atelier lint`, `atelier export --check`, and `git diff --check` pass.
