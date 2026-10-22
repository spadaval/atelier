---
created_at: "2026-10-22T21:01:26.005352043+00:00"
id: "atelier-7dl7"
evidence_type: "test"
commit: "5eea164a64939ac05a36a635277a7f60e0659bcc"
result: "pass"
captured_at: "2026-10-22T21:01:22.924627805+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(start_with_a_base)'"
exit_status: "0"
agent_identity: "claude-implementer-yc4d"
target:
  kind: "issue"
  id: "atelier-yc4d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yc4d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "start --base works from detached HEAD and a different current branch; refuses non-policy and missing bases"
updated_at: "2026-10-22T21:01:26.005850402+00:00"
---

## Summary

start --base works from detached HEAD and a different current branch; refuses non-policy and missing bases

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(start_with_a_base)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 683
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.63s
────────────
 Nextest run ID 4e8aa099-6042-43fa-87c1-807607e40225 with nextest profile: default
    Starting 2 tests across 6 binaries (632 tests skipped)
        PASS [   0.137s] (1/2) atelier-cli::session start_with_a_base_creates_the_owner_branch_from_a_detached_head
        PASS [   0.196s] (2/2) atelier-cli::session start_with_a_base_ignores_the_current_branch_and_checks_the_base
────────────
     Summary [   0.197s] 2 tests run: 2 passed, 632 skipped
```

