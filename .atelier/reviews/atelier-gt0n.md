---
schema: atelier.review
schema_version: 2
id: atelier-gt0n
title: 'crates/atelier-app/src/record_index.rs:99 — name_record_file treats the state-relative form (reviews/x.md) as naming the file, and the review parser (crates/atelier-records/src/review.rs:65,67,70) writes only that form, so review load errors dropped the only openable path. Master printed ''Invalid record file .atelier/reviews/base-kmy4.md: invalid review reviews/base-kmy4.md: ...''; this branch prints ''invalid review reviews/base-kmy4.md: ...'' in issue list and check (reproduced for bad YAML, unknown field, missing field, bad timestamp, bad event, wrong schema_version, id mismatch). reviews/x.md does not exist from the repo root and every issue and evidence error uses .atelier/<kind>/x.md. The unit test at record_index.rs:1488-1495 pins the regression as correct, and no integration test covers a broken review. Fix: have review.rs render .atelier/reviews/x.md and match the display form in name_record_file; add a review case to test_check_reports_every_invalid_record_file.'
target: atelier-x68o
identity: Independent reviewer (Claude subagent)
commit: dfe2703061fdcb63795e63352bf4895baf2a08dd
created_at: 2026-10-17T08:52:11.481826712Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: ed459adfce889368564f35f78527cd2fdc50a886
    recorded_at: 2026-10-17T08:55:37.584809882Z
    resolution_commit: 8d98633d2cedf3586f72ed3860aa161f980bc05f
    reason: Review errors print .atelier/reviews/x.md and name_record_file matches only the .atelier form; the check test covers a broken review.
    migrated_from: atelier-t1qv
---

crates/atelier-app/src/record_index.rs:99 — name_record_file treats the state-relative form (reviews/x.md) as naming the file, and the review parser (crates/atelier-records/src/review.rs:65,67,70) writes only that form, so review load errors dropped the only openable path. Master printed 'Invalid record file .atelier/reviews/base-kmy4.md: invalid review reviews/base-kmy4.md: ...'; this branch prints 'invalid review reviews/base-kmy4.md: ...' in issue list and check (reproduced for bad YAML, unknown field, missing field, bad timestamp, bad event, wrong schema_version, id mismatch). reviews/x.md does not exist from the repo root and every issue and evidence error uses .atelier/<kind>/x.md. The unit test at record_index.rs:1488-1495 pins the regression as correct, and no integration test covers a broken review. Fix: have review.rs render .atelier/reviews/x.md and match the display form in name_record_file; add a review case to test_check_reports_every_invalid_record_file.
