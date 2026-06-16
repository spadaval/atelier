---
created_at: "2026-06-16T20:14:38.984496803+00:00"
id: "atelier-0m0l"
evidence_type: "validation"
captured_at: "2026-06-16T20:14:31.527047574+00:00"
command: "bash -lc 'cargo fmt -- --check && cargo check -p atelier-cli && cargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture && cargo test -p atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping -- --nocapture && tmp=$(mktemp -d); cd \"$tmp\"; git init -q; /root/atelier/target/debug/atelier init >/tmp/atelier-z0yu-init.out; /root/atelier/target/debug/atelier issue create \"Rejected closeout smoke\" --issue-type closeout >/tmp/atelier-z0yu-create.out 2>/tmp/atelier-z0yu-create.err; create_status=$?; test \"$create_status\" -ne 0; rg -n \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/atelier-z0yu-create.err; issue_id=$(/root/atelier/target/debug/atelier issue create \"Mutable task\" | awk \"/Created issue/ {print \\\\$3}\"); /root/atelier/target/debug/atelier issue update \"$issue_id\" --issue-type closeout >/tmp/atelier-z0yu-update.out 2>/tmp/atelier-z0yu-update.err; update_status=$?; test \"$update_status\" -ne 0; rg -n \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/atelier-z0yu-update.err; rm -rf \"$tmp\"; cd /root/atelier; ! rg -n \"issue_type: \\\\\"closeout\\\\\"|closeout: standard_review_proof|Explicit work type .*closeout|New issue type .*closeout|\\\\\"closeout\\\\\",\" crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; target/debug/atelier lint atelier-z0yu && target/debug/atelier export --check && git diff --check'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-z0yu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-z0yu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Closeout issue type removed and migrated"
updated_at: "2026-06-16T20:14:42.837815688+00:00"
---

## Summary

Closeout issue type removed and migrated

## Command

```console
bash -lc 'cargo fmt -- --check && cargo check -p atelier-cli && cargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture && cargo test -p atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping -- --nocapture && tmp=$(mktemp -d); cd "$tmp"; git init -q; /root/atelier/target/debug/atelier init >/tmp/atelier-z0yu-init.out; /root/atelier/target/debug/atelier issue create "Rejected closeout smoke" --issue-type closeout >/tmp/atelier-z0yu-create.out 2>/tmp/atelier-z0yu-create.err; create_status=$?; test "$create_status" -ne 0; rg -n "Invalid issue_type '"'"'closeout'"'"'" /tmp/atelier-z0yu-create.err; issue_id=$(/root/atelier/target/debug/atelier issue create "Mutable task" | awk "/Created issue/ {print \\$3}"); /root/atelier/target/debug/atelier issue update "$issue_id" --issue-type closeout >/tmp/atelier-z0yu-update.out 2>/tmp/atelier-z0yu-update.err; update_status=$?; test "$update_status" -ne 0; rg -n "Invalid issue_type '"'"'closeout'"'"'" /tmp/atelier-z0yu-update.err; rm -rf "$tmp"; cd /root/atelier; ! rg -n "issue_type: \\"closeout\\"|closeout: standard_review_proof|Explicit work type .*closeout|New issue type .*closeout|\\"closeout\\"," crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues; target/debug/atelier lint atelier-z0yu && target/debug/atelier export --check && git diff --check'
```

Exit status: 0

## Stdout

Bytes: 865
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_removed_issue_type_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.13s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test workflow_policy::tests::rejects_missing_issue_type_mapping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out; finished in 0.00s

1:Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 1671
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Checking atelier-records v0.2.0 (/root/atelier/crates/atelier-records)
    Checking atelier-sqlite v0.2.0 (/root/atelier/crates/atelier-sqlite)
    Checking atelier-app v0.2.0 (/root/atelier/crates/atelier-app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.14s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.86s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)
awk: cmd. line:1: /Created issue/ {print \}
awk: cmd. line:1:                        ^ backslash not last character on line
awk: cmd. line:1: /Created issue/ {print \}
awk: cmd. line:1:                        ^ syntax error

thread 'main' (1019718) panicked at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/io/stdio.rs:1165:9:
failed printing to stdout: Broken pipe (os error 32)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rg: regex parse error:
    (?:issue_type: \closeout\|closeout: standard_review_proof|Explicit work type .*closeout|New issue type .*closeout|\closeout\,)
                   ^^
error: unrecognized escape sequence
```

