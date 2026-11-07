---
created_at: "2026-11-07T05:01:00.003526374+00:00"
id: "atelier-jnrh"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-jnrh
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4yqd"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T07:09:01.071346567+00:00"
status: "done"
title: "Do work in one order: tick Done when, review, and let merge mark it done"
updated_at: "2026-11-07T07:09:01.071346567+00:00"
---

## Goal

Give the lifecycle one order that no source contradicts: start, implement, tick every Done when line and write the implementer report, review, fix and re-review, approve, merge. Merge marks the work done, so "done" means merged everywhere and there is no close step to put in the wrong place. Today the skill says close before review, the policy once had a review status, and `merge --check` says close after approval; every November orchestrator closed after approval.

## Done when

- [x] `atelier review request` refuses while any Done when line of the open work items it covers is unticked, or their `Report` section is empty, and lists what is missing. A line carrying the old After merge marker is not required to be ticked; the refusal's message names `atelier migrate after-merge` for such lines (that command arrives with atelier-8bh4).
- [x] `atelier merge` always creates a merge commit, and that commit sets every open item in the `work` workflow on the branch (the owner and its non-superseded children) to the workflow's first done status, keeping the `Report` the reviewer read. Merge refuses while one of them has an open blocker. Tests cover a standalone issue and a parent with children, and `land` passes on the base after each merge.
- [x] `merge_strategy: fast_forward_only`, `squash` and a missing `merge_strategy` are all read as `merge_commit`, and `merge_commit` no longer needs a shell merge check (`merge_unproven` is removed). The starter policy and atelier's own say `merge_commit` or nothing.
- [x] The `work` workflow has no `close` transition; `supersede` remains and writes the `Report` section. The required report parts are Agent Factory guidance, not policy: Atelier requires only a non-empty `Report`. No command output, policy description, hint or doc tells an agent to close a work item. An item already done on an unmerged branch (closed by an older binary) merges as it is.
- [x] Docs and the mission close hint say a mission closes when all its work items are merged and its validation, if it has one, is done. This is described, not a new refusal.
- [x] A change to a Done when line's text after start stays reported, as atelier-r9hb does today, including after approval and including a change merged in from the base (finding atelier-l5fb), in the review request brief and `merge --check`, without refusing; a change that only ticks boxes is no longer reported. Tests for each.
- [x] PRODUCT_INTENT.md and ADR 0028 describe the mechanics as built: decision 2's open paragraph is replaced by them, including the merge-strategy tradeoff below and how today's "Atelier never creates commits" principle now reads.
- [x] Removed, and listed in the implementer report: the `close` transition for work items, the policy's report `required_fields`, `fast_forward_only`, `merge_unproven`, the rule that a parent closes after its children, and the rules and messages about close order.

## Scope

