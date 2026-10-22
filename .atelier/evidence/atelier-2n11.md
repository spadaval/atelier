---
created_at: "2026-10-22T22:52:31.835519855+00:00"
id: "atelier-2n11"
evidence_type: "test"
commit: "75d2dc1389174d45d8362abf14b10dc0eef309a7"
result: "pass"
captured_at: "2026-10-22T22:52:19.052427269+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it merge_refuses_a_done_mission_the_branch_gives_open_work land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim"
exit_status: "0"
agent_identity: "atelier merge --sync"
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
title: "## Summary\n\nUnder both merge strategies, adding an issue to a mission re-checks no old done sibling in land (base and branch) or merge --check, doctor still reports it, and merge refuses a done mission the branch gives open work as land does\n\n## Command\n\n```console\ncargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it merge_refuses_a_done_mission_the_branch_gives_open_work land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim\n```\n\nExit status: 0\n\n## Stdout\n\nBytes: 0\nTruncated: no\n\n```text\n```\n\n## Stderr\n\nBytes: 1002\nTruncated: no\n\n```text\n   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)\n    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s\n────────────\n Nextest run ID 1cf44ace-b1ea-49c0-96c4-4e3d7f7d9f6b with nextest profile: default\n    Starting 5 tests across 1 binary (100 tests skipped)\n        PASS [   0.463s] (1/5) atelier-cli::session land_checks_a_mission_whose_advancing_work_reopens\n        PASS [   1.707s] (2/5) atelier-cli::session land_checks_only_changed_claims_and_doctor_checks_every_claim\n        PASS [   1.547s] (3/5) atelier-cli::session merge_refuses_a_done_mission_the_branch_gives_open_work\n        PASS [   2.085s] (4/5) atelier-cli::session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it\n        PASS [   2.124s] (5/5) atelier-cli::session land_checks_claims_that_linked_records_or_removed_proof_affect\n────────────\n     Summary [   2.124s] 5 tests run: 5 passed, 100 skipped\n```\n (rerun after merging master)"
updated_at: "2026-10-22T22:52:31.835833870+00:00"
---

## Summary

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
 (rerun after merging master)

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

Bytes: 1062
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
────────────
 Nextest run ID e1806c71-f552-4fbf-83ee-00ce1e8a3c81 with nextest profile: default
    Starting 5 tests across 1 binary (102 tests skipped)
        PASS [   0.294s] (1/5) atelier-cli::session land_checks_a_mission_whose_advancing_work_reopens
        PASS [   0.864s] (2/5) atelier-cli::session land_checks_only_changed_claims_and_doctor_checks_every_claim
        PASS [   0.737s] (3/5) atelier-cli::session merge_refuses_a_done_mission_the_branch_gives_open_work
        PASS [   1.075s] (4/5) atelier-cli::session land_checks_claims_that_linked_records_or_removed_proof_affect
        PASS [   1.097s] (5/5) atelier-cli::session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it
────────────
     Summary [   1.097s] 5 tests run: 5 passed, 102 skipped
```

