---
created_at: "2026-07-07T05:45:47.771391378+00:00"
id: "atelier-w2py"
evidence_type: "test"
captured_at: "2026-07-07T05:45:44.955410826+00:00"
command: "cargo test -p atelier-cli --test cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output -- --exact --nocapture"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-dy3u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-dy3u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Temporary-repository Mission Overview CLI transcript: hierarchy, blocker-aware progress, direct/unassigned accounting, NO_COLOR parity, quiet IDs"
updated_at: "2026-07-07T05:45:51.817106968+00:00"
---

## Summary

Temporary-repository Mission Overview CLI transcript: hierarchy, blocker-aware progress, direct/unassigned accounting, NO_COLOR parity, quiet IDs

## Command

```console
cargo test -p atelier-cli --test cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output -- --exact --nocapture
```

Exit status: 0

## Stdout

Bytes: 233
Truncated: no

```text

running 1 test
test mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 279 filtered out; finished in 0.41s
```

## Stderr

Bytes: 267
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.33s
     Running tests/cli_integration.rs (target/debug/deps/cli_integration-55f33c0e51df1ecf)
```
