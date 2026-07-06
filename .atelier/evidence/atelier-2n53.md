---
created_at: "2026-06-23T21:51:45.502338530+00:00"
id: "atelier-2n53"
evidence_type: "test"
captured_at: "2026-06-23T21:51:45.502320684+00:00"
target:
  kind: "issue"
  id: "atelier-krt8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-krt8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Removed ambient expected-branch and branch-owner output from root status, issue show, and mission/objective status. Branch context now remains scoped to transition options only when branch-affecting actions are planned. Passed: cargo fmt -- --check; cargo nextest run --test cli_integration -E 'test(test_branch_lifecycle_context_surfaces)'; cargo nextest run -p atelier-cli -E 'test(branch_prepare_is_explicit_planned_action) | test(action_preflight_checks_git_actions_before_execution)'; target/debug/atelier lint atelier-c0qc; git diff --check. Live transcripts verified status/show/mission status omit Branch Policy/Expected output and non-branch krt8 options omit Branch Context."
updated_at: "2026-06-23T21:51:50.818078398+00:00"
---

Removed ambient expected-branch and branch-owner output from root status, issue show, and mission/objective status. Branch context now remains scoped to transition options only when branch-affecting actions are planned. Passed: cargo fmt -- --check; cargo nextest run --test cli_integration -E 'test(test_branch_lifecycle_context_surfaces)'; cargo nextest run -p atelier-cli -E 'test(branch_prepare_is_explicit_planned_action) | test(action_preflight_checks_git_actions_before_execution)'; target/debug/atelier lint atelier-c0qc; git diff --check. Live transcripts verified status/show/mission status omit Branch Policy/Expected output and non-branch krt8 options omit Branch Context.
