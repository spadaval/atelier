---
created_at: "2026-08-05T18:05:14.204258691+00:00"
id: "atelier-2aym"
evidence_type: "test"
captured_at: "2026-08-05T18:05:09.218705131+00:00"
command: "cargo nextest run -E 'test(/starter_policy_distinguishes_draft_epics_from_execution_ready_epics|validates_strategy_field_as_mission_only|issue_cache_round_trips_typed_strategy_projection|mission_record_round_trips_typed_strategy_reference|bundle_issue_omits_evidence_section_when_no_proof_is_planned|test_work_mission_dashboard_is_scoped_and_operational/)'"
exit_status: "0"
agent_identity: "codex"
target:
  kind: "issue"
  id: "atelier-js22"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-js22"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "recorded"
title: "PASS: focused adaptive mission contract tests"
updated_at: "2026-08-05T18:05:14.206076739+00:00"
---

## Summary

PASS: focused adaptive mission contract tests

## Command

```console
cargo nextest run -E 'test(/starter_policy_distinguishes_draft_epics_from_execution_ready_epics|validates_strategy_field_as_mission_only|issue_cache_round_trips_typed_strategy_projection|mission_record_round_trips_typed_strategy_reference|bundle_issue_omits_evidence_section_when_no_proof_is_planned|test_work_mission_dashboard_is_scoped_and_operational/)'
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1102
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/root/atelier/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.26s
────────────
 Nextest run ID ebfd7975-a937-44bb-b6b9-7fc80ac6fcf0 with nextest profile: default
    Starting 6 tests across 9 binaries (737 tests skipped)
        PASS [   0.008s] (1/6) atelier-records tests::mission_record_round_trips_typed_strategy_reference
        PASS [   0.010s] (2/6) atelier-cli commands::bundle::tests::bundle_issue_omits_evidence_section_when_no_proof_is_planned
        PASS [   0.012s] (3/6) atelier-workflow tests::validates_strategy_field_as_mission_only
        PASS [   0.016s] (4/6) atelier-workflow tests::starter_policy_distinguishes_draft_epics_from_execution_ready_epics
        PASS [   0.058s] (5/6) atelier-sqlite cache::tests::issue_cache_round_trips_typed_strategy_projection
        PASS [   2.378s] (6/6) atelier-cli::cli_integration issues::test_work_mission_dashboard_is_scoped_and_operational
────────────
     Summary [   2.380s] 6 tests run: 6 passed, 737 skipped
```

