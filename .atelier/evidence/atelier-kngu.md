---
created_at: "2026-10-22T23:07:45.074596386+00:00"
id: "atelier-kngu"
evidence_type: "test"
commit: "c5833bba72a3097a6c409f32903b490b43ccbb49"
result: "pass"
captured_at: "2026-10-22T23:07:24.553901336+00:00"
command: "cargo nextest run -j 4 -p atelier-app path_log ancestry_path"
exit_status: "0"
agent_identity: "claude-implementer-7b6e"
target:
  kind: "issue"
  id: "atelier-7b6e"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-7b6e"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "In-process record history answers equal git log for every query shape from every commit over merges, octopus, criss-cross, deleted and re-added paths, cherry-picks, grafts, shallow clones and backward or equal dates; ancestry_path equals rev-list --ancestry-path including a side branch dated before its parents"
updated_at: "2026-10-22T23:07:45.075068335+00:00"
---

## Summary

In-process record history answers equal git log for every query shape from every commit over merges, octopus, criss-cross, deleted and re-added paths, cherry-picks, grafts, shallow clones and backward or equal dates; ancestry_path equals rev-list --ancestry-path including a side branch dated before its parents

## Command

```console
cargo nextest run -j 4 -p atelier-app path_log ancestry_path
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1625
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
────────────
 Nextest run ID 2c740230-70e8-4faa-95ab-4b420f091f65 with nextest profile: default
    Starting 11 tests across 1 binary (95 tests skipped)
        PASS [   0.059s] ( 1/11) atelier-app path_log::tests::additions_list_records_resolved_or_renamed_into_being
        PASS [   0.118s] ( 2/11) atelier-app git_freshness::tests::ancestry_path_matches_git_for_octopus_merges_and_graft_cycles
        PASS [   0.239s] ( 3/11) atelier-app git_freshness::tests::ancestry_path_matches_git_on_branchy_history
        PASS [   0.475s] ( 4/11) atelier-app path_log::tests::matches_git_for_cherry_picked_review_files
        PASS [   0.602s] ( 5/11) atelier-app git_freshness::tests::ancestry_path_matches_git_when_a_side_branch_is_dated_before_its_parents
        PASS [   0.559s] ( 6/11) atelier-app path_log::tests::matches_git_for_deleted_and_re_added_paths
        PASS [   2.606s] ( 7/11) atelier-app path_log::tests::matches_git_in_shallow_clones
        PASS [   0.022s] ( 8/11) atelier-app path_log::tests::rejects_paths_and_starts_outside_the_indexed_history
        PASS [   2.820s] ( 9/11) atelier-app path_log::tests::matches_git_under_grafts
        PASS [   2.998s] (10/11) atelier-app path_log::tests::matches_git_over_merges_octopus_and_criss_cross
        PASS [   7.345s] (11/11) atelier-app path_log::tests::matches_git_when_dates_run_backwards_or_tie
────────────
     Summary [   8.022s] 11 tests run: 11 passed, 95 skipped
```

