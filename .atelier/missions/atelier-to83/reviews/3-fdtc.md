---
schema: atelier.review
schema_version: 3
round: 3
commit: ddb78efe1c459bc6096a449919c43b2a4983ae40
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3A5825c4d7-9dfa-45e8-b062-0de5fae68e2c
model: gpt-6-astra
verdict: approve
created_at: 2026-11-13T06:35:11.736787316Z
---

## Summary

Complexity: high. This changes record placement, cross-clone claims, plan approval, forge merges and the Desk across five repositories.

Value: high. Protected-base support and a view of published work require these changes, and the plan removes the repeated human go-ahead while retaining independent review.

Risk of merging it: high. An incorrect precedence rule or merge gate could silently accept stale work or ignore cancellation across clones.

The value warrants the scope. The documentation-first item, separate implementation items, old-format compatibility and staged rollout make the work reviewable; I found no smaller change that delivers the stated outcome.

I reviewed the mission and its design and all eight item records at ddb78efe1c459bc6096a449919c43b2a4983ae40. I compared the seven carried findings with the revised acceptance criteria and checked the current record precedence and plan comparison in docs/design.md and crates/atelier-app/src/plan.rs. I also inspected the current merge implementation to distinguish existing behavior from the proposed forge path. All seven carried findings are addressed at the plan level. There are no new fix-now findings.

This is a plan review, not validation of an implementation. I did not run the future two-clone, protected-remote, Desk or five-repository rollout tests. The precise precedence rules and forge completion mechanics remain work for atelier-ogpg, followed by the implementation tests explicitly required by the dependent items.

## Finding 1: Items on a proposed plan sit outside the two-review gate
Recommendation: fix now
Was: 2-knm6.10
Now: fixed

Fixed by atelier-2vdq: the proposed mission includes its items, changes/additions/removals/reassignments stale approvals, and an initial item acceptance-criteria edit must fail merge. The mission Outcome and design agree.

## Finding 2: Result cannot be told from the gated sections of mission.md
Recommendation: fix now
Was: 2-knm6.11
Now: fixed

Fixed by the explicit section-level comparison in atelier-2vdq and design.md. The item now requires both a Result-only merge without plan review and refusal of a Why edit on an open mission.

## Finding 3: Plan-review refs are unnamed and the reader never looks at them
Recommendation: fix now
Was: 2-knm6.12
Now: fixed

Fixed by the review/<mission-id>/<round>-<suffix> convention. atelier-2vdq specifies publication on the reviewer branch and collection by submit/merge; atelier-dciw requires discovery and presentation of those verdicts before collection.

## Finding 4: Stop is not an act that publishes, and the Desk is not required to finish it
Recommendation: fix now
Was: 2-knm6.13
Now: fixed

Fixed by atelier-2vdq specifying drop authority for a planner and the configured Desk caller, the proposed/open paths, publication and status 3 while waiting. atelier-v958 requires the Desk to drive the stop through completion and distinguish pending from stopped, with interface coverage of both paths.

## Finding 5: The losing start claimant keeps a local branch that still looks started
Recommendation: fix now
Was: 2-knm6.14
Now: fixed

Fixed by atelier-ob9u requiring removal of the losing local branch and restoration of the checkout, subsequent take-over from the remote branch, and a two-clone test that checks the losing clone lists the winner after fetching. The design states the same cleanup contract; the reader precedence rule is explicitly assigned to atelier-ogpg.

## Finding 6: Dropping a started item is no longer effective until some other branch merges
Recommendation: fix now
Was: 2-knm6.15
Now: fixed

Fixed at the plan level by moving a started item drop onto its own branch and publishing it there, with submit/merge refused thereafter. atelier-ob9u requires both cross-clone orderings and refusal of a drop after merge; the base copy follows the record-only path. This replaces the separate unmerged cancellation branch described by the earlier finding.

## Finding 7: Keep the proposed mission's items inside approval invalidation
Recommendation: fix now
Was: 2-noyj.10
Now: fixed

Fixed by the same proposed-item comparison and stale-approval test that resolve Finding 1. Adding an item without plan review remains limited to an already-open mission.
