---
created_at: "2026-07-01T15:43:56.158241080+00:00"
id: "atelier-wz4q"
evidence_type: "test"
captured_at: "2026-07-01T15:43:53.947284983+00:00"
command: "cargo test -p atelier-cli --test cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery -- --exact"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-ie31"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ie31"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo test -p atelier-cli --test cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery -- --exact"
updated_at: "2026-07-01T15:44:01.995889333+00:00"
---

## Summary

cargo test -p atelier-cli --test cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery -- --exact

## Command

```console
cargo test -p atelier-cli --test cli_integration setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery -- --exact
```

Exit status: 0

## Stdout

Bytes: 223
Truncated: no

```text

running 1 test
test setup_guidance::test_forgejo_role_setup_is_hidden_from_normal_guidance_but_callable_for_recovery ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 265 filtered out; finished in 0.18s
```

## Stderr

Bytes: 253
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/365e/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.95s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```

