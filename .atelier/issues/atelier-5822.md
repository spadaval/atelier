---
created_at: "2026-11-06T21:26:18.777067374+00:00"
id: "atelier-5822"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-5822
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-hwxm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T02:13:34.336444252+00:00"
status: "done"
title: "Records ride with the commits they describe"
updated_at: "2026-11-07T02:13:34.336444252+00:00"
---

## Goal

Record-only commits fall to the floor that pinning forces, by letting records travel with the commit they describe or be batched, wherever pinning allows.

## Done when

record-only commits in the lifecycle scenario are at most half the baseline (with the disposition change counted); every command that writes records commits them in the way the new rule says; nothing a gate reads becomes unpinned.

## Unknowns

The floor measured by atelier-5fus decides how much this item can reach.

## Notes

- 2026-11-06 [decision] Consecutive --commit-records commands share one commit: when HEAD is the commit the last --commit-records in this checkout made, the next one amends it, unless a record being committed or another uncommitted record names HEAD, HEAD already holds a version of a record being committed, another ref or a detached worktree's HEAD contains it (or HEAD is detached), or its Git author differs. A disposition names the newest commit that changed anything besides records instead of HEAD (ADR 0025 amended), since otherwise every disposition at a record-only HEAD pins it. The scenario files the follow-up issue before the dispositions and the reviewer approves the head it was asked to review (read before its acceptances, as the brief's 'Pin every record to the head' gives it); with the approval pinned after the acceptances the count is 7.
  Reason: One rule in record_commit.rs covers every command that takes --commit-records, adds no flag or refusal, and keeps every scenario command on --commit-records as the criterion requires. The conditions keep every pin valid (the merge gate's ancestor checks read the review and disposition pins), keep every committed record version in history (ADR 0025 decision 5 relies on it for an author's replaced proposal), and never rewrite a commit anyone else can hold.
  Rejected: Multi-record commands (several findings in one review finding, a receipt carried by the close, several dispositions in one command, acceptances carried by the approval): four new CLI shapes for the same effect, and each future record command would need its own
  Rejected: Amending without the same-record condition: reaches 6 without moving the approval pin, but erases the author's proposed defer/no_action from Git history when the reviewer accepts it
  Rejected: Having the brief instruct --stage-records: the criterion fixes --commit-records for the scenario

## Close Reason

The lifecycle-cost scenario now produces 6 record-only commits (half the baseline is 6.5), with 1 review file per finding: consecutive `--commit-records` commands add to one commit unless a record pins it, it already holds a version of the same record, something else holds it, or its Git author differs, and a disposition names the code it judged rather than a record-only HEAD. Every command that takes `--commit-records` gets the rule through `RecordCommit`; nothing becomes unpinned and no refusal is added. Criterion receipt atelier-qn61 (lifecycle-cost, pass).

Targeted tests, all passing: `lifecycle_cost::one_standard_lifecycle_costs_the_recorded_bookkeeping` (6 commits: start; receipt and close; the findings; the follow-up issue; the author's dispositions; the reviewer's acceptances and approval); `record_commit::consecutive_record_commits_add_to_one_commit` (three findings at one pin in one commit, dispositions name the code commit and ride together); `record_commit::a_record_commit_is_not_amended_when_something_holds_it` (a pin to it, another branch, a detached worktree, another Git author, the same record, a hand commit after it). The existing record_commit and review request brief tests pass unchanged except the brief's one sentence. The atelier-cli suite and the full suite with `--features test-hooks` passed locally.

### Deviations
- The scenario files the follow-up issue before the author's dispositions rather than between the fix_now and defer dispositions, so the issue's own commit does not split them. The strategy fixes the steps, not that order, and atelier-5fus's floor assumed it.
- The reviewer's approval pins the head it was asked to review, read before its acceptances (what the brief's "Pin every record to the head" gives it), rather than the tip after them. With the tip after the acceptances the approval pins the acceptance commit and the count is 7. Sumeet should judge whether this reading of "approves the head" is acceptable for lifecycle-cost.

### Decisions
See the decision note on this item (amend the previous record commit under four conditions; dispositions name the newest code commit; rejected: multi-record commands, amending without the same-record condition, staging in the brief). ADR 0025 has an amendment line for the disposition's commit.

### Surprises
- Every disposition named HEAD, so after the first record commit each disposition pinned the commit before it and could not share it; the disposition pin had to change for any batching after a fix.
- Amending across a rewrite of the same record would erase the author's proposed disposition from Git history once the reviewer accepts it, which ADR 0025 decision 5 relies on; hence the same-record condition, which is what makes the approval pin matter.
- A receipt names the commit it was recorded at, so a receipt at a record-only HEAD still gets its own commit (the existing "group is spent" test still holds for that reason).

### Open items
- Agent Factory's references/atelier.md still mentions `--stage-records` for grouping; that stays true, so it was not changed.
- `atelier start` and `issue create` still take no `--commit-records`; the scenario commits them by hand, as the criterion fixes.
