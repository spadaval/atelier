---
created_at: "2026-06-16T20:15:12.430470764+00:00"
id: "atelier-k1ey"
evidence_type: "validation"
captured_at: "2026-06-16T20:15:06.664598740+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ncargo check -p atelier-cli\ncargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture\ncargo test -p atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping -- --nocapture\ntmp=$(mktemp -d)\ntrap '\"'\"'rm -rf \"$tmp\"'\"'\"' EXIT\ncd \"$tmp\"\ngit init -q\n/root/atelier/target/debug/atelier init >/tmp/atelier-z0yu-init.out\nif /root/atelier/target/debug/atelier issue create \"Rejected closeout smoke\" --issue-type closeout >/tmp/atelier-z0yu-create.out 2>/tmp/atelier-z0yu-create.err; then\n  echo \"closeout issue create unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/atelier-z0yu-create.err\nissue_id=$(/root/atelier/target/debug/atelier issue create \"Mutable task\" | sed -n \"s/^Created issue \\([^ ]*\\) -.*/\\1/p\")\ntest -n \"$issue_id\"\nif /root/atelier/target/debug/atelier issue update \"$issue_id\" --issue-type closeout >/tmp/atelier-z0yu-update.out 2>/tmp/atelier-z0yu-update.err; then\n  echo \"closeout issue update unexpectedly succeeded\" >&2\n  exit 1\nfi\nrg -F \"Invalid issue_type '\"'\"'closeout'\"'\"'\" /tmp/atelier-z0yu-update.err\ncd /root/atelier\n! rg -n -F -e '\"'\"'issue_type: \"closeout\"'\"'\"' -e '\"'\"'closeout: standard_review_proof'\"'\"' -e '\"'\"'Explicit work type (bug, closeout'\"'\"' -e '\"'\"'New issue type (bug, closeout'\"'\"' -e '\"'\"'\"closeout\",'\"'\"' crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues\ntarget/debug/atelier lint atelier-z0yu\ntarget/debug/atelier export --check\ngit diff --check'"
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
title: "Closeout issue type removal verified with corrected CLI smokes"
updated_at: "2026-06-16T20:15:16.011989287+00:00"
---

## Summary

Closeout issue type removal verified with corrected CLI smokes

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
cargo check -p atelier-cli
cargo test -p atelier-cli issues::test_removed_issue_type_is_rejected -- --nocapture
cargo test -p atelier-app workflow_policy::tests::rejects_missing_issue_type_mapping -- --nocapture
tmp=$(mktemp -d)
trap '"'"'rm -rf "$tmp"'"'"' EXIT
cd "$tmp"
git init -q
/root/atelier/target/debug/atelier init >/tmp/atelier-z0yu-init.out
if /root/atelier/target/debug/atelier issue create "Rejected closeout smoke" --issue-type closeout >/tmp/atelier-z0yu-create.out 2>/tmp/atelier-z0yu-create.err; then
  echo "closeout issue create unexpectedly succeeded" >&2
  exit 1
fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/atelier-z0yu-create.err
issue_id=$(/root/atelier/target/debug/atelier issue create "Mutable task" | sed -n "s/^Created issue \([^ ]*\) -.*/\1/p")
test -n "$issue_id"
if /root/atelier/target/debug/atelier issue update "$issue_id" --issue-type closeout >/tmp/atelier-z0yu-update.out 2>/tmp/atelier-z0yu-update.err; then
  echo "closeout issue update unexpectedly succeeded" >&2
  exit 1
fi
rg -F "Invalid issue_type '"'"'closeout'"'"'" /tmp/atelier-z0yu-update.err
cd /root/atelier
! rg -n -F -e '"'"'issue_type: "closeout"'"'"' -e '"'"'closeout: standard_review_proof'"'"' -e '"'"'Explicit work type (bug, closeout'"'"' -e '"'"'New issue type (bug, closeout'"'"' -e '"'"'"closeout",'"'"' crates/atelier-sqlite/src crates/atelier-records/src crates/atelier-workflow/src crates/atelier-app/src/workflow_policy.rs crates/atelier-cli/src .atelier/workflow.yaml .atelier/issues
target/debug/atelier lint atelier-z0yu
target/debug/atelier export --check
git diff --check'
```

Exit status: 0

## Stdout

Bytes: 959
Truncated: no

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 170 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test issues::test_removed_issue_type_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 339 filtered out; finished in 0.12s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 52 filtered out; finished in 0.00s


running 1 test
test workflow_policy::tests::rejects_missing_issue_type_mapping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 43 filtered out; finished in 0.00s

Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Error: Invalid issue_type 'closeout'. Valid values: bug, epic, feature, spike, task, validation
Lint passed.
Canonical export is current
State: /root/atelier/.atelier
```

## Stderr

Bytes: 768
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.76s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.73s
     Running unittests src/lib.rs (target/debug/deps/atelier-b858d6145a090057)
     Running unittests src/main.rs (target/debug/deps/atelier-b3519a551517247c)
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-3c9e2d6e9f1b0cb8)
     Running tests/smoke_tests.rs (target/debug/deps/smoke_tests-8cb54623b766e956)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running unittests src/lib.rs (target/debug/deps/atelier_app-b996d9a0f399a6f5)
```

