---
created_at: "2026-07-06T18:43:42.755188485+00:00"
id: "atelier-bx4u"
evidence_type: "test"
captured_at: "2026-07-06T18:43:23.424509312+00:00"
command: "env HOME=/tmp/atelier-vqhi-focused-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E 'test(provider_review_open_action_reads_workflow_config_and_global_secret) or test(request_review_preserves_review_artifact_field) or test(review_surface_derives_open_context_and_uses_submit_and_show) or test(review_help_exposes_only_the_collapsed_public_contract) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_removed_maintenance_delete_is_unknown)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-vqhi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-vqhi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent integrated CLI scenarios for review, evidence/history, provider/recovery, and maintenance removal"
updated_at: "2026-07-06T18:43:48.827383256+00:00"
---

## Summary

Independent integrated CLI scenarios for review, evidence/history, provider/recovery, and maintenance removal

## Command

```console
env HOME=/tmp/atelier-vqhi-focused-home CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup cargo nextest run -E 'test(provider_review_open_action_reads_workflow_config_and_global_secret) or test(request_review_preserves_review_artifact_field) or test(review_surface_derives_open_context_and_uses_submit_and_show) or test(review_help_exposes_only_the_collapsed_public_contract) or test(test_evidence_capture_records_command_metadata_and_attaches_targets) or test(test_evidence_list_elides_command_transcripts) or test(test_evidence_list_bounds_no_summary_command_capture) or test(test_evidence_list_bounds_default_output) or test(test_evidence_quiet_output_is_a_stable_id_composition_path) or test(test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs) or test(test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence) or test(test_history_issue_scope_excludes_other_targets_of_reused_evidence) or test(test_history_rejects_removed_query_and_objective_scope_flags) or test(test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery) or test(test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly) or test(test_removed_maintenance_delete_is_unknown)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3893
Truncated: no

```text
   Compiling bitflags v2.10.0
   Compiling getrandom v0.3.4
   Compiling num-traits v0.2.19
   Compiling rustix v1.1.3
   Compiling linux-raw-sys v0.11.0
   Compiling fastrand v2.3.0
   Compiling regex-syntax v0.8.8
   Compiling zerocopy v0.8.31
   Compiling bit-vec v0.8.0
   Compiling wait-timeout v0.2.1
   Compiling fnv v1.0.7
   Compiling quick-error v1.2.3
   Compiling unarray v0.1.4
   Compiling arbitrary v1.4.2
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-cli)
   Compiling rusqlite v0.38.0
   Compiling bit-set v0.8.0
   Compiling rand_core v0.9.3
   Compiling rand v0.9.4
   Compiling rand_xorshift v0.4.0
   Compiling chrono v0.4.42
   Compiling tempfile v3.24.0
   Compiling regex-automata v0.4.14
   Compiling ppv-lite86 v0.2.21
   Compiling atelier-core v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-core)
   Compiling rusty-fork v0.3.1
   Compiling rand_chacha v0.9.0
   Compiling proptest v1.9.0
   Compiling atelier-workflow v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-records)
   Compiling matchers v0.2.0
   Compiling tracing-subscriber v0.3.23
   Compiling atelier-sqlite v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/.codex/worktrees/0cff/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 15.81s
────────────
 Nextest run ID ce60edde-2cb1-4b95-8968-89bf9772e2b0 with nextest profile: default
    Starting 16 tests across 9 binaries (701 tests skipped)
        PASS [   0.055s] ( 1/16) atelier-cli::cli_integration review_help_exposes_only_the_collapsed_public_contract
        PASS [   0.241s] ( 2/16) atelier-cli::cli_integration setup_guidance::test_removed_maintenance_delete_is_unknown
        PASS [   0.285s] ( 3/16) atelier-cli::cli_integration issues::test_history_rejects_removed_query_and_objective_scope_flags
        PASS [   0.357s] ( 4/16) atelier-cli::cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery
        PASS [   0.424s] ( 5/16) atelier-cli::cli_integration records_evidence::test_evidence_list_elides_command_transcripts
        PASS [   0.444s] ( 6/16) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_no_summary_command_capture
        PASS [   0.528s] ( 7/16) atelier-cli::cli_integration setup_guidance::test_branch_recovery_is_hidden_from_routine_work_but_callable_explicitly
        PASS [   0.593s] ( 8/16) atelier-cli::cli_integration issues::test_history_repo_wide_is_bounded_and_routes_to_issue_drill_downs
        PASS [   0.939s] ( 9/16) atelier-cli::cli_integration records_evidence::test_evidence_quiet_output_is_a_stable_id_composition_path
        PASS [   0.940s] (10/16) atelier-cli::cli_integration provider_review_open_action_reads_workflow_config_and_global_secret
        PASS [   0.970s] (11/16) atelier-cli::cli_integration issues::test_history_issue_scope_stays_on_one_record_and_includes_linked_evidence
        PASS [   1.093s] (12/16) atelier-cli::cli_integration request_review_preserves_review_artifact_field
        PASS [   1.121s] (13/16) atelier-cli::cli_integration issues::test_history_issue_scope_excludes_other_targets_of_reused_evidence
        PASS [   1.237s] (14/16) atelier-cli::cli_integration review_surface_derives_open_context_and_uses_submit_and_show
        PASS [   2.464s] (15/16) atelier-cli::cli_integration records_evidence::test_evidence_capture_records_command_metadata_and_attaches_targets
        PASS [   3.152s] (16/16) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
────────────
     Summary [   3.154s] 16 tests run: 16 passed, 701 skipped
```

