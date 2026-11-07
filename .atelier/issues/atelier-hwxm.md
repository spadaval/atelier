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

review files per finding drop to 1 in the lifecycle scenario; merge and land gates read dispositions from findings; the non-author rules for defer and no_action still hold, with who set the disposition traced per field rather than by the commit that added a separate file (today original_independent in crates/atelier-app/src/review_gate.rs); recording a disposition does not make an approval stale (today editing existing review records does, docs/product/reviews-and-merge.md); an atelier migrate step folds existing disposition events into their findings and the fixture repositories pass atelier doctor after it; an ADR records the change and the part of ADR 0019's append-only events it supersedes, and PRODUCT_INTENT.md, CONTEXT.md and the Agent Factory text describing two events (references/finding-disposition.md, references/atelier.md in ~/projects/agents-skills) are updated; the same docs say the re-review after a fix_now may cover only the fix's diff and may be done by the same reviewer; the new binary reads old separate disposition events as if folded into their findings, with the same results as after migration, so repositories that have not migrated keep working with the shared installed binary.

## Unknowns

How a disposition changed later (fix_now then defer) stays auditable without a second record: the implementer decides (Git history of the finding's file is one option) and records the choice under this item and in the ADR. How atelier-efnf's readiness view and atelier-ma4a's review history read the new shape, if they land first.

## Notes

- 2026-11-06 [decision] A changed disposition is audited through the Git history of the finding's file: the record keeps only the current disposition, review show names git log -p for the earlier ones, and who set a defer or no_action is judged at the oldest commit since which the file has held the current disposition, not the commit or time it names. Recorded in ADR 0025 decision 5.
  Reason: Git already keeps every version of the file, each pinned to a commit and author, and the gates read only the current disposition, as they read only the latest event before; a record per change would be the second record this item removes.
  Rejected: A list of earlier dispositions inside the finding: it duplicates Git and the gates never read it
  Rejected: Separate successor events for changes only: two shapes for one fact

## Close Reason

A finding now carries its disposition as a field (decision, identity, commit, time, resolution or follow-up, reason), so the lifecycle scenario counts one review file per finding (was 2/3/3); the merge and land gates read it from the finding, defer and no_action are judged by who set the field at the commit that set it, recording one keeps an approval current, and repositories with separate disposition events keep working unmigrated: the binary reads each finding's latest event as its disposition, with the same results as after `atelier migrate dispositions --apply`, which is now optional cleanup (ADR 0025; docs and Agent Factory text updated, including that a fix_now re-review may cover only the fix's diff and be by the same reviewer).

Targeted proof, all passing after the last change: `lifecycle_cost` (review files per finding 1/1/1, record-only commits still 13); session.rs `migrating_dispositions_on_an_approved_branch_keeps_the_approval` (an approved branch with schema-1 findings and events passes check, land, merge --check and merge with the same output unmigrated and migrated, then doctor and land on the base), `migrating_dispositions_folds_events_into_findings_and_passes_doctor` (unmigrated base passes check, doctor and land; dry run, apply, doctor and land pass; a successor event from an older binary folds; a migrated no_action changed and then restored by hand is judged at the restore, after its identity authored work, and refused), `a_migrated_author_no_action_is_still_refused` (refused unmigrated and migrated; a reviewer's `review disposition` writes the field and removes the event; a fold edited inside the migration commit is refused), `a_disposition_is_judged_at_the_commit_that_set_it`, `a_disposition_after_approval_keeps_it_current`, `an_uncommitted_disposition_is_committed_not_restored`; record_index `a_separate_disposition_event_reads_as_folded_into_its_finding`. Full `cargo nextest run`, with and without `test-hooks`, passed locally (970 and 976); fmt and diff-check clean. Receipt atelier-cnk7 records the migration on copies of atelier, astromech, demiurge and cosplay-studio; unmigrated clones of astromech, demiurge and life-on-rails also passed check, doctor and land with this binary (not recorded as a receipt).

### Deviations
This branch also migrates this repository's own records (577 findings, 580 events removed, commit 1c952fe1). The first round refused unmigrated events; on the orchestrator's direction it became expand-and-contract (commit a9620335), with atelier-42th filed for the contraction and its trigger. `review disposition`'s quiet output is now the finding's ID, and its confirmation line changed (`Disposition <decision> of <finding> on <issue> @<pin>: "<finding>", replacing <decision> by <identity>`). A Done when clause was added after start for the read path.

### Decisions
How a changed disposition stays auditable: Git history of the finding's file (decision note of 2026-11-06 on this item; ADR 0025 decision 5). Review records move to schema version 2 so older binaries say they need a newer Atelier; version 1 records still load. One fold function (`fold_retired_dispositions`) serves both the read path and the migration, so they cannot disagree. A disposition read from an event, or written by the migration commit that removed it, is judged at the event's commit and must match it; otherwise at the commit that set the field (fix for review finding atelier-w0m7). One rule (`review_change_allowed`) decides both append-only and approval freshness for review files.

### Surprises
Historical fix_now acceptance compared the finding's bytes on the base (`retained`), which every migrated finding fails; it now compares the disposition itself, or the event it came from. Prune would have left an unmigrated finding's events behind when pruning its cluster, so they are now candidates that go with it. cosplay-studio's doctor already fails on legacy `.yaml` review files in its history, with or without this change. An uncommitted disposition used to be advised as "restore from HEAD"; land now names it `disposition changed` and commits it.

### Open items
After this merges, reinstall the CLI (`cargo install --path crates/atelier-cli --locked`) before running land or hooks here: the old installed binary reads this repository's migrated findings as a newer schema. Other repositories keep working on the new binary without migrating. atelier-42th (backlog) removes the read path once every repository the shared binary serves has migrated; link it to a mission if wanted. The lifecycle-cost criterion's record-only commit count is unchanged at 13; atelier-5822 owns that cut.
