---
created_at: "2026-11-07T05:00:24.456556181+00:00"
id: "atelier-3jdm"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-3jdm
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T05:42:10.098775055+00:00"
status: "done"
title: "Judge an approval by the code it saw, not the records around it"
updated_at: "2026-11-07T05:42:10.098775055+00:00"
---

## Goal

Today an approval goes stale when almost any tracker record on the branch changes after it, unless the edit is on a short allowed list (status, timestamps, Close Reason, new review and evidence records). Agents hit this dozens of times: ticking Done when boxes at close, adding a note, linking a follow-up, recording dispositions. Almost every time, a reviewer re-approved a records-only change and found nothing. Make an approval stale only when code changed after it, so record edits never cost a re-review, while any code change, including a fix after review, still does.

## Done when

- [x] A test approves an item, then on the branch edits, one per commit: the item's Done when ticks, Notes, links, status and Close Reason; another issue's record; a finding's disposition; a new evidence receipt. After each, `atelier merge --check` reports the approval current.
- [x] A test changes a source file after approval, and `merge --check` reports the approval stale and names the file. Changing `.atelier/workflow.yaml` or `.atelier/config.toml` after approval also makes it stale.
- [x] A clean merge from the base keeps the approval current, and a conflicted merge whose resolution changes code makes it stale (existing tests kept or adjusted).
- [x] The astromech-l5c6 sequence, closing and ticking Done when after approval, merges without a second approval in a scenario test.
- [x] Approval freshness no longer reads record contents: `allowed_edge`, `issue_status_only` and their helpers in `crates/atelier-app/src/review_gate.rs` are gone or no longer used for freshness. The append-only check for review records keeps its own tests passing.
- [x] docs/product and docs/architecture describe approval freshness as: stale only when code changed since the approved commit, not counting a clean merge from the base.

## Scope

Approval freshness in merge, land and status; their tests and docs. Must not change: merge refuses without a current approval; a fix commit after a `fix_now` finding makes the approval stale, so the fix is re-reviewed; a Done when edited after start is still shown to the reviewer (atelier-r9hb). The agent-factory skill text is updated later, by the item that rewrites the skill.

## Context

- Code means everything except `.atelier/issues`, `.atelier/reviews`, `.atelier/evidence` (and `.atelier/runtime`), the same split `scripts/merge-ci-passed.sh` uses.
- `crates/atelier-app/src/review_gate.rs` (`allowed_edge`, `issue_status_only`, `review_change_allowed`), `crates/atelier-app/src/git_freshness.rs`.
- Other users of the same freshness rule: `author_identities_to` and `allowed_base_merge` in review_gate.rs, and trivial exemptions, which are judged by the same rule as approvals (review_gate.rs:430). Each keeps working on the new rule; a trivial exemption also goes stale only on code changes.
- One installed binary serves every repository on this host; this change reads no new record format, so no migration is needed.

## Design

docs/strategies/simpler-lifecycle.md, Shape: "Approvals cover code".

## Notes

- 2026-11-07 [decision] Code is everything git_freshness::is_record_path does not match (Markdown under .atelier/issues, reviews, evidence); .atelier/runtime is not listed separately
  Reason: is_record_path already defines the record/code split for the clean-checkout checks, so freshness reuses that one owner instead of a second list; .atelier/runtime is gitignored and never committed, so excluding it changes nothing, and a force-added runtime file counting as code errs toward review
  Rejected: a separate path list copied from scripts/merge-ci-passed.sh
- 2026-11-07 [decision] A base merge is clean, and keeps the approval, when its tree matches Git's own merge of its parents everywhere outside the records; record conflicts and record edits while merging do not matter, and the atelier-1isk three-way check of resolved records is removed
  Reason: records are not what an approval covers, and a record edit in a later commit would keep the approval anyway, so judging record resolutions protected nothing; the gates still read the resolved records, so a kept blocker still holds the merge. merge_resolution/MergeResolution became one hand_resolved_paths helper that clean_merge also uses
  Rejected: keep the three-way check so a resolution cannot drop a blocker the base added
- 2026-11-07 [decision] Freshness functions no longer take the approved scope (ids), and author identities count only the authors of commits that change code
  Reason: with records ignored the scope no longer affects freshness, so the parameter was dead; independence is judged by the same rule an approval goes stale by, so someone who only edited records is not an author of the work. The scope still decides which done claims an approval covers (claims.reviewed, land_base_done_claim)
