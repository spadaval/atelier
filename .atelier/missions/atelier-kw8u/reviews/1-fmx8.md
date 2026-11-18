---
schema: atelier.review
schema_version: 3
round: 1
commit: 93d3b59f2b6a7d600123d55bb187df47f907ddd2
reviewers:
- thread:delegated-task:command%3Amcp%3A899a046c-fc07-4a9b-a0a8-05ac7d8b429d%3Adelegate-task%3Af713a7d4-290a-49ee-8227-4b65d1530eca
model: gpt-6-astra
verdict: changes
created_at: 2026-11-18T21:03:48.476305608Z
---

## Summary

Reviewed the mission and all ten items at 93d3b59f2b6a7d600123d55bb187df47f907ddd2 against the current role instructions, harness, skip trailers and log attribution. Checked the replay source inventory and ran `atelier check`, which accepts the records (with unrelated warnings about three older missions' empty Why sections). This is a plan review; I did not run model evals or validate unimplemented harness behavior.

Complexity: high. This changes lead authority, mixed-provider delegation, grading and the experiment used to choose future instructions.
Value: high. Outcome-based evals and visible departures are needed to assess the proposed lead policy.
Risk of merging it: medium. This is a reversible plan, but its grading and attribution contracts govern subsequent changes to the working process.

The value warrants the work. The following contradictions need resolution before implementation; no wholesale reduction is needed.

## Finding 1: Keep assessments of recorded reasons out of the pass predicate
Recommendation: fix now

`.atelier/items/atelier-b9o6/item.md:36` requires every judge answer to pass for the run to pass. `.atelier/items/atelier-xfsn/item.md` requires the rubric to ask whether a recorded departure's reason holds. Together these fail a run whenever the judge rejects that reason, even when the resulting code works and the departure was recorded. This contradicts `mission.md:121-124`, which explicitly says such departures are reported and not failed, and the results item says the human reads them.

Distinguish questions that determine the grade from observations reported for human judgment. Require the aggregation test to cover working code with a recorded reason that the judge finds unconvincing: report that assessment without changing the pass. Preserve failures for defects and unrecorded departures.

## Finding 2: Record the actual bypass caller before promising to display it
Recommendation: fix now

`.atelier/items/atelier-nv22/item.md:29-31` promises the responsible thread from the existing trailers and limits scope to show and its tests at line 38. Those trailers contain only rule and reason (`crates/atelier-core/src/skip.rs`, `Skip::pass`; `crates/atelier-core/src/git.rs`, `with_skips`). The log does not supply the missing actor: its merge entry uses `new.implementers.holder()` (`crates/atelier-core/src/log.rs:617`), not the thread invoking the merge.

The mission specifically introduces a lead bypassing review on an implementer's item. In that case reusing the log's thread blames the implementer, and no display-only change can recover the actual caller from these trailers. Extend the item and mission scope to record the caller when the departure happens and read it when displaying departures. Add a test with different lead and implementer threads, and specify an honest unknown for historical trailers without caller data. This can use commit metadata without introducing a new tracker record.

## Finding 3: Let the stalled-work case recognize an authorized documented drop
Recommendation: fix now

`.atelier/items/atelier-aq82/item.md:38-42` passes the third, costly item only if merged or its question is raised. The new policy explicitly allows the lead to drop an item when the work is much harder than planned, explaining the omission in the mission Result (`.atelier/items/atelier-mtzj/item.md`, fourth Done when; `mission.md:112-114`). A lead that merges the two useful items and records that justified drop therefore follows the intended policy but fails this case.

Specify how the case grades a documented drop, and include that path in its model-free grader tests. Keep an unexplained abandonment distinguishable from the authorized reduction. Otherwise the before/after comparison penalizes the very discretion the mission asks to evaluate.
