---
created_at: "2026-10-23T14:32:50.802467435+00:00"
id: "atelier-hpnt"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:32:48.033553338+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks review_list_ status_owner_block_shows_base_mission_strategy_and_eight_newest_reviews a_base_exemption_covers_exactly_its_commit"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-h6o4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-h6o4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "review list rows: time order, owner column dropped, first summary line, no Debug text; short ID same view; quiet IDs; known owner with no events and unknown ID; status Reviews use the same row"
updated_at: "2026-10-23T14:32:50.803314424+00:00"
---

## Summary

review list rows: time order, owner column dropped, first summary line, no Debug text; short ID same view; quiet IDs; known owner with no events and unknown ID; status Reviews use the same row

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks review_list_ status_owner_block_shows_base_mission_strategy_and_eight_newest_reviews a_base_exemption_covers_exactly_its_commit
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
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.93s
────────────
 Nextest run ID 6725c2d1-e8ee-4cc4-8454-22c01f96adcf with nextest profile: default
    Starting 4 tests across 10 binaries (896 tests skipped)
        PASS [   0.168s] (1/4) atelier-cli::cli_integration review_views::review_list_without_events_or_approval_says_so_and_unknown_ids_fail
        PASS [   0.395s] (2/4) atelier-cli::session status_owner_block_shows_base_mission_strategy_and_eight_newest_reviews
        PASS [   0.442s] (3/4) atelier-cli::cli_integration review_views::review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree
        PASS [   0.553s] (4/4) atelier-cli::session a_base_exemption_covers_exactly_its_commit
────────────
     Summary [   0.554s] 4 tests run: 4 passed, 896 skipped
```

