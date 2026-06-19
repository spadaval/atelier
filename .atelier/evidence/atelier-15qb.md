---
created_at: "2026-06-18T00:43:34.910842141+00:00"
id: "atelier-15qb"
evidence_type: "validation"
captured_at: "2026-06-18T00:43:29.821086291+00:00"
command: "bash -lc 'set -euo pipefail\nrg -n \"schema_version: 2|forge_pr|required: \\[provider, host, owner, repo, number, url, source_branch, target_branch\\]\" .atelier/workflow.yaml\ncargo test -p atelier-workflow validates_forge_pr_field_shape\ncargo test -p atelier-workflow rejects_mismatched_forge_pr_field_shape\ncargo test -p atelier-app effective_forge_pr_field_inherits_from_nearest_parent_epic\ncargo test -p atelier-app effective_forge_pr_field_rejects_child_duplicate\ncargo test -p atelier-app rebuild_rejects_child_local_forge_pr_field\ncargo test -p atelier-app rebuild_rejects_issue_fields_that_violate_workflow_schema\ncargo check -p atelier-cli\ncargo fmt -- --check\ntarget/debug/atelier workflow check\ntarget/debug/atelier lint atelier-x1fn\ntarget/debug/atelier export --check\ngit diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-x1fn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x1fn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "forge_pr workflow field definition, validation, and child inheritance checks pass"
updated_at: "2026-06-18T00:43:38.671965120+00:00"
---

## Summary

forge_pr workflow field definition, validation, and child inheritance checks pass

## Command

```console
bash -lc 'set -euo pipefail
rg -n "schema_version: 2|forge_pr|required: \[provider, host, owner, repo, number, url, source_branch, target_branch\]" .atelier/workflow.yaml
cargo test -p atelier-workflow validates_forge_pr_field_shape
cargo test -p atelier-workflow rejects_mismatched_forge_pr_field_shape
cargo test -p atelier-app effective_forge_pr_field_inherits_from_nearest_parent_epic
cargo test -p atelier-app effective_forge_pr_field_rejects_child_duplicate
cargo test -p atelier-app rebuild_rejects_child_local_forge_pr_field
cargo test -p atelier-app rebuild_rejects_issue_fields_that_violate_workflow_schema
cargo check -p atelier-cli
cargo fmt -- --check
target/debug/atelier workflow check
target/debug/atelier lint atelier-x1fn
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 1546
Truncated: no

```text
2:schema_version: 2
36:  forge_pr:
38:    required: [provider, host, owner, repo, number, url, source_branch, target_branch]

running 1 test
test tests::validates_forge_pr_field_shape ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 1 test
test tests::rejects_mismatched_forge_pr_field_shape ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s


running 1 test
test workflow_policy::tests::effective_forge_pr_field_inherits_from_nearest_parent_epic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.13s


running 1 test
test workflow_policy::tests::effective_forge_pr_field_rejects_child_duplicate ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.07s


running 1 test
test rebuild::tests::rebuild_rejects_child_local_forge_pr_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.08s


running 1 test
test rebuild::tests::rebuild_rejects_issue_fields_that_violate_workflow_schema ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out; finished in 0.16s

Workflow Check
==============
Path:           .atelier/workflow.yaml
Policy:         pass
Issue Types:    6
Statuses:       7
Validators:     7
Workflows:      3
Record Health:  pass
Issues Checked: 608
Docs/Help Drift: clear
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1085
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/lib.rs (target/debug/deps/atelier_workflow-1c1848f4dab0f01c)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-09b7043f2491b1fc)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.81s
```

