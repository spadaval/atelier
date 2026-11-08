---
created_at: "2026-11-08T05:35:31.390561285+00:00"
id: "atelier-stnv"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: atelier-stnv
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Let review say no: a won't-fix decision, a declined status, and deferring an item's findings in one command"
updated_at: "2026-11-08T06:47:07.498118914+00:00"
---

## Goal

Give findings and work items a recorded "no". Findings today end as fix_now, defer (now or later, with a follow-up issue each) or no_action (the finding is wrong). Add won't-fix for a real finding nobody will fix, a declined status for work that will not be done, and one command that defers all of an item's deferred findings so deferring costs no more than fixing.

## Done when

- [x] `atelier review disposition --decision wont_fix <finding> "<reason>"` records that a real finding will not be fixed. The reason is required and non-empty. It satisfies merge's disposition gate as defer and no_action do, needs no follow-up issue, and like no_action any identity may record it: identities are recorded, not compared with Git authors (atelier-md0v). Who should decide is the skill's rule, not a gate. `review list`, `review show`, the review brief, `debrief` and `doctor` show it as won't fix, distinct from no_action.
- [x] The help for `review finding` and `review disposition` says no_action means the finding is wrong (mistaken, a duplicate, or without consequence) and wont_fix means it is real but not worth fixing.
- [x] Work items have a terminal `declined` status (category done) and a `decline` transition from backlog, todo and in_progress with a required close reason, in the workflow `atelier init` writes and in this repository's `.atelier/workflow.yaml`. The `on_base` workflow (loose ends, validations) gets `decline` from backlog and todo. The gate sites that skip `superseded` today skip `declined` the same way, so a base commit that only declines an item lands: land's base completion claims (`base_done_claims` in `crates/atelier-cli/src/commands/git.rs`) and the completion-claim and parent blocker filters in `crates/atelier-app/src/review_gate.rs`. Reporting does not copy superseded's treatment: work views, `debrief` and `doctor` show a declined item as finished without merging under its own status (not dropped, retired or replaced, as `mission_review.rs` and `decisions.rs` show superseded). A repository whose workflow.yaml lacks the status loads and works unchanged.
- [x] One command defers every open finding on an item that was recommended defer and has no disposition yet: it creates one backlog follow-up per finding (discovered from that finding) under a single parent per item, and records each defer disposition. The parent is created on the first run and found again on later runs by its link to the item, so a second run reuses it and files nothing twice. A dry-run flag lists what it would do and writes nothing. Its records can be committed in one commit, like other commands' `--commit-records`.
- [x] Tests cover: merge passes with a wont_fix disposition; decline without a reason refused; a base commit that only declines a work item, and one that declines a loose end, lands; an older workflow.yaml without declined loads; the defer command's dry run, real run, and a second run that files nothing new.
- [x] The new binary runs `atelier check`, `atelier doctor` and `atelier work missions` in scratch clones of each local Atelier repository (atelier, astromech, life-on-rails, cosplay-studio, demiurge, worldcrafter) with no error the installed binary does not also report. The Report lists the commands and results.

## Scope

Atelier's code, `docs/product` and this repository's workflow.yaml and records. Not other repositories' workflow.yaml (loose end atelier-azg2 adds declined there once the binary is installed). Not the agent-factory skill.

## Context

Agreed with Sumeet in conversation on 2026-11-08 after an audit of review records in atelier, astromech and life-on-rails (about 175 findings sampled). Review found real defects about three times as often as it did harm, and no sampled finding was wrong. The harm came after review: orchestrators fixed findings the reviewer marked defer (atelier: 122 of about 242 deferred findings were fixed anyway, 80 with no reason beyond "Fixed in <sha>"), several "easy" fixes grew to hundreds of lines (astromech-u6ak, astromech-emsi, the astromech clock-skew chain, atelier-hwxm), and nothing in the process lets anyone decide not to do real work. A defer costs a follow-up issue per finding, and backlog is the only place a no can go (atelier has 100 backlog items, astromech 71 open follow-ups from dispositions). The skill changes that use these commands are a sibling item.

## Notes

- 2026-11-08 [decision] A review finding that recommends or decides wont_fix is written at review schema version 4 (WONT_FIX_VERSION), about an issue or a branch; every other record keeps version 2, or 3 about a branch, and this binary reads 1 through 4 and refuses wont_fix below 4.
  Reason: An older Atelier then reports a wont_fix record as needing a newer Atelier, with the reinstall command, instead of as an invalid review it tells you to fix by hand (atelier-xlxn), while every record without wont_fix stays readable by older binaries. This follows how branch events moved to version 3 (atelier-4nsl).
  Rejected: Bump REVIEW_KIND to 3 or 4 for every review: older binaries would stop reading ordinary reviews, which the light path's versioning was built to avoid.
  Rejected: Accept wont_fix at versions below 4 when reading: no Atelier writes that, and it would hide a hand-edited record that older binaries reject as invalid.

