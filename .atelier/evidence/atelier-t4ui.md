---
created_at: "2026-10-22T22:41:01.522053966+00:00"
id: "atelier-t4ui"
evidence_type: "test"
commit: "1e5098895b49dc7f65e74a604b03b6da9bdaa57a"
result: "pass"
captured_at: "2026-10-22T22:40:57.439487716+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it merge_refuses_a_done_mission_the_branch_gives_open_work land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim"
exit_status: "0"
agent_identity: "claude-implementer-r28p"
target:
  kind: "issue"
  id: "atelier-ss25"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ss25"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Under both merge strategies, adding an issue to a mission re-checks no old done sibling in land (base and branch) or merge --check, doctor still reports it, and merge refuses a done mission the branch gives open work as land does"
updated_at: "2026-10-22T22:41:01.522554829+00:00"
---

## Summary

Under both merge strategies, adding an issue to a mission re-checks no old done sibling in land (base and branch) or merge --check, doctor still reports it, and merge refuses a done mission the branch gives open work as land does

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it merge_refuses_a_done_mission_the_branch_gives_open_work land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 1002
Truncated: no

```text
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s
────────────
 Nextest run ID 1cf44ace-b1ea-49c0-96c4-4e3d7f7d9f6b with nextest profile: default
    Starting 5 tests across 1 binary (100 tests skipped)
        PASS [   0.463s] (1/5) atelier-cli::session land_checks_a_mission_whose_advancing_work_reopens
        PASS [   1.707s] (2/5) atelier-cli::session land_checks_only_changed_claims_and_doctor_checks_every_claim
        PASS [   1.547s] (3/5) atelier-cli::session merge_refuses_a_done_mission_the_branch_gives_open_work
        PASS [   2.085s] (4/5) atelier-cli::session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it
        PASS [   2.124s] (5/5) atelier-cli::session land_checks_claims_that_linked_records_or_removed_proof_affect
────────────
     Summary [   2.124s] 5 tests run: 5 passed, 100 skipped
```

