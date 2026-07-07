---
created_at: "2026-07-06T19:14:28.286409166+00:00"
id: "atelier-4uqh"
evidence_type: "test"
captured_at: "2026-07-06T19:14:24.240425895+00:00"
command: "cargo nextest run -p atelier-cli -E 'test(/test_cache_query_rebuilds_missing_cache_on_demand|test_root_status_reports_current_mission_counts_without_active_focus|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild|test_evidence_list_bounds_default_output|test_issue_show_surfaces_evidence_status|test_issue_list_inventory_filters_across_status_type_and_state/)'"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-0p7e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-0p7e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "cargo nextest run -p atelier-cli -E 'test(/test_cache_query_rebuilds_missing_cache_on_demand|test_root_status_reports_current_mission_counts_without_active_focus|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild|test_evidence_list_bounds_default_output|test_issue_show_surfaces_evidence_status|test_issue_list_inventory_filters_across_status_type_and_state/)'"
updated_at: "2026-07-06T19:14:32.172709162+00:00"
---

## Summary

cargo nextest run -p atelier-cli -E 'test(/test_cache_query_rebuilds_missing_cache_on_demand|test_root_status_reports_current_mission_counts_without_active_focus|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild|test_evidence_list_bounds_default_output|test_issue_show_surfaces_evidence_status|test_issue_list_inventory_filters_across_status_type_and_state/)'

## Command

```console
cargo nextest run -p atelier-cli -E 'test(/test_cache_query_rebuilds_missing_cache_on_demand|test_root_status_reports_current_mission_counts_without_active_focus|test_issue_mutations_leave_stale_cache_for_one_lazy_repair|test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild|test_evidence_list_bounds_default_output|test_issue_show_surfaces_evidence_status|test_issue_list_inventory_filters_across_status_type_and_state/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1359
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/mska-ckca/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.05s
────────────
 Nextest run ID 3bdbad47-7ab6-4234-9349-f92be135254d with nextest profile: default
    Starting 7 tests across 4 binaries (452 tests skipped)
        PASS [   0.178s] (1/7) atelier-cli::cli_integration mission_projection_worktree::test_cache_query_rebuilds_missing_cache_on_demand
        PASS [   0.315s] (2/7) atelier-cli::cli_integration issues::test_issue_sections_are_canonical_after_direct_markdown_edit_and_rebuild
        PASS [   0.442s] (3/7) atelier-cli::cli_integration issues::test_issue_list_inventory_filters_across_status_type_and_state
        PASS [   0.476s] (4/7) atelier-cli::cli_integration issues::test_issue_mutations_leave_stale_cache_for_one_lazy_repair
        PASS [   0.564s] (5/7) atelier-cli::cli_integration records_evidence::test_evidence_list_bounds_default_output
        PASS [   0.651s] (6/7) atelier-cli::cli_integration mission_projection_worktree::test_root_status_reports_current_mission_counts_without_active_focus
        PASS [   1.686s] (7/7) atelier-cli::cli_integration issues::test_issue_show_surfaces_evidence_status
────────────
     Summary [   1.687s] 7 tests run: 7 passed, 452 skipped
```
