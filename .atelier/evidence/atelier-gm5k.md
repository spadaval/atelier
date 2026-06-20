---
created_at: "2026-06-20T01:17:09.827155761+00:00"
id: "atelier-gm5k"
evidence_type: "validation"
captured_at: "2026-06-20T01:17:00.128525131+00:00"
command: "bash -lc 'cargo test -p atelier-cli workflow --lib && cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field && cargo fmt -- --check && cargo build -p atelier-cli && target/debug/atelier workflow check && target/debug/atelier lint atelier-v4ah && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-v4ah"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-v4ah"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Command-backed validation for review.complete artifact semantics and transition review-field preservation"
updated_at: "2026-06-20T01:17:14.307612013+00:00"
---

## Summary

Command-backed validation for review.complete artifact semantics and transition review-field preservation

## Command

```console
bash -lc 'cargo test -p atelier-cli workflow --lib && cargo test -p atelier-cli --test cli_integration request_review_preserves_review_artifact_field && cargo fmt -- --check && cargo build -p atelier-cli && target/debug/atelier workflow check && target/debug/atelier lint atelier-v4ah && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1432
Truncated: no

```text

running 10 tests
test commands::forgejo::tests::workflow_role_authors_are_mapped_to_forgejo_config ... ok
test commands::workflow::tests::default_validators_are_target_and_transition_aware ... ok
test commands::workflow::tests::transition_action_plan_is_ordered_and_side_effect_free ... ok
test commands::workflow::tests::provider_review_action_preflight_uses_workflow_role_authors_and_env_secret ... ok
test commands::workflow::tests::action_preflight_checks_git_actions_before_execution ... ok
test commands::workflow::tests::linked_pr_merged_is_not_in_starter_close_policy ... ok
test commands::workflow::tests::linked_pr_merged_validator_reports_required_states ... ok
test commands::workflow::tests::linked_pr_merged_validator_rejects_branch_mismatch ... ok
test commands::workflow::tests::review_open_action_persists_room_review_field ... ok
test commands::workflow::tests::room_review_complete_requires_merged_room_artifact ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 172 filtered out; finished in 0.35s


running 1 test
test request_review_preserves_review_artifact_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 359 filtered out; finished in 0.52s

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
```

## Stderr

Bytes: 589
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.51s
     Running unittests src/lib.rs (target/debug/deps/atelier-945a47c12cf55d10)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.48s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.36s
```

