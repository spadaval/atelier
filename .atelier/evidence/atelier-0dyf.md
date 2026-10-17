---
created_at: "2026-10-17T19:08:37.723204008+00:00"
id: "atelier-0dyf"
evidence_type: "test"
commit: "e6dcd73b6651da1f0658b89f7680e112dd3f3388"
result: "pass"
captured_at: "2026-10-17T19:08:31.353423469+00:00"
command: "cargo nextest run -j 4 --test cli_integration -E 'test(criteria::)'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-r6bt"
  role: "validates"
criteria:
- slug: "S8"
  result: "pass"
criteria_mission: "atelier-r6bt"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-r6bt"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "work mission criterion mark tests pass on master"
updated_at: "2026-10-17T19:08:37.723708709+00:00"
---

## Summary

work mission criterion mark tests pass on master

## Command

```console
cargo nextest run -j 4 --test cli_integration -E 'test(criteria::)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1817
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.80s
────────────
 Nextest run ID fae62358-5b25-4868-b8aa-7560e5088206 with nextest profile: default
    Starting 11 tests across 1 binary (327 tests skipped)
        PASS [   1.786s] ( 1/11) atelier-cli::cli_integration criteria::blocked_and_not_applicable_receipts_leave_a_criterion_unproven
        PASS [   1.818s] ( 2/11) atelier-cli::cli_integration criteria::captured_receipts_give_each_criterion_the_command_result
        PASS [   1.829s] ( 3/11) atelier-cli::cli_integration criteria::check_refuses_receipts_naming_undefined_or_renamed_criteria
        PASS [   1.905s] ( 4/11) atelier-cli::cli_integration criteria::a_receipt_pinned_outside_head_history_does_not_decide
        PASS [   0.131s] ( 5/11) atelier-cli::cli_integration criteria::receipts_record_and_show_a_result_per_criterion
        PASS [   0.133s] ( 6/11) atelier-cli::cli_integration criteria::scoped_check_of_a_mission_covers_receipts_in_its_scope
        PASS [   0.211s] ( 7/11) atelier-cli::cli_integration criteria::receipts_refuse_criteria_their_mission_does_not_define
        PASS [   0.281s] ( 8/11) atelier-cli::cli_integration criteria::proof_stays_with_the_mission_it_was_recorded_against
        PASS [   0.035s] ( 9/11) atelier-cli::cli_integration criteria::work_mission_needs_no_git_history
        PASS [   0.130s] (10/11) atelier-cli::cli_integration criteria::work_mission_reads_criteria_from_the_linked_strategy
        PASS [   0.277s] (11/11) atelier-cli::cli_integration criteria::work_mission_marks_each_criterion_from_its_newest_receipt
────────────
     Summary [   2.227s] 11 tests run: 11 passed, 327 skipped
```

