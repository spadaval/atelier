---
created_at: "2026-11-06T21:26:18.664159142+00:00"
id: "atelier-n0qh"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-n0qh
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-bqbz"
    type: "related"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
  - kind: "issue"
    id: "atelier-r9hb"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rkxm"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T02:15:31.917197054+00:00"
status: "done"
title: "Proof has a time: before merge by default, after merge only with a reason"
updated_at: "2026-11-07T02:15:31.917197054+00:00"
---

## Goal

Each Done when check is checked before merge unless marked for after merge, with a reason (needs the deployed or merged system; a device or place no agent can reach; the human's judgment; time or real use) and a nearest stand-in run before merge. Merged work with open after-merge checks shows as awaiting validation; validation is recorded on the base without a reviewer identity; a failure files a follow-up bug. Nothing here refuses: it is shown where work is judged and the reviewer judges it.

## Done when

the proof-timing scenario test passes; the review request brief lists each after-merge check with its reason and stand-in and says when a reason is missing, and Agent Factory's review text tells the reviewer to flag any that could have run before merge; moving a check to after merge once the item has started is shown to the reviewer through atelier-r9hb; closing a mission with open validation warns and names it unless the validation is carried to another item; agent-facing text in Atelier and Agent Factory says post-merge validation is only what cannot be checked before the merge, and that a failed validation files a follow-up bug (`issue create --discovered-from`) rather than reopening the item; no new refusal and no per-check evidence gate; an ADR records the model.

## Unknowns

Whether after-merge state is a status, a field, or a marker on the Done when check; decided from atelier-rkxm's counts. Recording validation on the base must not trip land's refusal of done claims made on the base (land_base_done_claim): preferably the item is done at merge and only its awaiting-validation marker clears on the base.

## Notes

- 2026-11-06 [decision] An after-merge check is a marked Done when line, '- [ ] After merge (<reason>): <check>. Stand-in: <...>', whose state is the line's own: [x] validated, 'carried to <id>' moved; no status, no field, no record format change (ADR 0027)
  Reason: atelier-rkxm's counts show progress statuses (retired validation, review, blocked, in_progress) never rest on a base and are read by no code by name, and a status would change every policy and need a base transition that land_base_done_claim refuses; a marker keeps the item done at merge, is read unchanged by every installed atelier, and needs no migration
  Rejected: A post-done status such as validating: new workflow state in every policy, merged item not done, base transition trips land
  Rejected: A front-matter field listing checks: copies Done when text and changes the record format, needing old-format reads and a migrate step
- 2026-11-06 [decision] Ticking an open after-merge check or adding 'carried to <id>' is not a Done when edit for atelier-r9hb's notes, and the review brief now also shows those notes
  Reason: Validation on the base is made after the start and would otherwise show as a changed criterion in every debrief; the Done when asks that a check moved to after merge be shown to the reviewer, and the brief is what the reviewer reads
  Rejected: Leave r9hb as is: every validation would read as a loosened criterion

## Close Reason

A Done when line marked `- [ ] After merge (<reason>): <check>. Stand-in: <...>` is now checked after the merge, and nothing about it refuses. The review brief lists each such check with its item, reason and stand-in, says `reason missing` / `stand-in missing`, and asks the reviewer to record a finding for any that could run before merge. It also shows atelier-r9hb's Done-when-edited notes, so a check moved to after merge once the item has started reaches the reviewer. `status`, `work all`, `work mission` and `work parent` list done work with an open check under Awaiting Validation, and the debrief says `Awaiting validation after merge: <check>` under the item. Closing a mission with one open prints `Warning: <mission> closes with validation still open: <id> (N open after-merge check)` and closes anyway. Ticking the box (`[x]`) on the base, or writing `carried to <id>` on the line, clears it. `land` accepts that because the status does not change. ADR 0027 records the model.

Targeted tests, all passing:
- `proof_timing::a_check_after_merge_is_briefed_shown_until_validated_and_refuses_nothing` is the proof-timing scenario: the brief lists the check with reason and stand-in, then `reason missing`; the merged item shows in status, `work all` and `work mission`; mission close warns and names it, and does not warn once the check is carried to a `--discovered-from` bug; validation ticked in an unpushed base commit passes `land`, clears the views, and adds no Done-when-edit or awaiting line to the debrief. Receipt atelier-at5c records it for criterion proof-timing on atelier-n4mj.
- `proof_timing::a_check_moved_to_after_merge_once_started_is_shown_to_the_reviewer`: the brief and `merge --check` name the commit that moved the check after the start.
- `after_merge::tests` (5 tests): parsing, missing reason and stand-in, ticked and carried states, prose that is not a marker, and `only_validated`.
- `review_request::tests::the_brief_renders_as_dense_markdown`, which covers the After merge and edit sections.

Mutation checks: without the `only_validated` exemption, or with `awaiting_validation` always false, the scenario fails. `cargo nextest run -p atelier-app -p atelier-cli` passed (879). `cargo fmt -- --check` and `git diff --check` are clean.

### Deviations
- The brief also shows atelier-r9hb's Done-when-edited notes. Before this, only `merge --check` and the debrief showed them, and the Done when asks that the reviewer see a moved check.
- The debrief also marks awaiting validation (the strategy's "Show, don't enforce" names the debrief). `work parent` shows the panel too, because it shares the scoped dashboard with `work mission`.
- Agent-facing text changed in `issue create --body` help, `review request` help, and the needs-human-child advice from `start` and `status`, which now offers the marker before a standalone validation. Docs changed: validation.md (new "Before and after merge"), work-model.md, reviews-and-merge.md, mission-reviews.md, mission-authoring.md, human-cli-output.md and CONTEXT.md.
- Agent Factory: tracker-records.md (new After-Merge Checks section, plus the Issue template and Validation Item), atelier.md, and the review, validate, plan, implement and orchestrate procedures. These are committed on branch feature/atelier-n0qh in ~/projects/agents-skills-wt/atelier-n0qh (d0e645f) and not pushed.

### Decisions
See the two decision notes on this item: a marked Done when line rather than a status or a field, decided from atelier-rkxm's counts; and validation edits are exempt from r9hb's notes, which now also appear in the brief.

### Surprises
No record format change was needed, so there is no migrate step and nothing to expand and contract. An older installed `atelier` reads the marker as plain Done when text.

### Open items
- Nothing checks that a reason is one of the four or that the stand-in ran. The brief shows what is given and the reviewer judges, as the strategy intends.
- Awaiting Validation reads the checkout's records, so on a work branch a done owner with an open check is listed before it merges.
- `status -q` and `work ... -q` summary counts do not include awaiting validation. `work all -q` lists the ids.
- No Agent Factory skill eval was run for the new text. It describes a new flow and does not correct an observed failure.
- `cargo clippy` fails on existing code in atelier-records (lib.rs:1637, collapsible if), which this change does not touch. CI does not run clippy.
- Independent review is outstanding, for both the atelier branch and the agents-skills branch.
