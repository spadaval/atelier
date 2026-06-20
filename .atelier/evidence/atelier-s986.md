---
created_at: "2026-06-20T21:57:06.633413535+00:00"
id: "atelier-s986"
evidence_type: "validation"
captured_at: "2026-06-20T21:56:52.863535847+00:00"
command: "bash -lc 'set -euo pipefail\ncargo fmt -- --check\ngit diff --check\ntest -f crates/atelier-cli/src/issue_cli.rs\ncargo build -p atelier-cli\ncargo test -p atelier-cli --test cli_integration test_issue_ready_list_uses_current_workflow_commands -- --nocapture\ncargo test -p atelier-cli --test cli_integration test_mission_status_reports_terminal_checks_and_explicit_approval -- --nocapture\ncargo test -p atelier-cli --test cli_integration test_agent_factory_guidance_avoids_raw_workflow_validate_commands -- --nocapture\n'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-p1yz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p1yz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "CLI module boundary split validation"
updated_at: "2026-06-20T21:57:11.585965391+00:00"
---

## Summary

CLI module boundary split validation

## Command

```console
bash -lc 'set -euo pipefail
cargo fmt -- --check
git diff --check
test -f crates/atelier-cli/src/issue_cli.rs
cargo build -p atelier-cli
cargo test -p atelier-cli --test cli_integration test_issue_ready_list_uses_current_workflow_commands -- --nocapture
cargo test -p atelier-cli --test cli_integration test_mission_status_reports_terminal_checks_and_explicit_approval -- --nocapture
cargo test -p atelier-cli --test cli_integration test_agent_factory_guidance_avoids_raw_workflow_validate_commands -- --nocapture
'
```

Exit status: 0

## Stdout

Bytes: 613
Truncated: no

```text

running 1 test
test setup_guidance::test_issue_ready_list_uses_current_workflow_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.19s


running 1 test
test records_evidence::test_mission_status_reports_terminal_checks_and_explicit_approval ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 4.39s


running 1 test
test setup_guidance::test_agent_factory_guidance_avoids_raw_workflow_validate_commands ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.00s
```

## Stderr

Bytes: 832
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.86s
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.98s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.14s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.97s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

