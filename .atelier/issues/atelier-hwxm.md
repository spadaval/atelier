---
created_at: "2026-11-06T21:26:18.397443944+00:00"
id: "atelier-hwxm"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-hwxm
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5fus"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-efnf"
    type: "related"
  - kind: "issue"
    id: "atelier-ma4a"
    type: "related"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T00:50:06.783059752+00:00"
status: "done"
title: "A review finding carries its disposition in one record"
updated_at: "2026-11-07T00:50:06.783059752+00:00"
---

## Goal

A finding and what was done about it are one record: the disposition (fix_now with resolution commit, defer with follow-up, no_action with reason) becomes fields on the finding, with who set it and at which commit, instead of a separate immutable event. Existing repositories migrate with one atelier migrate step.

## Done when

review files per finding drop to 1 in the lifecycle scenario; merge and land gates read dispositions from findings; the non-author rules for defer and no_action still hold, with who set the disposition traced per field rather than by the commit that added a separate file (today original_independent in crates/atelier-app/src/review_gate.rs); recording a disposition does not make an approval stale (today editing existing review records does, docs/product/reviews-and-merge.md); an atelier migrate step folds existing disposition events into their findings and the fixture repositories pass atelier doctor after it; an ADR records the change and the part of ADR 0019's append-only events it supersedes, and PRODUCT_INTENT.md, CONTEXT.md and the Agent Factory text describing two events (references/finding-disposition.md, references/atelier.md in ~/projects/agents-skills) are updated; the same docs say the re-review after a fix_now may cover only the fix's diff and may be done by the same reviewer.

## Unknowns

How a disposition changed later (fix_now then defer) stays auditable without a second record: the implementer decides (Git history of the finding's file is one option) and records the choice under this item and in the ADR. How atelier-efnf's readiness view and atelier-ma4a's review history read the new shape, if they land first.

## Notes

- 2026-11-06 [decision] A changed disposition is audited through the Git history of the finding's file: the record keeps only the current disposition, review show names git log -p for the earlier ones, and who set a defer or no_action is judged at the oldest commit since which the file has held the current disposition, not the commit or time it names. Recorded in ADR 0025 decision 5.
  Reason: Git already keeps every version of the file, each pinned to a commit and author, and the gates read only the current disposition, as they read only the latest event before; a record per change would be the second record this item removes.
  Rejected: A list of earlier dispositions inside the finding: it duplicates Git and the gates never read it
  Rejected: Separate successor events for changes only: two shapes for one fact

## Close Reason

A finding now carries its disposition as a field (decision, identity, commit, time, resolution or follow-up, reason), so the lifecycle scenario counts one review file per finding (was 2/3/3); the merge and land gates read it from the finding, defer and no_action are judged by who set the field at the commit that set it, recording one keeps an approval current, and `atelier migrate dispositions --apply` folds existing events in (ADR 0025; docs and Agent Factory text updated, including that a fix_now re-review may cover only the fix's diff and be by the same reviewer).

Targeted proof, all passing after the last change: `lifecycle_cost` (review files per finding 1/1/1, record-only commits still 13); session.rs `migrating_dispositions_folds_events_into_findings_and_passes_doctor` (fixture repository with schema-1 findings and events, author proposal superseded by the reviewer, dry run, apply, doctor and land pass, a successor event from an older binary folds, a hand-edited migrated disposition is refused), `migrating_dispositions_on_an_approved_branch_keeps_the_approval`, `a_migrated_author_no_action_is_still_refused`, `a_disposition_is_judged_at_the_commit_that_set_it` (the same no_action accepted before its identity authored work, refused when set again after, and refused when the earlier bytes are restored by hand), `a_disposition_after_approval_keeps_it_current` (fix_now then no_action after approval keep review.current; editing the finding's text is refused), `an_uncommitted_disposition_is_committed_not_restored`; record_index `a_separate_disposition_event_points_at_the_migration`. Full `cargo nextest run`, with and without `test-hooks`, passed locally (970 and 976); fmt and diff-check clean. Receipt atelier-cnk7 records the migration on copies of atelier, astromech, demiurge and cosplay-studio.

### Deviations
This branch also migrates this repository's own records (577 findings, 580 events removed, commit 1c952fe1), since the new binary refuses separate disposition events in the checkout. `review disposition`'s quiet output is now the finding's ID, and its confirmation line changed (`Disposition <decision> of <finding> on <issue> @<pin>: "<finding>", replacing <decision> by <identity>`).

### Decisions
How a changed disposition stays auditable: Git history of the finding's file (decision note of 2026-11-06 on this item; ADR 0025 decision 5). Review records move to schema version 2 so older binaries say they need a newer Atelier; version 1 records without separate dispositions load unchanged. A migrated disposition keeps `migrated_from` naming its event, so who set it is traced to the commit that added that event and must match it. One rule (`review_change_allowed`) decides both append-only and approval freshness for review files.

### Surprises
Historical fix_now acceptance compared the finding's bytes on the base (`retained`), which every migrated finding fails; it now compares the disposition itself, or the event it was migrated from. cosplay-studio's doctor already fails on legacy `.yaml` review files in its history, with or without this change. An uncommitted disposition used to be advised as "restore from HEAD"; land now names it `disposition changed` and commits it.

### Open items
After this merges, reinstall the CLI (`cargo install --path crates/atelier-cli --locked`) before running land or hooks here: the installed binary reads the migrated findings as a newer schema. Every open work branch that recorded dispositions with the old binary, including atelier-s66u's if it did, must run `atelier migrate dispositions --apply` after merging master. astromech, demiurge and cosplay-studio need the same migration when they upgrade. The lifecycle-cost criterion's record-only commit count is unchanged at 13; atelier-5822 owns that cut.
