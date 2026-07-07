---
created_at: "2026-07-07T06:14:22.287003084+00:00"
id: "atelier-phev"
evidence_type: "test"
captured_at: "2026-07-07T06:14:20.158913874+00:00"
command: "cargo test -p atelier-cli --test cli_integration setup_guidance::test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views -- --exact --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-sjsz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-sjsz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Mission Overview help, root dashboard, manager role guidance, and plural-versus-scoped command parity"
updated_at: "2026-07-07T06:14:26.277799537+00:00"
---

## Summary

Mission Overview help, root dashboard, manager role guidance, and plural-versus-scoped command parity

## Command

```console
cargo test -p atelier-cli --test cli_integration setup_guidance::test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views -- --exact --nocapture
```

Exit status: 0

## Stdout

Bytes: 226
Truncated: no

```text

running 1 test
test setup_guidance::test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 281 filtered out; finished in 0.09s
```

## Stderr

Bytes: 267
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.96s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```
