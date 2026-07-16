---
created_at: "2026-07-07T06:27:11.476275068+00:00"
id: "atelier-c3gj"
evidence_type: "validation"
captured_at: "2026-07-07T06:27:00.422319817+00:00"
command: "bash -lc 'cargo nextest run -p atelier-cli -E '\"'\"'test(/test_work_missions|mission_overview_|test_issue_list|test_list_issues/)'\"'\"' && cargo nextest run -p atelier-app -E '\"'\"'test(/mission_overview/)'\"'\"' && target/debug/atelier work missions --help | rg \"Mission Overview|--all\" >/dev/null && COLUMNS=40 NO_COLOR= target/debug/atelier work missions | awk '\"'\"'length($0) > 40 { exit 1 } END { if (NR == 0) exit 1 }'\"'\"''"
exit_status: "0"
target:
  kind: "issue"
  id: "atelier-4fip"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-4fip"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "Independent scenario validation at 96dd1321dca6ddb8034086006f50e40542f195d2: PASS claim 1—work missions shows current mission/epic backlog with collapsed leaf work; PASS claim 2—default hides done, --all includes done, and counts derive from advances/descendant facts; PASS claim 3—shared renderer preserves complete semantics with TTY color, NO_COLOR, noninteractive, and COLUMNS=40. Fresh temp-repo/public-command matrix covered hierarchy, direct/unassigned accounting, blockers/progress, deterministic omissions/drilldowns, empty/quiet, and 100-character unbroken title. Actual pseudo-TTY comparison: 86 ANSI sequences, zero under NO_COLOR/noninteractive, stripped output equal. Help/docs parity and issue-list regression pass. Full atelier-app+atelier-cli suite: 568/568. No relevant ignored tests. Scope excludes inventory claims, atelier-g5fl, and mission closeout."
updated_at: "2026-07-07T06:27:15.422471007+00:00"
---

## Summary

Independent scenario validation at 96dd1321dca6ddb8034086006f50e40542f195d2: PASS claim 1—work missions shows current mission/epic backlog with collapsed leaf work; PASS claim 2—default hides done, --all includes done, and counts derive from advances/descendant facts; PASS claim 3—shared renderer preserves complete semantics with TTY color, NO_COLOR, noninteractive, and COLUMNS=40. Fresh temp-repo/public-command matrix covered hierarchy, direct/unassigned accounting, blockers/progress, deterministic omissions/drilldowns, empty/quiet, and 100-character unbroken title. Actual pseudo-TTY comparison: 86 ANSI sequences, zero under NO_COLOR/noninteractive, stripped output equal. Help/docs parity and issue-list regression pass. Full atelier-app+atelier-cli suite: 568/568. No relevant ignored tests. Scope excludes inventory claims, atelier-g5fl, and mission closeout.

## Command

```console
bash -lc 'cargo nextest run -p atelier-cli -E '"'"'test(/test_work_missions|mission_overview_|test_issue_list|test_list_issues/)'"'"' && cargo nextest run -p atelier-app -E '"'"'test(/mission_overview/)'"'"' && target/debug/atelier work missions --help | rg "Mission Overview|--all" >/dev/null && COLUMNS=40 NO_COLOR= target/debug/atelier work missions | awk '"'"'length($0) > 40 { exit 1 } END { if (NR == 0) exit 1 }'"'"''
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 4342
Truncated: yes

```text
   Compiling atelier-cli v0.2.0 (/root/.codex/worktrees/e613/atelier-c0mp-overview/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.99s
────────────
 Nextest run ID 8bcefd29-1e52-447d-8f1c-b98c26f6b766 with nextest profile: default
    Starting 18 tests across 4 binaries (450 tests skipped)
        PASS [   0.010s] ( 1/18) atelier-cli commands::work::mission_overview_render_tests::mission_overview_at_forty_columns_preserves_hierarchy_without_terminal_wrapping
        PASS [   0.016s] ( 2/18) atelier-cli commands::work::mission_overview_render_tests::empty_overview_keeps_a_record_browsing_drill_down
        PASS [   0.024s] ( 3/18) atelier-cli commands::work::mission_overview_render_tests::mission_overview_panel_renders_hierarchy_rollups_omissions_and_drill_downs
        PASS [   0.027s] ( 4/18) atelier-cli commands::work::mission_overview_render_tests::mission_overview_color_is_semantic_only_and_no_color_preserves_meaning
        PASS [   0.119s] ( 5/18) atelier-cli::cli_integration issues::test_issue_list_ready_rejects_closed_status
        PASS [   0.159s] ( 6/18) atelier-cli::cli_integration setup_guidance::test_mission_overview_help_and_manager_guidance_distinguish_plural_and_scoped_views
        PASS [   0.292s] ( 7/18) atelier-cli::cli_integration issues::test_issue_list_orders_visible_blockers_before_blocked_rows
        PASS [   0.318s] ( 8/18) atelier-cli::cli_integration issues::test_issue_list_blocked_replaces_blocked_helper
        PASS [   0.341s] ( 9/18) atelier-cli::cli_integration issues::test_issue_list_ready_excludes_blocked_and_quiet_matches_human_order
        PASS [   0.347s] (10/18) atelier-cli::cli_integration issues::test_list_issues
        PASS [   0.384s] (11/18) atelier-cli::cli_integration issues::test_issue_list_marks_external_epic_blockers_by_id
        PASS [   0.425s] (12/18) atelier-cli::cli_integration mission_cache_worktree::test_work_missions_hides_done_by_default_and_all_includes_done_without_expanding_work
        PASS [   0.500s] (13/18) atelier-cli::cli_integration issues::test_issue_list_ready_treats_internal_epic_blockers_as_ready
        PASS [   0.530s] (14/18) atelier-cli::cli_integration issues::test_issue_list_bounds_blocker_footer_actions
        PASS [   0.550s] (15/18) atelier-cli::cli_integration issues::test_issue_list_inventory_filters_across_status_type_and_state
        PASS [   0.559s] (16/18) atelier-cli::cli_integration issues::test_issue_list_ready_marks_blocked_parent_headers_as_context
        PASS [   0.635s] (17/18) atelier-cli::cli_integration mission_cache_worktree::test_work_missions_renders_collapsed_scope_exceptional_work_and_plain_quiet_output
        PASS [   0.719s] (18/18) atelier-cli::cli_integration issues::test_issue_list_ready_still_shows_ready_children_when_another_issue_is_active
────────────
     Summary [   0.720s] 18 tests run: 18 passed, 450 skipped
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
────────────
 Nextest run ID 40d9d545-4bdd-43e7-b130-d338a7398a03 with nextest profile: default
    Starting 6 tests across 1 binary (94 tests skipped)
        PASS [   0.008s] (1/6) atelier-app mission_overview::tests::excludes_done_missions_and_reports_exceptional_work_until_all_is_requested
        PASS [   0.009s] (2/6) atelier-app mission_overview::tests::orders_rows_deterministically_and_applies_budgets_after_counting
        PASS [   0.009s] (3/6) atelier-app mission_overview::tests::projects_directed_epics_direct_work_descendants_and_blocker_state
        PASS [   0.010s] (4/6) atelier-app mission_overview::tests::deduplicates_overlapping_roots_per_mission_but_preserves_shared_membership
        PASS [   0.011s] (5/6) atelier-app mission_overview::tests::cyclic_hierarchy_is_bounded_and_missing_or_reversed_links_do_not_assign_work
        PASS [   0.132s] (6/6) atelier-app mission_overview::tests::acquires_directed_cache_facts_and_projects_the_command_model
────────────
     Summary
```
