---
created_at: "2026-10-22T20:37:53.542139349+00:00"
id: "atelier-5h93"
evidence_type: "test"
commit: "f94ba14a547a5361909855d328787ffa74dd3ddf"
result: "pass"
captured_at: "2026-10-22T20:37:51.536223423+00:00"
command: "cargo nextest run -j 4 --test merge_gates -E 'test(/reruns_the_gates|base_that_moved_by_code|base_moves_during|records_only|base_that_moved_by_records|divergence_and/)'"
exit_status: "0"
agent_identity: "claude-implementer-p189"
target:
  kind: "issue"
  id: "atelier-p189"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-p189"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Merge reruns gates for records-only base moves under merge_commit, refuses code moves naming the commit, and names commits under fast_forward_only"
updated_at: "2026-10-22T20:37:53.542679426+00:00"
---

## Summary

Merge reruns gates for records-only base moves under merge_commit, refuses code moves naming the commit, and names commits under fast_forward_only

## Command

```console
cargo nextest run -j 4 --test merge_gates -E 'test(/reruns_the_gates|base_that_moved_by_code|base_moves_during|records_only|base_that_moved_by_records|divergence_and/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1232
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-c3kh/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.25s
────────────
 Nextest run ID 6ff8b5ba-61bc-4d39-a405-73741af44d7e with nextest profile: default
    Starting 6 tests across 1 binary (53 tests skipped)
        PASS [   0.162s] (1/6) atelier-cli::merge_gates fast_forward_strategy_names_records_only_base_commits_it_cannot_land_on
        PASS [   0.189s] (2/6) atelier-cli::merge_gates merge_commit_strategy_refuses_a_base_that_moved_by_code_during_checks_naming_the_commit
        PASS [   0.193s] (3/6) atelier-cli::merge_gates divergence_and_dirty_candidate_or_base_checkout_are_refused
        PASS [   0.212s] (4/6) atelier-cli::merge_gates fast_forward_strategy_does_not_rerun_checks_for_a_base_that_moved_by_records
        PASS [   0.315s] (5/6) atelier-cli::merge_gates merge_commit_strategy_reruns_the_gates_when_the_base_moves_only_by_records_during_checks
        PASS [   0.362s] (6/6) atelier-cli::merge_gates merge_commit_strategy_refuses_when_the_base_moves_during_checks
────────────
     Summary [   0.524s] 6 tests run: 6 passed, 53 skipped
```