## Report

All six Done when lines hold at b12632fd. `review disposition --decision wont_fix` records a real finding nobody will fix, needs a non-empty reason and no follow-up, settles merge, and shows as `wont_fix` in `review list`, `review show`, the brief and the gate lines, and as "won't fix" in the debrief (`merge_passes_with_a_wont_fix_disposition`, which also runs `doctor` after the merge, and `finding_counts_name_wont_fix_apart_from_no_action`: pass). `declined` and `decline` are in the starter and in `.atelier/workflow.yaml`; a base commit that only declines a blocked task, and one that declines a blocked loose end, land, and decline without a reason is refused (`a_base_commit_that_only_declines_work_or_a_loose_end_lands`: pass, and it fails when either the `base_done_claims` skip or the completion-claim skip is reverted). A declined child's blockers do not hold its parent's merge (`a_declined_childs_own_blockers_do_not_hold_its_parents_merge`: pass, fails with the parent filter reverted). The debrief shows a declined item as declined with its Report, counts it done in the plan, asks no review of it, and keeps superseded work under Dropped (`declined_work_is_finished_under_its_own_status_not_dropped`: pass); `work mission` lists it under Done Work as `✓ declined` (seen by hand, not tested). A policy without `declined` loads, offers and accepts no decline, and supersedes and lands as before (`a_policy_without_declined_loads_and_works_unchanged`: pass). `atelier review defer <id>` files a backlog follow-up per undecided defer finding under one `deferred-findings` parent, reuses that parent on a later run and files nothing twice, writes nothing with `--dry-run`, and commits in one commit with `--commit-records` (`review_defer_dry_run_lists_the_plan_and_writes_nothing`, `review_defer_files_one_follow_up_per_finding_and_a_second_run_files_nothing`: pass). The full suite passed locally with and without `--features test-hooks` (1032 and 1038 tests), and `cargo fmt -- --check` is clean.

Fix for finding atelier-xlxn: a finding that recommends or decides `wont_fix` is now written at review schema version 4, about an issue or a branch, and every other record keeps version 2 (3 about a branch) (`a_wont_fix_finding_is_written_at_the_newer_version_and_round_trips`: failed before the fix with the record at version 2, passes after; `a_newer_review_needs_a_newer_atelier` now probes version 5). In a scratch repository, this branch's binary wrote a `wont_fix` recommendation and a `wont_fix` disposition at version 4, and the installed 0.3.1+e03d6b71 `atelier check` reports each as "has schema_version 4, and this atelier reads 3, so records need a newer atelier" with the reinstall command, not as an invalid review; a plain fix_now finding stays at version 2 and the installed `check` passes. The full suite passed (1033 tests) and `cargo fmt -- --check` is clean.

Cross-repository smoke, in `git clone --local --no-hardlinks` clones under the scratchpad (hardlinks fail across filesystems), comparing the installed binary (0.3.1+e03d6b71) with this branch's debug build: `atelier check`, `atelier doctor` and `atelier work missions` exit 0 with byte-identical output in atelier (at master 7b956105, at the original checkout's detached b81f650d, and at this branch b12632fd), astromech (feature/astromech-v6r3), life-on-rails, cosplay-studio, demiurge and worldcrafter. In the astromech clone `doctor` first failed on both binaries alike because the clone had no local `main`; after `git branch main origin/main` both pass.

### Decisions

- `wont_fix` is a fourth `FindingDecision`, so a reviewer can also recommend it; any non-fix_now decision already settled a finding, so the gates needed no change. It is shown as the token `wont_fix` wherever `no_action` is shown as a token, and as "won't fix" in the debrief's prose counts.
- One helper, `atelier_core::closes_undone`, names the statuses the gates skip (superseded and declined): land's base done claims, the completion-claim and parent blocker filters, `land`'s readiness check, the evidence close judgement, After merge lines and the gate report's closing path. Reporting does not use it.
- A `wont_fix` finding is written at review schema version 4, chosen by content, so older binaries keep reading every other record (decision note, atelier-xlxn).
- `review defer` files `task` items in `backlog`; the parent is labelled `deferred-findings` and discovered from the item, and only an open parent is reused, so new follow-ups never land under closed work. A follow-up's priority comes from the finding's leading `critical:`/`high:`/`medium:`/`low:`. It takes the item's findings and its children's, as `review list` does.

### Surprises

- The debrief called declined work "delivered" and listed it as "No review recorded"; both now say what happened.

### Open items

- An older Atelier cannot read a review record with `wont_fix`; since atelier-xlxn it reports it as needing a newer Atelier. Other repositories should still install the new binary before anyone records a `wont_fix` there (the same order atelier-azg2 needs for `declined`).
- Review is outstanding.
