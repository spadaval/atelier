---
created_at: "2026-10-23T14:41:20.362119885+00:00"
id: "atelier-cuzp"
evidence_type: "test"
commit: "fa47f83a4dbb588e03d70cbfeed54763ddd47cbc"
result: "pass"
captured_at: "2026-10-23T14:41:18.274336001+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check)'"
exit_status: "0"
agent_identity: "claude-implementer-h1nc"
target:
  kind: "issue"
  id: "atelier-9jr8"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-9jr8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "land refusal prints one line per failed check and record with cause and real-ID fix, folds repeated blocker checks, lists passes, verdict counts all 5 checks; Stop hook reports the same lines"
updated_at: "2026-10-23T14:41:20.363018941+00:00"
---

## Summary

land refusal prints one line per failed check and record with cause and real-ID fix, folds repeated blocker checks, lists passes, verdict counts all 5 checks; Stop hook reports the same lines

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks -E 'test(land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 574
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-h1nc/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s
────────────
 Nextest run ID 647385e1-8fcf-4a0b-8e2f-c5a0e1409615 with nextest profile: default
    Starting 1 test across 10 binaries (897 tests skipped)
        PASS [   0.248s] (1/1) atelier-cli::session land_refusal_lists_each_failed_check_with_its_fix_and_counts_every_check
────────────
     Summary [   0.249s] 1 test run: 1 passed, 897 skipped
```

