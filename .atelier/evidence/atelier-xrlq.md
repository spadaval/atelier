---
created_at: "2026-06-20T01:01:04.511684923+00:00"
id: "atelier-xrlq"
evidence_type: "validation"
captured_at: "2026-06-20T01:00:54.750150135+00:00"
command: "bash -lc 'cargo test -p atelier-workflow rejects_obsolete_flat_validator_names && cargo test -p atelier-cli workflow && cargo fmt -- --check && target/debug/atelier lint atelier-ee4u && target/debug/atelier workflow check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ee4u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ee4u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Namespaced validator rename proof: parser rejects obsolete flat names, CLI workflow tests exercise namespaced validator output, and tracker checks pass."
updated_at: "2026-06-20T01:01:09.095858335+00:00"
---

## Summary

Namespaced validator rename proof: parser rejects obsolete flat names, CLI workflow tests exercise namespaced validator output, and tracker checks pass.

## Command

```console
bash -lc 'cargo test -p atelier-workflow rejects_obsolete_flat_validator_names && cargo test -p atelier-cli workflow && cargo fmt -- --check && target/debug/atelier lint atelier-ee4u && target/debug/atelier workflow check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 4098
Truncated: yes

```text

running 1 test
test tests::rejects_obsolete_flat_validator_names ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out; finished in 0.02s


running 9 tests
test commands::forgejo::tests::workflow_role_authors_are_mapped_to_forgejo_config ... ok
test commands::workflow::tests::default_validators_are_target_and_transition_aware ... ok
test commands::workflow::tests::transition_action_plan_is_ordered_and_side_effect_free ... ok
test commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret ... ok
test commands::workflow::tests::action_preflight_checks_git_actions_before_execution ... ok
test commands::workflow::tests::linked_pr_merged_is_not_in_starter_close_policy ... ok
test commands::workflow::tests::linked_pr_merged_validator_reports_required_states ... ok
test commands::workflow::tests::linked_pr_merged_validator_rejects_branch_mismatch ... ok
test commands::workflow::tests::review_open_action_persists_room_review_field ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 172 filtered out; finished in 0.19s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 26 tests
test setup_guidance::test_agent_factory_guidance_avoids_raw_workflow_validate_commands ... ok
test setup_guidance::test_workflow_configuration_docs_describe_internal_diagnostics ... ok
test setup_guidance::test_workflow_help_is_scoped_as_advanced_internal_diagnostic ... ok
test issues::test_bundle_preview_rejects_status_outside_workflow_policy ... ok
test records_evidence::test_workflow_init_is_removed_and_root_init_owns_starter_policy ... ok
test setup_guidance::test_issue_ready_list_uses_current_workflow_commands ... ok
test setup_guidance::test_issue_transition_requires_workflow_policy_file ... ok
test records_evidence::test_workflow_check_rejects_stale_agent_guidance_commands ... ok
test records_evidence::test_workflow_check_rejects_nonexistent_option_in_hidden_context ... ok
test records_evidence::test_workflow_check_allows_removed_command_only_in_removal_history_context ... ok
test setup_guidance::test_root_start_reports_workflow_validator_failure ... ok
test records_evidence::test_workflow_check_rejects_hidden_command_as_normal_workflow ... ok
test records_evidence::test_workflow_check_rejects_stale_agent_guidance_options ... ok
test records_evidence::test_workflow_check_allows_hidden_command_only_in_hidden_context ... ok
test records_evidence::test_workflow_check_rejects_removed_command_as_normal_workflow ... ok
test records_evidence::test_workflow_check_rejects_issue_status_outside_selected_workflow ... ok
test records_evidence::test_workflow_check_reports_policy_and_issue_record_health ... ok
test records_evidence::test_workflow_check_rejects_legacy_issue_statuses_without_migration_path ... ok
test provider_review_open_action_reads_workflow_config_and_env_secret ... ok
test setup_guidance::test_root_start_applies_workflow_transition_and_records_active_work ... ok
test setup_guidance::test_issue_transition_options_and_successful_execution_follow_workflow_policy ... ok
test records_evidence::test_issue_create_after_workflow_init_uses_configured_initial_status ... ok
test sessions::workflow_milestones_emit_issue_attempt_metadata_without_session_records ... ok
test mission_projection_worktree::test_issue_orientation_uses_workflow_categories_and_exact_statuses ... ok
test mission_projection_worktree::test_branch_actions_prepare_and_integrate_epic_workflow ... ok
test records_evidence::test_validation_issue_closeout_uses_workflow_approval_not_contract_audit_terms ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 333 filtered out; finished in 2.37s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s

Lint passed.
Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Applicability:  6
Statuses:       7
Workflows:      4
Record Health:  pass
Issues Checked: 695
Docs/Help Drift: clea
```

## Stderr

Bytes: 1003
Truncated: no

```text
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
   Compiling atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
   Compiling atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
   Compiling atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.87s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
```

