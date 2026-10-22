---
created_at: "2026-10-22T22:55:51.001671492+00:00"
id: "atelier-566f"
evidence_type: "test"
commit: "93698dad3dcc53c8f267216dbb447b6f484c9d9a"
result: "pass"
captured_at: "2026-10-22T22:54:15.464657781+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session lease start_ status"
exit_status: "0"
agent_identity: "claude-implementer-fngz"
target:
  kind: "issue"
  id: "atelier-x5j3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-x5j3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "A failed lease write after start warns; status refresh rechecks the holder under a lock (race test fails without the recheck)"
updated_at: "2026-10-22T22:55:51.001965219+00:00"
---

## Summary

A failed lease write after start warns; status refresh rechecks the holder under a lock (race test fails without the recheck)

## Command

```console
cargo nextest run -j 4 -p atelier-cli --features test-hooks --test session lease start_ status
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 3408
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-fngz/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.42s
────────────
 Nextest run ID 1005242d-973d-4d26-89b3-458e3858540a with nextest profile: default
    Starting 25 tests across 1 binary (83 tests skipped)
        PASS [   0.172s] ( 1/25) atelier-cli::session cross_branch_status_keeps_base_clean_and_reports_legacy_without_loading_it
        PASS [   0.302s] ( 2/25) atelier-cli::session pruned_owner_labels_cover_other_branch_rows_and_an_uncommitted_status
        PASS [   0.375s] ( 3/25) atelier-cli::session execute_bits_hidden_by_core_filemode_follow_git_status_in_land
        PASS [   0.094s] ( 4/25) atelier-cli::session start_cannot_switch_away_from_hidden_modified_code
        PASS [   0.330s] ( 5/25) atelier-cli::session scoped_views_offer_start_only_for_work_that_passes_the_readiness_gates
        PASS [   0.513s] ( 6/25) atelier-cli::session a_task_parent_owns_its_children_through_start_land_and_merge
        PASS [   0.252s] ( 7/25) atelier-cli::session start_and_status_warn_when_a_branch_owner_has_a_needs_human_child
        PASS [   0.126s] ( 8/25) atelier-cli::session start_child_installs_owner_metadata_and_child_transition_together
        PASS [   0.111s] ( 9/25) atelier-cli::session start_refuses_a_branch_whose_type_prefix_does_not_match_the_issue
        PASS [   0.123s] (10/25) atelier-cli::session start_existing_remote_branch_uses_its_tip_and_target_records
        PASS [   0.122s] (11/25) atelier-cli::session start_warns_instead_of_failing_when_the_lease_cannot_be_written
        PASS [   0.194s] (12/25) atelier-cli::session start_refuses_an_item_a_live_lease_holds_until_taken_over
        PASS [   0.153s] (13/25) atelier-cli::session start_replaces_a_stale_lease_with_a_note
        PASS [   0.147s] (14/25) atelier-cli::session start_with_a_base_creates_the_owner_branch_from_a_detached_head
        PASS [   0.189s] (15/25) atelier-cli::session start_with_a_base_ignores_the_current_branch_and_checks_the_base
        PASS [   0.196s] (16/25) atelier-cli::session start_with_a_base_works_from_a_checkout_without_the_issue_record
        PASS [   0.199s] (17/25) atelier-cli::session status_and_active_landing_are_fresh_and_read_only
        PASS [   0.205s] (18/25) atelier-cli::session status_branch_row_shows_upstream_divergence_only_when_it_differs
        PASS [   0.095s] (19/25) atelier-cli::session status_current_work_row_reads_like_its_owner_row
        PASS [   0.110s] (20/25) atelier-cli::session status_does_not_write_the_old_holder_back_over_a_take_over
        PASS [   0.229s] (21/25) atelier-cli::session status_owner_block_shows_base_mission_strategy_and_eight_newest_reviews
        PASS [   0.187s] (22/25) atelier-cli::session status_shows_policy_fallback_diagnostics_in_the_page_and_hook_context
        PASS [   0.311s] (23/25) atelier-cli::session status_keeps_eight_newest_scoped_proofs_and_fresh_hidden_dirt
        PASS [   0.148s] (24/25) atelier-cli::session status_shows_the_owner_decisions_in_brief
        PASS [   0.544s] (25/25) atelier-cli::session status_preserves_native_branch_trimming_for_header_owner_and_orientation
────────────
     Summary [   1.574s] 25 tests run: 25 passed, 83 skipped
```

