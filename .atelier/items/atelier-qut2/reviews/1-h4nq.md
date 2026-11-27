---
schema: atelier.review
schema_version: 3
round: 1
commit: d9cd45b95bfe824e9083bc285b273c2e11a57f65
reviewers:
- thread:delegated-task:command%3Amcp%3A661c03ab-d48a-4f2a-b71e-0831e8c98cf3%3Adelegate-task%3Ad134f97b-0c1d-4f7a-8aed-c4662862ba3c
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-27T08:49:44.276074176Z
---

## Summary

Complexity: low. The change adjusts existing instructions and briefs, with
one read of the mission record and no new state or command.

Value: high. These are the planning and review instructions the mission
exists to correct.

Risk of merging: medium. The instructions affect what agents build and
review across repositories, though the text and brief changes are reversible.

Read the full diff against master, the mission and its notes, the item and
atelier-b0vb, the affected roles, and the surrounding review and merge code.
The plan counts match notes.md. An independent recount of hub-v2-bg14 at
49f96611 confirms the Report's review and finding counts and minute-scale
timings. The two skill files total 1,036 words both before and after.
The changes otherwise fit the Outcome and their cost is proportionate.

`cargo nextest run -p atelier-cli` passed all 280 tests (one skipped).
`cargo fmt -- --check` and `git diff --check master...d9cd45b` passed.
`evals/agent-factory/test.sh` passed all eval fixtures and harness tests.
No model eval matrix or plan re-review was run, so actual changes to agents'
review behavior remain unmeasured, as the Report acknowledges.

## Finding 1: Important: Preserve the Outcome comparison for Done when lines changed in later rounds
Recommendation: fix now

`crates/atelier-cli/src/output.rs:1307-1308` gates both the mission Outcome
and the new `Shrink:` instruction on a whole-change review. A normal round
2 therefore receives neither, even when its delta adds or rewrites Done
when lines. Those lines are explicitly still in that round's remit; round
1 cannot have judged a requirement that did not exist then. The later
reviewer is again left checking the item's checklist without the mission
context or the instruction to challenge excess in it.

Reproduced with the built binary and the existing later-round-wrong fixture
in a temporary repository. Put a mission and its item association on main
before round 1, then add a Done when requirement in the round-2 delta.
`atelier review hours-q7tn` prints round 2 and the added requirement, but
neither the mission's Outcome nor the `Shrink:` instruction. The new test
in planning.rs only checks the round-1 brief, so it misses this case.

Keep the Outcome and the instruction available when a later round judges
changed Done when lines. The instruction can expressly apply only to those
changed lines in later rounds, leaving the earlier judgments intact. Cover
that case with a mission-backed later-round brief test.
