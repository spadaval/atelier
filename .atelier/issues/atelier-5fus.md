---
created_at: "2026-11-06T21:26:18.219762489+00:00"
id: "atelier-5fus"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: task/atelier-5fus
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-06T23:49:16.260175134+00:00"
status: "done"
title: "Measure the bookkeeping of one standard lifecycle"
updated_at: "2026-11-06T23:49:16.260175134+00:00"
---

## Goal

Give the mission its baseline and its floor: how many record-only commits and review files one standard lifecycle produces with the product as it is at the mission start, and how many of those commits are forced rather than chosen.

## Done when

- A scenario test under crates/atelier-cli/tests/ drives the lifecycle-cost scenario exactly as the strategy defines it: a fixture repository with the `atelier init` starter policy; commands that accept `--commit-records` use it, and `atelier start` and `issue create` are each followed by one commit of exactly the records they wrote; start an issue, record one manual evidence receipt (`--result pass`), close it; one review (by an identity other than the author) that raises three findings, one of them `fix_now`, and does not approve; a fix commit; dispositions recorded by the author: `fix_now` with that resolution commit, `defer` with a follow-up issue, `no_action` with a reason (the last two accepted by the reviewer identity, as today's rules require); the reviewer reviews the fix and approves the head; merge.
- The test asserts two counts against today's numbers: record-only commits (commits on the work branch that change only files under `.atelier/`) and, for each finding, review files about it (the finding plus every review event that names it). Later items change these expected numbers, which shows each reduction.
- An evidence receipt on this issue, recorded at the branch head before any product change (product code equal to master at the commit the branch left from), records both counts as the baseline.
- A note on this issue classifies each record-only commit in the scenario as forced by pinning (the record names a commit it cannot be part of), forced by gate ordering (for example, the done claim before review, approval before merge), or chosen by how commands commit records today. The note says whether "at most half the baseline" is reachable given the forced commits; if it is not, it proposes a rewritten lifecycle-cost criterion for Sumeet.

## Scope

Test-only: a new test file, reusing the fixture helpers in crates/atelier-cli/tests/merge_gates.rs (moved to a shared test module if needed). No product code, policy or doc changes.

## Context

Strategy: docs/strategies/lighter-workflow.md (criterion lifecycle-cost). The review request brief tells reviewers to use `--commit-records` (crates/atelier-cli/src/commands/review_request.rs); `--stage-records` batching exists (docs/product/cli-surface.md) but the scenario must not switch to it, so reductions come from product changes.

## Notes

- 2026-11-06 [result] Classification of the 13 record-only commits in the lifecycle-cost scenario (baseline receipt atelier-cqv2). "Forced" means no product change can remove the commit; "chosen" means records could share a commit with a neighbour as far as pinning and gate ordering go, and today they do not only because each --commit-records command makes its own commit (or because the scenario fixes the commit).

  Before review:
  1. Start: chosen by the scenario. Start pins nothing and could ride with the implementation commit (start itself says so); the strategy fixes one commit after start, so no product change removes it under this scenario.
  2. Receipt: forced by pinning. It names the implementation commit, so it cannot be part of it, and the reviewer should see the proof, so it lands before review.
  3. Close: chosen. The done claim names no commit; gate ordering only needs it committed before review, so it could ride with the implementation commit or with the receipt.

  Review round:
  4. First finding: forced by pinning and gate ordering. It names the reviewed head, so it comes after it, and it reaches the author before the fix it causes (with the reviewer in its own checkout, through a commit).
  5-6. Second and third findings: chosen. They pin the same head and could share commit 4.

  After the fix:
  7. fix_now disposition: forced by pinning and gate ordering. It names the fix commit, so it comes after it; it is the author's record, so (with the reviewer in its own checkout) it reaches the reviewer through an author commit before the reviewer accepts and approves; and it cannot ride with the follow-up issue's commit, which the scenario fixes as exactly that issue's records.
  8. Follow-up issue: chosen by the scenario (issue create plus one commit); it could ride with the defer disposition that names it.
  9-10. Author defer and no_action dispositions: chosen; they could share commit 7.
  11-12. Reviewer acceptances: chosen; new review events do not stale an approval, so they could ride with the approval commit.
  13. Approval: forced by pinning (it names the head) and gate ordering (approval before merge).

  Forced: 4 (2, 4, 7, 13). Fixed by the scenario: 2 (1, 8). Chosen: 7.

  Reachability: at most half the baseline is at most 6 commits (half of 13 is 6.5). The floor under this scenario is 6 (4 forced plus 2 fixed by the scenario), so the target is reachable with no commit to spare. Getting there takes all three batchings: the receipt and close in one commit (2-3), the review round in one (4-6), and the post-fix records in one commit per actor, the author's dispositions (7, 9-10) and the reviewer's acceptances with the approval (11-13). That gives start, receipt and close, findings, follow-up issue, author dispositions, reviewer acceptances and approval: 6. Missing any one of the three batchings gives 7 or more. atelier-hwxm alone cuts review files to 1 but not commits, since its disposition and acceptance updates are still separate commands that each commit. The cut rests on atelier-5822 committing the records of one actor's sitting together while each command still uses --commit-records as the brief says. If 5822 cannot batch some of these, the target is not reachable (with no batching after the fix the best case is 1 + 1 + 1 + 7 = 10); Sumeet would then need to restate lifecycle-cost, for example as "record-only commits are at most the scenario's floor of 6, measured by atelier-5fus, plus one (7)", or as a count named per segment. No rewrite is proposed now, since the target is reachable.

  Correction (2026-11-06, finding atelier-idld): an earlier version of this note classified 7 as chosen and gave a floor of 5 with one commit to spare. That applied the reviewer-to-author hand-off to finding 4 but not the author-to-reviewer hand-off after the fix.

  Outside the scenario conventions the strict floor is 1 (the approval): the start, receipt, close, findings and author dispositions could all ride with code commits when author and reviewer share a checkout.

## Close Reason

The lifecycle-cost scenario now runs as a test (crates/atelier-cli/tests/lifecycle_cost.rs) and asserts today's baseline: 13 record-only commits on the work branch, and 2, 3 and 3 review files about the fix_now, defer and no_action findings. The baseline is receipt atelier-cqv2, and the classification and reachability of "at most half" are in this item's result note: 4 commits are forced, 2 are fixed by the scenario, 7 are chosen, so the target (at most 6) is reachable with no commit to spare, and only if atelier-5822 batches the receipt with the close, the review round, and the post-fix records per actor.

### Deviations
- The scenario has one implementation commit between start and the receipt, which the strategy does not list but its review and fix steps assume.
- The follow-up issue for the defer finding is created on the work branch, followed by one commit of its records, as the strategy says for `issue create`; this puts it among the 13.
- The git and atelier runners (git, cli, ok, commit) moved from merge_gates.rs into tests/support/cli.rs, which both tests include.

### Decisions
None beyond how the scenario reads, listed above. Today's rules decide the rest: the reviewer accepts a defer or no_action by recording the same decision, which supersedes the author's; findings and the approval pin the head with `--commit`, as the review request brief does.

### Surprises
- The starter policy has no shell check, so merge fast-forwards and adds no merge commit.
- atelier-hwxm alone will not reduce commits: its disposition and acceptance updates are still separate commands that each commit.
- `measurement` is not a known evidence kind (`evidence record` warns and accepts it; known kinds include benchmark and validation).

### Open items
- Whether "at most half" holds depends on atelier-5822 achieving all three batchings; if it cannot, the note gives a possible rewrite for Sumeet.
