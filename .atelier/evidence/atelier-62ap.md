---
created_at: "2026-07-16T23:51:18.364767996+00:00"
id: "atelier-62ap"
evidence_type: "test"
captured_at: "2026-07-16T23:51:15.075524124+00:00"
command: "env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run -p atelier-cli -E 'test(test_man_plan_review_roles_explain_independent_handoff) or test(public_plan_review_surface_records_findings_resolutions_changes_and_approval) or test(exact_revision_approval_and_closed_dependencies_allow_ready_and_start) or test(review_approval_does_not_override_open_dependency_blockers) or test(ready_work_hides_unreviewed_and_transitively_blocked_mission_work)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-qi40"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-qi40"
    role: "validates"
  - kind: "issue"
    id: "atelier-t876"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run -p atelier-cli -E 'test(test_man_plan_review_roles_explain_independent_handoff) or test(public_plan_review_surface_records_findings_resolutions_changes_and_approval) or test(exact_revision_approval_and_closed_dependencies_allow_ready_and_start) or test(review_approval_does_not_override_open_dependency_blockers) or test(ready_work_hides_unreviewed_and_transitively_blocked_mission_work)'"
updated_at: "2026-07-17T00:14:28.891758036+00:00"
---

## Summary

env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run -p atelier-cli -E 'test(test_man_plan_review_roles_explain_independent_handoff) or test(public_plan_review_surface_records_findings_resolutions_changes_and_approval) or test(exact_revision_approval_and_closed_dependencies_allow_ready_and_start) or test(review_approval_does_not_override_open_dependency_blockers) or test(ready_work_hides_unreviewed_and_transitively_blocked_mission_work)'

## Command

```console
env CARGO_TARGET_DIR=/root/atelier-worktrees/atelier-p2wk/target cargo nextest run -p atelier-cli -E 'test(test_man_plan_review_roles_explain_independent_handoff) or test(public_plan_review_surface_records_findings_resolutions_changes_and_approval) or test(exact_revision_approval_and_closed_dependencies_allow_ready_and_start) or test(review_approval_does_not_override_open_dependency_blockers) or test(ready_work_hides_unreviewed_and_transitively_blocked_mission_work)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```
## Stderr

Bytes: 1206
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-h3wg/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.38s
────────────
 Nextest run ID 07d86f36-5533-407a-90d7-96b9506b007e with nextest profile: default
    Starting 5 tests across 4 binaries (527 tests skipped)
        PASS [   0.147s] (1/5) atelier-cli::cli_integration setup_guidance::test_man_plan_review_roles_explain_independent_handoff
        PASS [   0.418s] (2/5) atelier-cli::cli_integration mission_plan_review_lifecycle::review_approval_does_not_override_open_dependency_blockers
        PASS [   0.418s] (3/5) atelier-cli::cli_integration mission_plan_review_lifecycle::public_plan_review_surface_records_findings_resolutions_changes_and_approval
        PASS [   0.458s] (4/5) atelier-cli::cli_integration mission_plan_review_lifecycle::ready_work_hides_unreviewed_and_transitively_blocked_mission_work
        PASS [   0.502s] (5/5) atelier-cli::cli_integration mission_plan_review_lifecycle::exact_revision_approval_and_closed_dependencies_allow_ready_and_start
────────────
     Summary [   0.503s] 5 tests run: 5 passed, 527 skipped
```
