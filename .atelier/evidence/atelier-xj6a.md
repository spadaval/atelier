---
created_at: "2026-07-06T18:13:57.111260630+00:00"
id: "atelier-xj6a"
evidence_type: "test"
captured_at: "2026-07-06T18:13:51.510244301+00:00"
command: "cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yysm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yysm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Review fixes: no-summary evidence list stays under 1 KiB, reused proof remains issue-scoped, prior regressions pass"
updated_at: "2026-07-06T18:14:01.786290106+00:00"
---

## Summary

Review fixes: no-summary evidence list stays under 1 KiB, reused proof remains issue-scoped, prior regressions pass

## Command

```console
cargo nextest run -E 'test(test_issue_to_issue_validates_link_is_rejected) or test(test_evidence_relation_role_errors_are_corrective) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_history_empty_states_and_invalid_limit)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 2018
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier-worktrees/atelier-yysm/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
────────────
 Nextest run ID 8e2fb6e1-9e97-4ec7-bc5f-949d353cc52f with nextest profile: default
    Starting 12 tests across 9 binaries (711 tests skipped)
        PASS [   0.192s] ( 1/12) atelier-cli::cli_integration issues::test_history_rejects_removed_query_and_objective_scope_flags
        PASS [   0.352s] ( 2/12) atelier-cli::cli_integration records_evidence::test_evidence_list_elides_command_transcripts
        PASS [   0.363s] ( 3/12) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_no_summary_command_capture
        PASS [   0.365s] ( 4/12) atelier-cli::cli_integration issues::test_history_empty_states_and_invalid_limit
        PASS [   0.499s] ( 5/12) atelier-cli::cli_integration issues::test_issue_to_issue_validates_link_is_rejected
        PASS [   0.522s] ( 6/12) atelier-cli::cli_integration issues::test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs
        PASS [   0.810s] ( 7/12) atelier-cli::cli_integration records_evidence::test_evidence_quiet_output_is_a_stable_id_composition_path
        PASS [   0.852s] ( 8/12) atelier-cli::cli_integration issues::test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence
        PASS [   0.960s] ( 9/12) atelier-cli::cli_integration records_evidence::test_evidence_relation_role_errors_are_corrective
        PASS [   1.023s] (10/12) atelier-cli::cli_integration issues::test_history_issue_scope_excludes_other_targets_of_reused_evidence
        PASS [   2.700s] (11/12) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
        PASS [   3.371s] (12/12) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   3.373s] 12 tests run: 12 passed, 711 skipped
```