- 2026-11-07 [decision] land on the base accepts a done record edited after merge, such as a loosened Done when; proof_timing's carrying_a_check test now asserts it lands
  Reason: land judges the historical approval by the same rule as merge, and that edit changes no code; Done when edits after start stay visible through the atelier-r9hb note in merge

## Close Reason

An approval now goes stale only when code changes after its pin: anything outside the tracker records, with workflow policy and config counting as code, and a merge of the base keeps it when the merge matches Git's own merge outside the records. Record edits never make it stale. Proof, all run after the last code change on 56c6403f (cargo nextest, crates/atelier-cli/tests): record_edits_after_approval_keep_it_current (tick, note, link, another issue's title and status, disposition, new receipt, edited receipt, close; each its own commit, review.current current after each, then merge passes on the one approval); code_policy_and_config_changes_stale_an_approval_naming_the_file (src.rs, .atelier/workflow.yaml, .atelier/config.toml each refused with the stale line naming the file); clean_merge_of_the_base_keeps_approval_but_resolved_or_edited_merges_do_not, base_merge_record_resolutions_keep_approval_but_code_edited_while_merging_does_not, base_merge_record_resolutions_keep_approval_and_the_gates_read_what_they_kept, sync_keeps_the_approval_across_a_base_edit_to_the_owner_record, gates_name_the_base_merge_that_resolved_code_by_hand; closing_and_ticking_after_approval_merges_on_that_approval (the astromech-l5c6 sequence; merge and land pass on one approval); allowed_edge, issue_status_only, body_without_close_reason, ReadContext::status_only, allowed_base_merge, record_three_way and merged are deleted, and review_rewrites_deletions_and_modes_are_rejected_even_before_new_approval and separate_disposition_events_cannot_be_deleted_or_restored_by_hand pass; docs/product/reviews-and-merge.md (Approval freshness), work-model.md and docs/architecture/markdown-first-record-store.md state the rule. Nine of the new or adjusted tests fail against the old code; the three that pass there check staleness that must not change. cargo nextest run -j 4 -p atelier-cli -p atelier-app: 883 passed; with --features test-hooks: 889 passed; cargo fmt -- --check and git diff --check clean; target/debug/atelier check passes.

### Deviations
Tests that asserted the old rule were changed, not kept: code_policy_acceptance_and_notes_changes_require_fresh_approval became the code/policy/config test; out_of_scope_statuses_relationship_changes_and_existing_proof_edits_stale_approval was folded into record_edits_after_approval_keep_it_current and deleted; the base-merge record-resolution, links, new-child and sync tests now assert the approval is kept; gates_name_the_base_merge_that_changed_an_in_scope_record_beyond_status became gates_name_the_base_merge_that_resolved_code_by_hand; session's a_close_after_approval_is_covered_by_merge_and_land_alike moves with_goal and with_note to the covered cases; proof_timing's test is renamed carrying_a_check_or_editing_a_done_record_on_the_base_lands and asserts a loosened Done when on the base lands. Two unit tests in review_gate.rs (list_resolutions_keep_both_sides..., exact_epic_migration_is_status_only...) went with the code they tested. Beyond the item's files, CONTEXT.md's Freshness entry and merge's and session's "merge it in" guidance now say what keeps an approval.

### Decisions
Four decision notes on this item: code is what git_freshness::is_record_path does not match (.atelier/runtime is gitignored, so not listed); a base merge is clean when it matches Git's merge outside the records, and the atelier-1isk three-way check of resolved records is gone; freshness no longer takes the approved scope, and author identities count only code authors; land on the base accepts a done record edited after merge. Stale-edge paths now list only code.

### Surprises
merge_resolution treated a base merge with a record edited while merging, not just a record conflict, as unclean, which would have made the base commits it brought in count as branch code; the new clean-merge test covers it (edit_while_merging case).

### Open items
Independent review is outstanding (the orchestrator arranges it). The agent-factory skill text still describes the allowed-edit list; the item that rewrites the skill updates it, as Scope says. Ticking a Done when box alone is still reported by the atelier-r9hb note in merge; the lifecycle item owns that.
