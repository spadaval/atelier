---
schema: atelier.review
schema_version: 3
round: 4
commit: 21a93b4778b97624ce1a6c1de8851bd604bb40d8
reviewers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3A11bf791a-e30e-4470-8e9b-341bed34d0a2
model: gpt-6-astra
verdict: approve
created_at: 2026-11-11T17:40:23.512506345Z
---

## Summary

Complexity: medium. One history command, its script consumers, and a procedure
with a checklist cross several owners, but introduce no stored history.
Value: high. Repeatable measurements and bounded proposals are necessary to
achieve this mission's outcome.
Risk of merging it: medium. The procedure reaches five repositories and wrong
measurement rules could misdirect upkeep; dated evidence and independent
review make the results inspectable and correctable.

The value warrants the scope. The command and checklist are already done;
the remaining procedure uses those facts without adding another command or
measurement framework. No smaller change would satisfy the stated outcome.

Reviewed the plan at 21a93b4778b97624ce1a6c1de8851bd604bb40d8: the mission,
design, notes and all five item briefs, including the completed items'
reports. Compared the procedure brief with the existing garden and audit
procedures, the product's Scorecard and lifecycle, and the design's log
format and record-writing rules. Traced both carried findings through the
revised item and mission design. No new supported plan finding remains.

This is a plan review, not a rerun of the completed command's tests or the
checklist trials. The arithmetic and rewritten procedure still require the
worked example and implementation review promised by atelier-9dku. Step D
remains expressly provisional and needs its boundary plan review before the
five runs start; this approval does not replace that review.

## Finding 1: Start the next run at the previous measurement cutoff, not its merge
Recommendation: fix now
Was: 3-x9w6.1
Now: fixed

The window requirement in `.atelier/items/atelier-9dku/item.md:29` now records
the measurement start and end, continues from the prior page's end even if
the session is pending, and assigns merges to (start, end]. It also requires
the worked example to include an item merged during the session wait. The
mission design's Shape section uses the same consecutive windows. The
Monday-measurement, Tuesday-merge, Wednesday-session example now puts the
Tuesday item in the following run rather than losing it.

## Finding 2: Do not substitute change volume for the promised rework hotspots
Recommendation: fix now
Was: 3-x9w6.2
Now: fixed

`.atelier/items/atelier-9dku/item.md:57` now explicitly distinguishes rework
from change volume, carries the dated repository rework share, and states
that rework by file is not measured by a routine run. Its fresh file ranking
comes from fix-now findings instead. The mission design's property table
agrees and allows rework by file only when a measurement supplies it. The
page's unavailable-with-reason provision covers the missing ranking without
changing the product's definition of rework.
