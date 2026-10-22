---
created_at: "2026-10-22T20:42:47.037306108+00:00"
id: "atelier-5fxc"
evidence_type: "test"
commit: "702c76d7df5f9bad337bab4661c077b46f63efff"
result: "pass"
captured_at: "2026-10-22T20:42:45.260982134+00:00"
command: "cargo nextest run -j 4 --test merge_gates -E 'test(/interrupted_merge/)'"
exit_status: "0"
agent_identity: "claude-implementer-rcnk"
target:
  kind: "issue"
  id: "atelier-rcnk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-rcnk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A killed merge leaves a registered checkout; the next merge removes it, keeps one whose lock is held, and lands"
updated_at: "2026-10-22T20:42:47.037596730+00:00"
---

## Summary

A killed merge leaves a registered checkout; the next merge removes it, keeps one whose lock is held, and lands

## Command

```console
cargo nextest run -j 4 --test merge_gates -E 'test(/interrupted_merge/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 576
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-c3kh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.13s
────────────
 Nextest run ID 5dcde1dc-10d4-452d-9255-aa846bda2d51 with nextest profile: default
    Starting 1 test across 1 binary (59 tests skipped)
        PASS [   0.390s] (1/1) atelier-cli::merge_gates merge_commit_strategy_removes_the_checkout_an_interrupted_merge_left_behind
────────────
     Summary [   0.390s] 1 test run: 1 passed, 59 skipped
```

