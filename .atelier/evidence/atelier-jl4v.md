---
created_at: "2026-06-20T01:26:29.366762334+00:00"
id: "atelier-jl4v"
evidence_type: "validation"
captured_at: "2026-06-20T01:26:26.551468677+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo test -p atelier-workflow && target/debug/atelier workflow check && target/debug/atelier lint atelier-bmqo && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-bmqo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-bmqo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'cargo fmt -- --check && cargo test -p atelier-workflow && target/debug/atelier workflow check && target/debug/atelier lint atelier-bmqo && git diff --check'"
updated_at: "2026-06-20T01:26:33.833356997+00:00"
---

## Summary

bash -lc 'cargo fmt -- --check && cargo test -p atelier-workflow && target/debug/atelier workflow check && target/debug/atelier lint atelier-bmqo && git diff --check'

## Command

```console
bash -lc 'cargo fmt -- --check && cargo test -p atelier-workflow && target/debug/atelier workflow check && target/debug/atelier lint atelier-bmqo && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 2095
Truncated: no

```text

running 29 tests
test tests::rejects_invalid_status_category ... ok
test tests::rejects_invalid_action_params ... ok
test tests::rejects_duplicate_issue_type_coverage ... ok
test tests::rejects_legacy_transition_effects_field ... ok
test tests::missing_branch_policy_is_rejected ... ok
test tests::rejects_missing_issue_type_coverage ... ok
test tests::validates_review_field_shape ... ok
test tests::rejects_mismatched_review_field_shape ... ok
test tests::parses_configured_branch_policy ... ok
test tests::rejects_configured_branch_policy_without_base_branch ... ok
test tests::rejects_unknown_inline_validator ... ok
test tests::starter_policy_does_not_require_legacy_pr_merge_gate ... ok
test tests::rejects_unknown_issue_type_in_record ... ok
test tests::rejects_review_action_on_non_review_transition ... ok
test tests::rejects_legacy_review_artifact_action_identifier ... ok
test tests::rejects_removed_top_level_fields ... ok
test tests::rejects_unknown_top_level_field ... ok
test tests::parses_forgejo_review_action_params ... ok
test tests::branch_name_for_owner_renders_configured_templates ... ok
test tests::rejects_duplicate_transition_action ... ok
test tests::parses_valid_policy ... ok
test tests::accepts_empty_action_param_object ... ok
test tests::rejects_unknown_transition_action ... ok
test tests::rejects_missing_issue_type_registry_entry ... ok
test tests::parses_custom_issue_type_registry ... ok
test tests::rejects_invalid_evidence_validator_params ... ok
test tests::rejects_legacy_pull_request_field_shape ... ok
test tests::rejects_invalid_issue_type_name_and_label ... ok
test tests::rejects_obsolete_flat_validator_names ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

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

Bytes: 191
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
   Doc-tests atelier_workflow
```

