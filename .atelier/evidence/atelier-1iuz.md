---
created_at: "2026-10-17T18:42:51.227738876+00:00"
id: "atelier-1iuz"
evidence_type: "test"
commit: "fd32b14c1d5c1a7f2be65cae1ffbd998adb8944e"
result: "pass"
captured_at: "2026-10-17T18:42:48.942152244+00:00"
command: "cargo nextest run -j 4 --test cli_integration criteria::"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "atelier-stsa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-stsa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "work mission criterion tests pass after the review fixes"
updated_at: "2026-10-17T18:42:51.228402295+00:00"
---

## Summary

work mission criterion tests pass after the review fixes

## Command

```console
cargo nextest run -j 4 --test cli_integration criteria::
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1837
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ho7o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.42s
────────────
 Nextest run ID 8051af8d-4790-4e83-a913-7054946a00bb with nextest profile: default
    Starting 11 tests across 1 binary (322 tests skipped)
        PASS [   0.099s] ( 1/11) atelier-cli::cli_integration criteria::blocked_and_not_applicable_receipts_leave_a_criterion_unproven
        PASS [   0.121s] ( 2/11) atelier-cli::cli_integration criteria::check_refuses_receipts_naming_undefined_or_renamed_criteria
        PASS [   0.124s] ( 3/11) atelier-cli::cli_integration criteria::captured_receipts_give_each_criterion_the_command_result
        PASS [   0.167s] ( 4/11) atelier-cli::cli_integration criteria::a_receipt_pinned_outside_head_history_does_not_decide
        PASS [   0.095s] ( 5/11) atelier-cli::cli_integration criteria::receipts_record_and_show_a_result_per_criterion
        PASS [   0.100s] ( 6/11) atelier-cli::cli_integration criteria::scoped_check_of_a_mission_covers_receipts_in_its_scope
        PASS [   0.143s] ( 7/11) atelier-cli::cli_integration criteria::receipts_refuse_criteria_their_mission_does_not_define
        PASS [   0.196s] ( 8/11) atelier-cli::cli_integration criteria::proof_for_an_item_in_several_missions_counts_toward_none
        PASS [   0.032s] ( 9/11) atelier-cli::cli_integration criteria::work_mission_needs_no_git_history
        PASS [   0.125s] (10/11) atelier-cli::cli_integration criteria::work_mission_reads_criteria_from_the_linked_strategy
        PASS [   0.236s] (11/11) atelier-cli::cli_integration criteria::work_mission_marks_each_criterion_from_its_newest_receipt
────────────
     Summary [   0.453s] 11 tests run: 11 passed, 322 skipped
```

