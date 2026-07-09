---
created_at: "2026-07-06T18:04:26.646718429+00:00"
id: "atelier-xens"
evidence_type: "test"
captured_at: "2026-07-06T18:04:21.408377525+00:00"
command: "cargo nextest run -p atelier-cli human_output::tests::issue_inventory"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-yf06"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-yf06"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli human_output::tests::issue_inventory"
updated_at: "2026-07-06T18:04:31.206403029+00:00"
---

## Summary

cargo nextest run -p atelier-cli human_output::tests::issue_inventory

## Command

```console
cargo nextest run -p atelier-cli human_output::tests::issue_inventory
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```
## Stderr

Bytes: 1022
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.84s
────────────
 Nextest run ID eebd737f-a836-4938-b0d2-97c303ee43cc with nextest profile: default
    Starting 5 tests across 4 binaries (452 tests skipped)
        PASS [   0.010s] (1/5) atelier-cli human_output::tests::issue_inventory_color_changes_style_only
        PASS [   0.010s] (2/5) atelier-cli human_output::tests::issue_inventory_reports_truncation_and_filter_guidance
        PASS [   0.010s] (3/5) atelier-cli human_output::tests::issue_inventory_empty_and_quiet_states_are_explicit
        PASS [   0.010s] (4/5) atelier-cli human_output::tests::issue_inventory_quiet_uses_view_order_and_contains_only_ids
        PASS [   0.011s] (5/5) atelier-cli human_output::tests::issue_inventory_renders_one_flat_metadata_row_per_issue
────────────
     Summary [   0.012s] 5 tests run: 5 passed, 452 skipped
```
