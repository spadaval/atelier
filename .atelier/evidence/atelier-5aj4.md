---
created_at: "2026-10-22T22:24:02.009332853+00:00"
id: "atelier-5aj4"
evidence_type: "test"
commit: "700e102e8189bdd3b6fc42cc57ba68f748b43892"
result: "pass"
captured_at: "2026-10-22T22:23:52.662255689+00:00"
command: "cargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim"
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
title: "A commit adding an issue that advances a mission lands on the base and on a work branch and passes merge --check while an old done sibling's proof fails; doctor still reports it; the mission's own claim and blocker/own-record re-checks still apply"
updated_at: "2026-10-22T22:24:02.009707451+00:00"
---

## Summary

A commit adding an issue that advances a mission lands on the base and on a work branch and passes merge --check while an old done sibling's proof fails; doctor still reports it; the mission's own claim and blocker/own-record re-checks still apply

## Command

```console
cargo nextest run -j 4 -p atelier-cli --test session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it land_checks_a_mission_whose_advancing_work_reopens land_checks_claims_that_linked_records_or_removed_proof_affect land_checks_only_changed_claims_and_doctor_checks_every_claim
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 954
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
   Compiling atelier-cli v0.2.0 (/home/supadava/projects/atelier-wt/atelier-r28p/crates/atelier-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.56s
────────────
 Nextest run ID 8cab863c-b0db-4c3a-94cd-e8decb2b7082 with nextest profile: default
    Starting 4 tests across 1 binary (100 tests skipped)
        PASS [   0.441s] (1/4) atelier-cli::session land_checks_a_mission_whose_advancing_work_reopens
        PASS [   0.733s] (2/4) atelier-cli::session adding_an_issue_to_a_mission_rechecks_no_old_done_claim_in_it
        PASS [   1.153s] (3/4) atelier-cli::session land_checks_only_changed_claims_and_doctor_checks_every_claim
        PASS [   1.452s] (4/4) atelier-cli::session land_checks_claims_that_linked_records_or_removed_proof_affect
────────────
     Summary [   1.453s] 4 tests run: 4 passed, 100 skipped
```

