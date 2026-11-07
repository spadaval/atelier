---
schema: atelier.review
schema_version: 2
id: atelier-w0m7
title: finding on atelier-hwxm @b6903e91
target: atelier-hwxm
identity: lighter-workflow reviewer for atelier-hwxm (claude-opus-5-5)
commit: b6903e91654d68c5664e05ef2c1951302fe6f538
created_at: 2026-11-07T01:03:10.265857350Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: d530bb2e71c52b8d97b47f8f74b29b01daa3c03d
    recorded_at: 2026-11-07T01:13:36.518010917Z
    resolution_commit: a9620335c14e45754a6919a5b79b2b3540502767
    reason: disposition_independent now traces a migrated defer or no_action to its event's commit only while it is read from the event or the file has held it since the migration commit that removed the event; otherwise at the commit that set the field, so a hand restore is judged where it is made. The migration test covers change-then-restore (refused once the identity has authored work) and a fold edited inside the migration commit.
---

low: crates/atelier-app/src/review_gate.rs:1942 A migrated defer/no_action that is replaced and later restored by hand is judged at its old event's commit, not at the restoring commit, so the non-author rule does not hold for it; ADR 0025 decision 4 and docs/product/reviews-and-merge.md:82 say restoring earlier bytes is judged at the later commit. disposition_independent takes the migrated_from branch without consulting disposition_set_at. Reproduced: v1 finding, Helper no_action event (old binary), migrate --apply, Helper authors a commit, disposition reset to Other, then the migrated bytes copied back: doctor passes; the control (Helper sets it again natively) is refused by findings.disposed. Fix: judge at the event's commit only while the file has held the folded disposition since the commit that removed the event (e.g. disposition_set_at's commit also deletes .atelier/reviews/<migrated_from>.md), else at disposition_set_at; add the restore case to migrating_dispositions_folds_events_into_findings_and_passes_doctor. Or, if judged not worth code, correct the ADR and doc claim.
