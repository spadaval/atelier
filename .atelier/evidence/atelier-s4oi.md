---
created_at: "2026-06-20T21:28:33.934587420+00:00"
id: "atelier-s4oi"
evidence_type: "validation"
captured_at: "2026-06-20T21:28:29.013826290+00:00"
command: "bash -lc 'set -euo pipefail\ntarget/debug/atelier --help > /tmp/atelier-help.txt\nif rg -q \"^[[:space:]]*worktree[[:space:]]\" /tmp/atelier-help.txt; then\n  cat /tmp/atelier-help.txt\n  exit 1\nfi\nif target/debug/atelier worktree --help >/tmp/worktree-help.out 2>/tmp/worktree-help.err; then\n  echo \"worktree help unexpectedly succeeded\"\n  exit 1\nfi\nrg -q \"unrecognized subcommand .worktree.\" /tmp/worktree-help.err\ncargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_work_commands_are_removed -- --nocapture\ncargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_epic_branch_commands_use_current_checkout -- --nocapture\necho \"worktree root removed and branch helpers work from current checkout\"'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-7eio"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7eio"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "bash -lc 'set -euo pipefail\ntarget/debug/atelier --help > /tmp/atelier-help.txt\nif rg -q \"^[[:space:]]*worktree[[:space:]]\" /tmp/atelier-help.txt; then\n  cat /tmp/atelier-help.txt\n  exit 1\nfi\nif target/debug/atelier worktree --help >/tmp/worktree-help.out 2>/tmp/worktree-help.err; then\n  echo \"worktree help unexpectedly succeeded\"\n  exit 1\nfi\nrg -q \"unrecognized subcommand .worktree.\" /tmp/worktree-help.err\ncargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_work_commands_are_removed -- --nocapture\ncargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_epic_branch_commands_use_current_checkout -- --nocapture\necho \"worktree root removed and branch helpers work from current checkout\"'"
updated_at: "2026-06-20T21:28:38.836985639+00:00"
---

## Summary

bash -lc 'set -euo pipefail
target/debug/atelier --help > /tmp/atelier-help.txt
if rg -q "^[[:space:]]*worktree[[:space:]]" /tmp/atelier-help.txt; then
  cat /tmp/atelier-help.txt
  exit 1
fi
if target/debug/atelier worktree --help >/tmp/worktree-help.out 2>/tmp/worktree-help.err; then
  echo "worktree help unexpectedly succeeded"
  exit 1
fi
rg -q "unrecognized subcommand .worktree." /tmp/worktree-help.err
cargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_work_commands_are_removed -- --nocapture
cargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_epic_branch_commands_use_current_checkout -- --nocapture
echo "worktree root removed and branch helpers work from current checkout"'

## Command

```console
bash -lc 'set -euo pipefail
target/debug/atelier --help > /tmp/atelier-help.txt
if rg -q "^[[:space:]]*worktree[[:space:]]" /tmp/atelier-help.txt; then
  cat /tmp/atelier-help.txt
  exit 1
fi
if target/debug/atelier worktree --help >/tmp/worktree-help.out 2>/tmp/worktree-help.err; then
  echo "worktree help unexpectedly succeeded"
  exit 1
fi
rg -q "unrecognized subcommand .worktree." /tmp/worktree-help.err
cargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_work_commands_are_removed -- --nocapture
cargo test -p atelier-cli --test cli_integration mission_projection_worktree::test_epic_branch_commands_use_current_checkout -- --nocapture
echo "worktree root removed and branch helpers work from current checkout"'
```

Exit status: 0

## Stdout

Bytes: 456
Truncated: no

```text

running 1 test
test mission_projection_worktree::test_work_commands_are_removed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 360 filtered out; finished in 0.09s


running 1 test
test mission_projection_worktree::test_epic_branch_commands_use_current_checkout ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 360 filtered out; finished in 0.53s

worktree root removed and branch helpers work from current checkout
```

## Stderr

Bytes: 488
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.00s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
Switched to branch 'main'
```

