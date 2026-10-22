---
created_at: "2026-10-22T21:13:02.344659159+00:00"
id: "atelier-61sm"
evidence_type: "test"
commit: "0ca727ef527ad49eb3911df7f8e1019a68ed4567"
result: "pass"
captured_at: "2026-10-22T21:12:59.085573837+00:00"
command: "cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(lease)'"
exit_status: "0"
agent_identity: "claude-implementer-81e5"
target:
  kind: "issue"
  id: "atelier-81e5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-81e5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "live lease refuses start until --take-over and status reports it; stale leases are replaced"
updated_at: "2026-10-22T21:13:02.345131670+00:00"
---

## Summary

live lease refuses start until --take-over and status reports it; stale leases are replaced

## Command

```console
cargo nextest run -j 4 --features test-hooks -p atelier-cli -E 'test(lease)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 653
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-yim7/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.62s
────────────
 Nextest run ID f33ef419-8a6f-43ba-9a1c-d8b7ace58a46 with nextest profile: default
    Starting 2 tests across 6 binaries (637 tests skipped)
        PASS [   1.262s] (1/2) atelier-cli::session start_replaces_a_stale_lease_with_a_note
        PASS [   1.346s] (2/2) atelier-cli::session start_refuses_an_item_a_live_lease_holds_until_taken_over
────────────
     Summary [   1.347s] 2 tests run: 2 passed, 637 skipped
```

