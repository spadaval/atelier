---
created_at: "2026-10-23T14:32:53.419718403+00:00"
id: "atelier-usqh"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:32:50.905338749+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree review_list_without_events_or_approval_says_so_and_unknown_ids_fail"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-uv5d"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-uv5d"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "review list <id> state block: current approval, stale approval with the commit and paths that staled it, missing approval, open finding first, fix_now and superseded no_action disposition chains"
updated_at: "2026-10-23T14:32:53.420003074+00:00"
---

## Summary

review list <id> state block: current approval, stale approval with the commit and paths that staled it, missing approval, open finding first, fix_now and superseded no_action disposition chains

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree review_list_without_events_or_approval_says_so_and_unknown_ids_fail
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 741
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.83s
────────────
 Nextest run ID 30eb55a7-980d-4180-b911-825fa8422e6a with nextest profile: default
    Starting 2 tests across 10 binaries (898 tests skipped)
        PASS [   0.147s] (1/2) atelier-cli::cli_integration review_views::review_list_without_events_or_approval_says_so_and_unknown_ids_fail
        PASS [   0.402s] (2/2) atelier-cli::cli_integration review_views::review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree
────────────
     Summary [   0.403s] 2 tests run: 2 passed, 898 skipped
```

