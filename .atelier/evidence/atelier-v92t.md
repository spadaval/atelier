---
created_at: "2026-10-23T14:32:56.073628925+00:00"
id: "atelier-v92t"
evidence_type: "test"
commit: "e7e76fde3096864c0735f705a8c742f5de060599"
result: "pass"
captured_at: "2026-10-23T14:32:53.518477183+00:00"
command: "cargo nextest run -j 4 --workspace --features test-hooks review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree"
exit_status: "0"
agent_identity: "claude-implementer-phfo"
target:
  kind: "issue"
  id: "atelier-jw1n"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-jw1n"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "review list <child> says the child is reviewed with its branch owner and prints the owner's view"
updated_at: "2026-10-23T14:32:56.073935086+00:00"
---

## Summary

review list <child> says the child is reviewed with its branch owner and prints the owner's view

## Command

```console
cargo nextest run -j 4 --workspace --features test-hooks review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 597
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-phfo/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.87s
────────────
 Nextest run ID 52178051-0d15-4c20-8e4c-dc67e789cefa with nextest profile: default
    Starting 1 test across 10 binaries (899 tests skipped)
        PASS [   0.402s] (1/1) atelier-cli::cli_integration review_views::review_list_shows_rows_in_time_order_then_approval_state_and_finding_tree
────────────
     Summary [   0.403s] 1 test run: 1 passed, 899 skipped
```