Workflow policy (starter and atelier's own), transitions, `review request`, `merge`, `land`'s view of merge commits, mission close hints, `status`/`work` hints, docs/product, PRODUCT_INTENT.md, ADR 0028. Must not change: merge refuses without a current independent approval; a fix after review is re-reviewed. Other repositories keep working with the shared binary: old policies (`close` transitions with `required_fields`, `fast_forward_only`) and old records (`Close Reason`) still parse; updating their policies is atelier-ml3r's job.

## Context

docs/adr/0028-simpler-lifecycle.md (decision 2 and its open mechanics), docs/strategies/simpler-lifecycle.md (Shape: One order). The `loose_end` type and the after-merge migration are atelier-8bh4's.

## Design

- One section, `Report`, says what happened, for every transition that requires the `close_reason` policy field: the implementer report on a work item (written by the author before requesting review), a supersede reason, a mission's close report, a decision's `decide`, and later a loose end's result (atelier-8bh4). The policy key `close_reason` keeps its name and now means that section. The old `Close Reason` heading is still read.
- Merge marks done by writing the status change into the merge commit it creates; nothing is committed on the branch.
- Merge strategy tradeoff, for Sumeet's after-merge read: always merging with a merge commit gives up linear history and the `merge_diverged` refusal in astromech, demiurge and cosplay-studio, and a repository without merge checks or CI on the merged tree now lands a merged tree nobody ran. In exchange no branch needs a catch-up merge before landing (astromech hit `merge_diverged` about 56 times since 2026-10-15). Rejected: committing the done state on top of the branch head and fast-forwarding, which keeps two strategies and adds a second kind of commit Atelier writes. atelier-ml3r tells those repositories.

## Notes

- 2026-11-07 [decision] The Report is a renamed section: records write ## Report, the field is IssueSections::report, and a ## Close Reason heading is read into the same slot
  Reason: One section says what happened for every transition that requires close_reason; reading the old heading keeps every repository's closed records valid with the shared binary (expand and contract); a record with both headings is refused as a duplicate
  Rejected: Keep the Close Reason heading and only rename it in docs
- 2026-11-07 [decision] Only a bulleted Done when line with a [ ] box is a checklist item; review request asks those to be ticked and ignores lines without a box
  Reason: A prose Done when has nothing to tick, and requiring boxes everywhere would be a new rule; this repository and Agent Factory already write boxes
  Rejected: Treat every Done when line as needing a tick
- 2026-11-07 [decision] The work merge marks done is the open items whose issue type maps to the workflow named work (WorkflowPolicy::merge_done_status), owner first; review request asks the same set (review_gate::open_work)
  Reason: The item names the work workflow, every repository on this host uses that name, and one function owns the set both commands use
  Rejected: Every type other than mission and decision, as land's base_done_claims does
- 2026-11-07 [decision] Merge writes the done marks into Git's merge tree through a scratch index, runs the checks and gates on the resulting merge commit's records, and judges claims.reviewed on the branch's commits up to its head
  Reason: No checkout or ref changes before integration; an older policy's work close then applies its required fields and validators to the marks, and the merge commit's own marks are the scope review.current already judges, so they no longer fail claims.reviewed alongside it
  Rejected: Commit the done state on the branch head and fast-forward
  Rejected: Gate on the records before marking
- 2026-11-07 [decision] A Done when change counts when HEAD holds it and no start commit does (the change is not an ancestor of a start); boxes are read as unticked before comparing; a merge counts only where it differs from every parent
  Reason: This reports edits merged in from the base (atelier-l5fb) and after approval while keeping per-commit attribution and the finding or receipt it may answer
  Rejected: Compare the Done when at the start commit with HEAD's
- 2026-11-07 [decision] A two-parent merge on the base introduces only paths that differ from every parent and from Git's own merge, and land no longer counts a clean merge or a merge whose only changes are approved done marks as a record-only commit outside review
  Reason: The merge commit atelier merge creates would otherwise blame every cleanly merged line on the base and be counted as unreviewed
- 2026-11-07 [decision] status shows readiness for every unmerged owner, and land on the owner's branch names atelier merge once the approval is current
  Reason: With no done claim before merge, readiness and the merge hint keyed on done would never appear for new work

## Close Reason

Work now has one order: start, implement, tick every Done when line and write the Report, review request, fix and re-review, approve, merge. `atelier merge` always creates a merge commit, and that commit marks the owner's open work-workflow items done. The work workflow has no close. `review request` refuses while an open item it covers has an unticked box or an empty Report. All proof below was run after the last code change, on commit 84df9abb and the record-only commits after it (cargo nextest, crates/atelier-cli/tests unless named):

- review request: review_request::the_request_refuses_unticked_done_when_lines_and_empty_reports_and_lists_them. It pins the full refusal for a parent and a child, the `atelier migrate after-merge` note for an After merge line, and that a superseded child and an item closed by an older Atelier are left out.
- merge marks done: merge_gates::approval_records_committed_by_author_remain_independent_and_merge_marks_the_issue_done covers a standalone issue: the merge commit's parents, the status-only diff from the reviewed head, the Report unchanged, and `land` passing on the base. merge_gates::a_parent_and_its_open_children_merge_done_together_and_open_blockers_refuse covers a parent with an open child and a superseded child, with `land` passing on the base, and shows that an open blocker of the owner, a transitive blocker, or a child's blocker refuses the merge. merge_gates::an_item_closed_by_an_older_atelier_merges_as_it_is covers work an older Atelier closed.
- strategies: atelier-workflow every_merge_strategy_is_read_as_a_merge_commit; merge_gates a_fast_forward_only_policy_merges_with_a_merge_commit_onto_a_moved_base, merge_commit_strategy_merges_without_merge_checks and merge_commit_strategy_merges_with_only_record_checks. The starter policy is covered by cli_integration setup_guidance::test_init_policy_names_no_merge_strategy_and_no_work_close and atelier-workflow parses_valid_policy. Atelier's own .atelier/workflow.yaml keeps `merge_strategy: merge_commit` and has no work close.
- no close, Report only: atelier-workflow retired_report_parts_are_read_as_close_reason; session an_older_policys_report_parts_require_only_a_report; cli_integration reports.rs (supersede requires and writes the Report, a legacy `## Close Reason` is read and rewritten as `## Report`, and `--reason` replaces the Report); atelier-records a_close_reason_heading_is_read_as_the_report_and_written_as_report.
- Done when edits: merge_gates::a_done_when_edited_after_approval_is_shown_but_ticking_it_is_not covers the brief and `merge --check`, after approval, and ticking or unticking alone. merge_gates::a_done_when_edit_merged_in_from_the_base_is_shown covers finding atelier-l5fb. a_done_when_edited_after_the_start_is_shown_without_changing_the_verdict is still green. Both new tests fail against the old record_history.rs: I swapped the old file in, ran them, then restored it.
- Suites: cargo nextest run -j 4 --workspace: 990 passed; with --features test-hooks: 996 passed. cargo fmt -- --check and git diff --check are clean, and target/debug/atelier check passes.

Removed: the `close` transition for work items (starter and atelier's policy); the policy's report `required_fields` (an older policy's report.* is read as close_reason) and the report-part gates with fields_required_when_claimed; `fast_forward_only` and `squash` as strategies (read as merge_commit) with the MergeStrategy enum and merge_diverged; `merge_unproven`; the rule that a parent closes after its children (the work close's work.terminal, merge's work.terminal gate, the epic migration's work.terminal step); and the close-order rules and messages: the "Done claim; commit it on the work branch before review" descriptions, the close line in root help, `--reason` keeping report parts, close fixes for work in gate lines, and the docs. Also gone: CloseReport::with_outcome, ReportPart::field/from_field, and prune's squash-merge detection.

### Deviations
- The changes go beyond the item's named files. prune no longer detects squash merges, since squash is read as merge_commit. The epic migration no longer adds work.terminal. status shows branch readiness for open owners. land on the owner's branch says `Next: atelier merge` once the approval is current. land on the base judges a merge by the paths that differ from Git's merge, and no longer counts clean or approved-marks merges as record-only commits outside review. The `evidence record` fail verdict, the needs-human child note and the behind-base note were reworded.
- The review request brief still leaves out the Report, as it leaves out notes; a reviewer reads it in the record and the diff.
- About 190 tests used the work close. Most now drop it and write a Report. Tests that existed only for removed behaviour were deleted or inverted: the report-part tests in reports.rs and session.rs, the fast-forward and merge_unproven tests, the work.terminal expectations, and squash prune. Others moved to an older policy fixture with a work close, to keep old policies covered.

### Decisions
Seven decision notes on this item: the Report as a renamed section; only boxed lines count; the open-work set is owned by merge_done_status and open_work; merge marks through a scratch index and gates on the merge commit's records; how a Done when change counts; how the base judges a merge's introduced paths; readiness and the merge hint for open owners.

### Surprises
- With the merge work.terminal gate gone, a child added to the scope after approval is marked done by merge, and nothing holds it. merge_gates a_follow_up_or_a_new_child_after_approval_keeps_it_current asserts this.
- Merge checks now always run in a checkout of the merge commit, never in the work checkout. Two tests about checks writing to the work checkout now assert that the write never lands, where they used to expect a refusal.
- claims.reviewed first failed alongside every stale review.current, because the merge commit itself made the claims. It now reads the branch's commits up to its head.
- An older policy's work close now applies its required fields to the marks, so in astromech an open item without a Report makes merge refuse with required_field.close_reason. review request refuses earlier anyway.
- A base Done when edit that a merge resolution later dropped is still reported. A policy that removes `done` from the work workflow's done_statuses would make done records invalid, since no transition reaches `done` now.

### Open items
- Independent review is outstanding; the orchestrator arranges it.
- After merge, reinstall the binary before using atelier here: older binaries refuse records with `## Report` and cannot merge under atelier's new policy.
- atelier-ml3r updates astromech's, demiurge's, cosplay-studio's and life-on-rails' policies (work close, fast_forward_only) and AGENTS.md. The Agent Factory skill still says to close before review.
- atelier-8bh4 adds `atelier migrate after-merge`, which the refusal already names; the after-merge machinery is unchanged.
- Other repositories will notice these changes:
  - merge always creates a merge commit, even with fast_forward_only, and marks open work done in it;
  - a moved base no longer refuses with merge_diverged;
  - no shell check is required to merge;
  - review request refuses on unticked boxes or an empty Report;
  - records the new binary writes say `## Report`;
  - --reason replaces the whole Report;
  - Done when tick-only changes are no longer reported, and base-merged edits are;
  - status rows show readiness for open owners;
  - a squash-merged branch is no longer prunable.
