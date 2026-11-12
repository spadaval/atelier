---
schema: atelier.review
schema_version: 3
round: 3
commit: c434876b7d21d39086d923f44b60e643ea68fc13
reviewers:
- thread:delegated-task:command%3Amcp%3Aa0526aa8-7650-4e5d-80ff-0b0535866187%3Adelegate-task%3Ab1292d9c-17f2-4ea2-a1dd-793b7224912a
model: gpt-6-astra
verdict: approve
created_at: 2026-11-12T01:20:10.620468166Z
---

## Summary

Reviewed the plan at c434876: the mission, all three draft items, the current skill and procedures, the referenced product and Desk documentation, and the amendment since round 2. Cross-checked the first-run evidence and inherited gaps against atelier-613a and atelier-h201, read atelier-0019 with `atelier show`, and counted the current procedure lines. This is a plan review: I did not run the proposed trial, inspect the original transcripts, or replay the backlog sweep. The trial must still demonstrate the promised improvement.

- Complexity: medium. Two bounded procedure rewrites, documentation updates and one trial, with no new CLI or record type.
- Value: high. The first-run report supports the repeated checklist proposals, and the existing backlog procedure sends substantive decisions to a separate digest. The plan directly addresses both limitations.
- Risk of merging it: medium. The procedures guide work in other repositories and permit autonomous drops, but require recorded proof, protect human-directed and owned work, and preserve review and the gardener's session.

The value justifies the scope. The existing line budgets constrain growth, and the trial checks the central claim with actual thread evidence and a structure proposal. I found no remaining fix-now problem. Both carried findings are fixed in the plan.

## Finding 1: Say how a waiting run's page is read
Recommendation: fix now
Was: 2-kw2o.8
Now: fixed

`.atelier/items/atelier-wb05/item.md:32` now requires `atelier show <id>` for candidate garden items, excludes this run, and requires the Report's template `Window:` line. It explicitly excludes the old digest. I verified that `atelier show atelier-0019` supplies the approved run's Report and its end time, 2026-11-11T17:56:43Z, which the trial checks. The procedure writer now has an actionable rule for finding a page still on its branch.

## Finding 2: Say what the thread cap counts
Recommendation: fix now
Was: 2-kw2o.9
Now: fixed

`.atelier/items/atelier-wb05/item.md:33` now caps items, expanding each into two full threads: the implementer and one reviewer. It specifies the latest review round, except when the selected escalation or skip identifies an earlier round. The mission's transcripts-first criterion also counts items and requires one reviewer. This resolves the ambiguity between item and transcript counts without requiring every review round.
