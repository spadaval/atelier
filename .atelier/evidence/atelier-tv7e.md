---
created_at: "2026-10-22T21:10:37.138653184+00:00"
id: "atelier-tv7e"
evidence_type: "test"
commit: "d8351d878a66a64355662ebeaa03603495bd8fa8"
result: "pass"
captured_at: "2026-10-22T21:10:35.190049985+00:00"
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
title: "start --base resolves from base records, from older branches and detached commits"
updated_at: "2026-10-22T21:10:37.139149860+00:00"
---

## Summary

start --base resolves from base records, from older branches and detached commits

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

Bytes: 800
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.51s
────────────
 Nextest run ID 19b3f7fb-27c2-43fc-a461-10ee25e7e33f with nextest profile: default
    Starting 3 tests across 6 binaries (634 tests skipped)
        PASS [   0.140s] (1/3) atelier-cli::session start_with_a_base_creates_the_owner_branch_from_a_detached_head
        PASS [   0.184s] (2/3) atelier-cli::session start_with_a_base_ignores_the_current_branch_and_checks_the_base
        PASS [   0.185s] (3/3) atelier-cli::session start_with_a_base_works_from_a_checkout_without_the_issue_record
────────────
     Summary [   0.186s] 3 tests run: 3 passed, 634 skipped
```

