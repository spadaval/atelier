---
created_at: "2026-10-22T22:51:15.751449240+00:00"
id: "atelier-10w0"
evidence_type: "test"
commit: "6fbdc5695856093faf5e168a6a5543e1dcd04b6e"
result: "pass"
captured_at: "2026-10-22T22:50:37.388641938+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session land_push"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-976s"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-976s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land --push refuses a gone upstream and a URL remote for the base, pushing nothing; existing land_push tests still pass"
updated_at: "2026-10-22T22:51:15.751721858+00:00"
---

## Summary

land --push refuses a gone upstream and a URL remote for the base, pushing nothing; existing land_push tests still pass

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session land_push
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 960
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.77s
────────────
 Nextest run ID 6a9b22bd-daa8-4f6b-a015-541180c2a88d with nextest profile: default
    Starting 4 tests across 1 binary (102 tests skipped)
        PASS [   0.193s] (1/4) atelier-cli::session land_push_pushes_nothing_when_a_check_moves_head_during_land
        PASS [   0.212s] (2/4) atelier-cli::session land_push_refuses_an_upstream_that_is_not_the_branchs_own_remote_branch
        PASS [   0.230s] (3/4) atelier-cli::session land_push_reports_a_failed_push_and_a_missing_upstream
        PASS [   0.268s] (4/4) atelier-cli::session land_push_refuses_a_base_upstream_that_does_not_resolve
────────────
     Summary [   0.269s] 4 tests run: 4 passed, 102 skipped
```

