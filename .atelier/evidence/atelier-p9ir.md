---
created_at: "2026-06-12T21:24:27.610515853+00:00"
id: "atelier-p9ir"
evidence_type: "validation"
captured_at: "2026-06-12T21:24:27.610499765+00:00"
command: null
exit_status: null
path: null
uri: null
proof_scope: null
agent_identity: null
independence_level: null
follow_up_ids: []
residual_risks: []
output: null
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-n1ys"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "fail"
title: "Sectioned issue workflow validation did not close: manual CLI behavior and individual focused tests pass, but the broad focused command cargo test --test cli_integration section -- --nocapture fails in test_work_start_reports_shared_section_diagnostic_without_blocking, which still expects warning-only malformed start behavior."
updated_at: "2026-06-12T21:24:32.377838229+00:00"
---

## Result

Fail. `atelier-n1ys` remains open because the broad focused section test command
still includes a contradictory stale test for malformed work start behavior.

## Passing Product Checks

- Manual transcript used the current local binary at `target/debug/atelier` in
  `/tmp/atelier-n1ys-transcript.UA3cg0`.
- `target/debug/atelier issue show atelier-e66i` rendered `Description`,
  `Outcome`, `Evidence`, and optional `Notes` in clean sections, with start
  readiness reporting `ready`.
- `target/debug/atelier lint atelier-b7e4` exited `1` and reported
  `Missing required issue body section 'Outcome'`, `section Outcome`, and
  `.atelier/issues/atelier-b7e4.md`.
- `target/debug/atelier start atelier-b7e4` exited `1` with the same missing
  `Outcome` diagnostic.
- `target/debug/atelier workflow validate mission atelier-7a5x --validator
  no_blocking_lints` exited `1` because linked issue `atelier-b7e4` had invalid
  canonical Markdown.
- Individual focused integration tests passed:
  `test_show_issue`, `test_work_start_refuses_structurally_invalid_issue`,
  `test_issue_closeout_refuses_structurally_invalid_issue`,
  `test_workflow_validate_can_use_parsed_issue_sections`,
  `test_lint_rejects_missing_required_issue_section`, and
  `test_lint_rejects_duplicate_recognized_issue_heading`.
- Health checks passed: `cargo fmt -- --check`, `atelier export --check`,
  `atelier lint`, `atelier doctor`, and `git diff --check`.

## Blocking Failure

- Command: `cargo test --test cli_integration section -- --nocapture`.
- Result: exit `101`; `3 passed`, `1 failed`.
- Failing test:
  `test_work_start_reports_shared_section_diagnostic_without_blocking`.
- First concrete failure:
  `tests/cli_integration.rs:7280` reported missing
  `"Missing required issue body section 'Outcome'"` in the lint stderr.
- Classification: stale or contradictory test-suite expectation. The test name
  and assertions still describe warning-only malformed start behavior, while
  the sectioned issue contract and current product checks require start
  refusal.

No Rust source was edited during this validation.
