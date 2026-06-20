---
created_at: "2026-06-20T21:23:33.393634108+00:00"
id: "atelier-ny5c"
evidence_type: "test"
captured_at: "2026-06-20T21:23:26.737322953+00:00"
command: "bash -lc 'set -euo pipefail\ncargo test -p atelier-app pr:: -- --nocapture\ncargo test -p atelier-cli review -- --nocapture\ncargo test -p atelier-cli linked_pr_merged -- --nocapture\nrg -q \"app_pr::linked_pull_request_merge_status_with_client\" crates/atelier-cli/src/commands/workflow.rs\nif rg -q \"linked_pr_merged_with_client|field review must be a provider pull_request object|linked PR branches\" crates/atelier-cli/src/commands/workflow.rs; then\n  echo \"workflow still owns linked PR parsing or branch validation\"\n  exit 1\nfi\necho \"review-link validation contract is centralized in atelier-app pr\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-0rx5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0rx5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ncargo test -p atelier-app pr:: -- --nocapture\ncargo test -p atelier-cli review -- --nocapture\ncargo test -p atelier-cli linked_pr_merged -- --nocapture\nrg -q \"app_pr::linked_pull_request_merge_status_with_client\" crates/atelier-cli/src/commands/workflow.rs\nif rg -q \"linked_pr_merged_with_client|field review must be a provider pull_request object|linked PR branches\" crates/atelier-cli/src/commands/workflow.rs; then\n  echo \"workflow still owns linked PR parsing or branch validation\"\n  exit 1\nfi\necho \"review-link validation contract is centralized in atelier-app pr\"'"
updated_at: "2026-06-20T21:23:38.108518844+00:00"
---

## Summary

bash -lc 'set -euo pipefail
cargo test -p atelier-app pr:: -- --nocapture
cargo test -p atelier-cli review -- --nocapture
cargo test -p atelier-cli linked_pr_merged -- --nocapture
rg -q "app_pr::linked_pull_request_merge_status_with_client" crates/atelier-cli/src/commands/workflow.rs
if rg -q "linked_pr_merged_with_client|field review must be a provider pull_request object|linked PR branches" crates/atelier-cli/src/commands/workflow.rs; then
  echo "workflow still owns linked PR parsing or branch validation"
  exit 1
fi
echo "review-link validation contract is centralized in atelier-app pr"'

## Command

```console
bash -lc 'set -euo pipefail
cargo test -p atelier-app pr:: -- --nocapture
cargo test -p atelier-cli review -- --nocapture
cargo test -p atelier-cli linked_pr_merged -- --nocapture
rg -q "app_pr::linked_pull_request_merge_status_with_client" crates/atelier-cli/src/commands/workflow.rs
if rg -q "linked_pr_merged_with_client|field review must be a provider pull_request object|linked PR branches" crates/atelier-cli/src/commands/workflow.rs; then
  echo "workflow still owns linked PR parsing or branch validation"
  exit 1
fi
echo "review-link validation contract is centralized in atelier-app pr"'
```

Exit status: 0

## Stdout

Bytes: 4021
Truncated: no

```text

running 20 tests
test pr::tests::parse_pull_request_reference_rejects_mismatched_url_context ... ok
test pr::tests::parse_pull_request_reference_accepts_number_and_matching_url ... ok
test pr::tests::parse_review_event_rejects_unknown_values ... ok
Initialized empty Git repository in /tmp/.tmp5pAcHJ/.git/
Initialized empty Git repository in /tmp/.tmpMOOiCi/.git/
Initialized empty Git repository in /tmp/.tmpZ2GpUW/.git/
Initialized empty Git repository in /tmp/.tmpzlww63/.git/
test pr::tests::infer_issue_id_rejects_missing_target ... ok
test pr::tests::pr_open_rejects_branch_mismatch_before_remote_create ... ok
test pr::tests::infer_issue_id_uses_owner_branch_before_active_work ... ok
test pr::tests::ensure_no_linked_pull_request_enforces_one_active_pr_per_owner ... ok
test pr::tests::linked_pull_request_merge_status_rejects_branch_mismatch ... ok
test pr::tests::status_outcome_resolves_linked_pr_without_rendering ... ok
test pr::tests::pr_comment_posts_to_linked_pull_and_records_owner_action ... ok
test pr::tests::pr_review_posts_review_event_and_records_owner_action ... ok
test pr::tests::record_pr_action_writes_owner_activity_with_remote_author_metadata ... ok
test pr::tests::infer_issue_id_rejects_ambiguous_active_work ... ok
test pr::tests::linked_pull_request_merge_status_reports_required_states ... ok
test pr::tests::pr_merge_rejects_missing_and_mismatched_pr_context ... ok
test pr::tests::pr_merge_confirms_already_merged_without_posting_merge_again ... ok
test pr::tests::persist_pull_request_writes_owner_epic_field_and_child_inherits ... ok
test pr::tests::pr_open_persists_link_and_records_action_after_preflight ... ok
test pr::tests::pr_link_fetches_remote_pull_and_persists_owner_field ... ok
test pr::tests::pr_merge_confirms_pull_request_attribution_and_preserves_status ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 59 filtered out; finished in 0.39s


running 7 tests
test commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret ... ok
test commands::pr::tests::explicit_review_role_wins ... ok
test commands::pr::tests::review_role_requires_status_role_when_not_explicit ... ok
test commands::pr::tests::review_role_infers_from_owner_status ... ok
test commands::workflow::tests::review_open_action_persists_room_review_field ... ok
test commands::workflow::tests::transition_status_write_preserves_review_field_from_pre_action_reload ... ok
test commands::workflow::tests::room_review_complete_requires_merged_room_artifact ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 151 filtered out; finished in 0.36s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test issues::test_bundle_preview_rejects_plan_and_milestone_resources ... ok
test issues::test_bundle_preview_rejects_duplicate_client_refs ... ok
test issues::test_bundle_preview_rejects_missing_client_ref ... ok
test issues::test_bundle_preview_rejects_status_outside_workflow_policy ... ok
test provider_review_open_action_reads_workflow_config_and_env_secret ... ok
test request_review_preserves_review_artifact_field ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 355 filtered out; finished in 1.35s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.00s


running 1 test
test commands::workflow::tests::linked_pr_merged_is_not_in_starter_close_policy ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 157 filtered out; finished in 0.11s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 361 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.00s

review-link validation contract is centralized in atelier-app pr
```

## Stderr

Bytes: 1147
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-91b312857bc4a702)
Switched to a new branch 'epic/atelier-epic'
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.99s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
     Running unittests src/main.rs (target/debug/deps/atelier-6490c36d57e88ab2)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-2a96ae708789461f)
```

