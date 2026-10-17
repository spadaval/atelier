---
created_at: "2026-10-17T18:16:30.366260483+00:00"
id: "atelier-gc5j"
evidence_type: "test"
commit: "c986148ae012a5e89cf33a485e138ef185578587"
result: "pass"
captured_at: "2026-10-17T18:16:28.551756723+00:00"
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
title: "work mission criterion mark tests pass"
updated_at: "2026-10-17T18:16:30.366882726+00:00"
---

## Summary

work mission criterion mark tests pass

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

Bytes: 1199
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-ho7o/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.04s
────────────
 Nextest run ID b6268607-08ee-40c5-bbe7-0934ba0cc066 with nextest profile: default
    Starting 6 tests across 1 binary (322 tests skipped)
        PASS [   0.088s] (1/6) atelier-cli::cli_integration criteria::receipts_record_and_show_a_result_per_criterion
        PASS [   0.109s] (2/6) atelier-cli::cli_integration criteria::check_refuses_receipts_naming_undefined_or_renamed_criteria
        PASS [   0.114s] (3/6) atelier-cli::cli_integration criteria::captured_receipts_give_each_criterion_the_command_result
        PASS [   0.134s] (4/6) atelier-cli::cli_integration criteria::receipts_refuse_criteria_their_mission_does_not_define
        PASS [   0.123s] (5/6) atelier-cli::cli_integration criteria::work_mission_reads_criteria_from_the_linked_strategy
        PASS [   0.241s] (6/6) atelier-cli::cli_integration criteria::work_mission_marks_each_criterion_from_its_newest_receipt
────────────
     Summary [   0.330s] 6 tests run: 6 passed, 322 skipped
```

