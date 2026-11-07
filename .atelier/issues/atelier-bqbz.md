---
created_at: "2026-11-06T21:26:18.892841983+00:00"
id: "atelier-bqbz"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-bqbz
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n0qh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
  - kind: "issue"
    id: "atelier-rkxm"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T03:08:21.939534763+00:00"
status: "done"
title: "Keep only the statuses and issue types repositories use"
updated_at: "2026-11-07T03:08:21.939534763+00:00"
---

## Goal

Each workflow status and issue type is kept only if atelier-rkxm's counts show repositories use it, or a gate needs it; the after-merge state from atelier-n0qh replaces what it makes redundant.

## Done when

the starter workflow policy drops what is unused, with each drop justified from atelier-rkxm's note; an atelier migrate step retypes existing records; existing repository policies keep working; the choice is recorded under this item.

## Unknowns

Whether any repository's own policy depends on a status the starter drops.

## Notes

- 2026-11-06 [decision] The starter policy and atelier's own policy drop the blocked and review statuses with their block, request_review and revise transitions; draft, backlog, todo, in_progress, done and superseded stay
  Reason: Counts (atelier-rkxm): blocked has 0 records on any ref in astromech, demiurge and cosplay-studio, 13 in atelier history and none at rest on any base tip; review has 0 in demiurge, 1 branch record in cosplay-studio, and rested on a base tip once each in atelier and astromech. Nothing reads either by name. Each makes agents choose and run a transition that buys nothing: a blocker is already a blocked_by link, which work blocked, the ready classification and start's blockers.transitive_none_open read, and close's blockers.none_open refuses while it is open; review is recorded in review records after the done claim, which Agent Factory makes before requesting review, so in_progress -> done is the path in use. The other six are each used in at least three repositories and read: draft by the mission ready transition and debrief, backlog by parking, superseded by name in 17 places, todo/in_progress/done as the main path. No live ref in atelier, astromech, demiurge, cosplay-studio or life-on-rails has a record in blocked or review (only pre-rebuild archive branches in atelier), so atelier's policy can drop them without a record change, and the installed binary reads the smaller policy (no reinstall needed).
  Rejected: Keep blocked for work that finds a blocker mid-flight: a blocked_by link already stops close and shows the blocker count; the item stays shown as active, which is what it is
  Rejected: Keep review as a visible 'awaiting review' state: review request and review records already show it, and agents close before review
- 2026-11-06 [decision] Every issue type stays: bug, decision, feature, mission, spike, task and validation
  Reason: Every type has records in every one of the four repositories (atelier-rkxm), so dropping any one retypes hundreds of records here (139 bug, 112 feature at rest in atelier) and leaves other repositories' policies on the old set. bug/feature/spike/validation differ only in label, but the choice costs agents nothing: issue create defaults to task, and the type names the work branch (<type>/<id>) and labels views, which humans read. validation stays for standalone validation items, which ADR 0027 keeps for mission acceptance and needs-human checks.
  Rejected: Fold bug, feature and spike into task (or into labels) with an atelier migrate step: it would retype most of atelier's records and change branch names, for a choice agents already skip by default
- 2026-11-06 [decision] No atelier migrate step ships with this change
  Reason: No record at rest on any live ref in any repository on this host uses blocked or review, and no type is dropped, so a migrate step would have nothing to retype. Repository policies are left as they are and keep working, since the binary reads statuses by category; a repository that wants the smaller set copies the starter's statuses and transitions.
  Rejected: A migrate command that rewrites a repository's policy to the new starter and moves blocked/review records to todo/in_progress: an extra command for a move no repository needs today

## Close Reason

The starter policy (`atelier init`) and atelier's own `.atelier/workflow.yaml` drop the `blocked` and `review` statuses and their `block`, `request_review` and `revise` transitions; `start`, `defer` and `decide` leave from `todo` only and `close` from `in_progress` only. Every issue type stays. Policies written from the earlier starter load and run unchanged; the three decision notes on this item give each drop and keep with its counts from atelier-rkxm and the code that reads it.

Proof, run after the last change:
- `starter_policy_declares_the_statuses_and_types_in_use` (atelier-workflow): the starter declares backlog, done, draft, in_progress, superseded, todo, all seven types, and no block/request_review/revise transition. Pass.
- `the_earlier_starter_with_blocked_and_review_still_parses` (atelier-workflow): the fixture `crates/atelier-workflow/fixtures/starter-policy-with-blocked-and-review.yaml` (the starter as it was) parses with `blocked`, `review` and their transitions. Pass.
- `test_policy_from_the_earlier_starter_keeps_blocked_and_review` (cli_integration): under that policy, start, block, `work blocked`, start, request_review, `work active`, revise, request_review, close and `check` all work. Pass; it fails at `block` when pointed at the new starter.
- Existing tests about `blocked`/`review` behaviour now run on that fixture and pass: `land_fixes_for_open_children_and_a_blocks_link_clear_their_failures` and `ready_work_in_flight_on_another_branch_is_marked_with_its_state_there` (session), `test_quiet_work_all_includes_every_open_bucket_in_panel_order`, `test_work_classification_agrees_across_global_mission_and_epic_views`, `test_status_footer_uses_complete_work_buckets`, `test_migrate_epics_retypes_epic_records_and_the_policy`, `notes_status_and_new_evidence_records_do_not_stale_passing_proof`, and the mission-ordering and inventory-rendering unit tests.
- `./target/debug/atelier check` and the installed `atelier 0.3.1+a6450640` `check` and `status` pass on this branch with the new policy.

### Deviations
No `atelier migrate` step, though Done when names one: nothing needs retyping. No record at rest on any live ref in atelier, astromech, demiurge, cosplay-studio or life-on-rails is in `blocked` or `review` (only atelier's pre-rebuild `origin/archive/*` and `origin/mission/atelier-rebuild-*` refs hold one, atelier-xz11, in the old schema), and no type is dropped. Other repositories' policies are untouched and keep both statuses.

### Decisions
See the three decision notes on this item (dated 2026-11-06): which statuses go and which stay, why every type stays, and why there is no migrate step.

### Surprises
46 tests leaned on the starter's `blocked`/`review` statuses or `block`/`request_review` transitions. Tests that only reached a state through them now use `start` alone (`move_issue_to_review` became `start_issue_work`); tests of how `blocked` and `review` behave run on the fixture of the earlier starter, so they keep proving existing policies work. `issue list --category blocked` is now a usage error under the starter, since no status is in that category; `work blocked` is unaffected because it reads blockers, not the category.

### Open items
Work that finds a blocker after it starts now adds a `blocked_by` link and stays `in_progress`, shown as active with its blocker count rather than as blocked; `close` still refuses while the blocker is open. Changing atelier's own policy needs no reinstall: the installed CLI reads the smaller policy (checked with `atelier check` and `atelier status`). Review is outstanding.
