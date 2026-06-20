---
created_at: "2026-06-20T01:19:22.244256992+00:00"
id: "atelier-7rgb"
evidence_type: "validation"
captured_at: "2026-06-20T01:19:12.070643901+00:00"
command: "bash -lc 'cargo test -p atelier-workflow && cargo test -p atelier-cli workflow --lib && cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field && cargo fmt -- --check && target/debug/atelier workflow check && target/debug/atelier lint atelier-yhui && target/debug/atelier issue transition atelier-yhui --options && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yhui"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yhui"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Parent validation for validator namespace cleanup and artifact-backed review completion"
updated_at: "2026-06-20T01:19:26.646637955+00:00"
---

## Summary

Parent validation for validator namespace cleanup and artifact-backed review completion

## Command

```console
bash -lc 'cargo test -p atelier-workflow && cargo test -p atelier-cli workflow --lib && cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field && cargo fmt -- --check && target/debug/atelier workflow check && target/debug/atelier lint atelier-yhui && target/debug/atelier issue transition atelier-yhui --options && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 4769
Truncated: yes

```text

running 25 tests
test tests::missing_branch_policy_is_rejected ... ok
test tests::branch_name_for_owner_renders_configured_templates ... ok
test tests::parses_configured_branch_policy ... ok
test tests::rejects_invalid_status_category ... ok
test tests::rejects_invalid_evidence_validator_params ... ok
test tests::parses_valid_policy ... ok
test tests::accepts_empty_action_param_object ... ok
test tests::rejects_legacy_review_artifact_action_identifier ... ok
test tests::rejects_duplicate_transition_action ... ok
test tests::rejects_invalid_action_params ... ok
test tests::rejects_unknown_inline_validator ... ok
test tests::rejects_legacy_pull_request_field_shape ... ok
test tests::rejects_missing_issue_type_coverage ... ok
test tests::rejects_unknown_transition_action ... ok
test tests::rejects_unknown_top_level_field ... ok
test tests::starter_policy_does_not_require_legacy_pr_merge_gate ... ok
test tests::rejects_configured_branch_policy_without_base_branch ... ok
test tests::rejects_duplicate_issue_type_coverage ... ok
test tests::parses_forgejo_review_action_params ... ok
test tests::rejects_mismatched_review_field_shape ... ok
test tests::rejects_legacy_transition_effects_field ... ok
test tests::rejects_removed_top_level_fields ... ok
test tests::rejects_review_action_on_non_review_transition ... ok
test tests::validates_review_field_shape ... ok
test tests::rejects_obsolete_flat_validator_names ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test commands::forgejo::tests::workflow_role_authors_are_mapped_to_forgejo_config ... ok
test commands::workflow::tests::default_validators_are_target_and_transition_aware ... ok
test commands::workflow::tests::transition_action_plan_is_ordered_and_side_effect_free ... ok
test commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret ... ok
test commands::workflow::tests::action_preflight_checks_git_actions_before_execution ... ok
test commands::workflow::tests::linked_pr_merged_is_not_in_starter_close_policy ... ok
test commands::workflow::tests::linked_pr_merged_validator_rejects_branch_mismatch ... ok
test commands::workflow::tests::linked_pr_merged_validator_reports_required_states ... ok
test commands::workflow::tests::review_open_action_persists_room_review_field ... ok
test commands::workflow::tests::room_review_complete_requires_merged_room_artifact ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 172 filtered out; finished in 0.36s


running 1 test
test request_review_preserves_review_artifact_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 359 filtered out; finished in 0.51s

Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Applicability:  6
Statuses:       7
Workflows:      4
Record Health:  pass
Issues Checked: 695
Docs/Help Drift: clear
Lint passed.
Lint passed.
Issue Transitions atelier-yhui - Epic: Namespace validators and fix semantics
=============================================================================
State
-----
Status:   validation
Type:     epic
Options:  2
Branch Context
--------------
Owner:    epic atelier-yhui (epic)
Expected: epic/atelier-yhui
Base:     master
Current:  epic/atelier-yhui
State:    current branch matches expected branch

block [allowed]
  From: todo, in_progress, review, validation
  To:   blocked
  Command: atelier issue transition atelier-yhui block
Validators
----------
(none)
Blockers
--------
(none)
Planned Actions
---------------
(none)
Description
-----------
(none)

close [blocked]
  From: validation
  To:   done
  Command: atelier issue transition atelier-yhui close
Validators
----------
  fail  evidence.attached
      expected at least 1 validating evidence record(s); found 0
  pass  children.proof_complete
      all epic child issues are closed with validating proof
  pass  blockers.none_open
      no o
```

## Stderr

Bytes: 718
Truncated: no

```text
   Compiling atelier-workflow v0.2.0 (/root/atelier/crates/atelier-workflow)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
   Doc-tests atelier_workflow
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.53s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.44s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

