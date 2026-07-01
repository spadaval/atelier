---
created_at: "2026-07-01T15:40:19.290192148+00:00"
id: "atelier-lhqe"
evidence_type: "test"
captured_at: "2026-07-01T15:40:17.261814851+00:00"
command: "cargo test -p atelier-cli --test cli_integration test_removed_maintenance_delete_is_unknown -- --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-g87o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-g87o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli --test cli_integration test_removed_maintenance_delete_is_unknown -- --nocapture"
updated_at: "2026-07-01T15:40:25.163589781+00:00"
---

## Summary

cargo test -p atelier-cli --test cli_integration test_removed_maintenance_delete_is_unknown -- --nocapture

## Command

```console
cargo test -p atelier-cli --test cli_integration test_removed_maintenance_delete_is_unknown -- --nocapture
```

Exit status: 0

## Stdout

Bytes: 185
Truncated: no

```text

running 1 test
test setup_guidance::test_removed_maintenance_delete_is_unknown ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 264 filtered out; finished in 0.09s
```

## Stderr

Bytes: 253
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/365e/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.87s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

