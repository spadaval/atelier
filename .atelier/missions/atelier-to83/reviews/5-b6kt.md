---
schema: atelier.review
schema_version: 3
round: 5
commit: 16e81ecae54f9b410eaf0099b44a82b379fab04c
reviewers:
- thread:delegated-task:command%3Amcp%3Adbe6537b-1cdf-48e8-acc9-b67a0a17e994%3Adelegate-task%3Ae4b2ec45-efd7-490a-8b2c-8e58849752e5
model: gpt-6-astra
verdict: approve
created_at: 2026-11-13T06:49:33.287756578Z
---

## Summary

Reviewed the plan at `16e81ecae54f9b410eaf0099b44a82b379fab04c`: the
mission, its design document and all eight items. Compared the four carried
findings with the revised requirements, and checked the existing record
precedence and merge rules in `docs/design.md` and the planner skill's blind
review procedure. `atelier check` passes for all 153 items, nine missions and
config.toml.

- Complexity: high. Branch discovery, remote claims, plan review gates,
  forge merges and live Desk reads change together across five repositories.
- Value: high. Protected-base operation and visibility across clones require
  changing the current base-writing model.
- Risk of merging the planned implementation: high. Incorrect precedence or
  merge checks could silently revive stopped work or merge unchecked code.

The value warrants the scope. The documentation-first dependency, explicit
cross-clone tests, compatibility requirements and staged rollout address the
main risks; a substantially smaller change would not deliver the stated
outcome. This is approval of the plan, not verification of its implementation.
I did not run the proposed concurrency, forge or live-repository proofs; those
remain acceptance work for the named items. No new fix-now findings.

## Finding 1: A started item's drop never lands on the base, and has no act
Recommendation: fix now
Was: 4-to95.1
Now: fixed

`atelier-ob9u/item.md:38` now names the actor and checkout: any thread from
any checkout, writing the item's branch through its ref and refusing dirty
item-directory changes. The same invocation lands the base copy through a
record-only branch, with retry after status 3. It explicitly tests both
branch deletion after the drop and refusal of `start` in a second clone
after fetching. The design's started-item-drop decision agrees.

## Finding 2: atelier-ogpg still documents three kinds of branch
Recommendation: fix now
Was: 4-to95.2
Now: fixed

`atelier-ogpg/item.md:29` distinguishes the three branches that merge from
review branches brought onto the mission's branch by the planner.
`atelier-ob9u/item.md:30` spells out all four prefixed names, including
`x/review/<mission-id>/<round>-<suffix>`, and requires one tested inverse
mapping. Together with dciw's convention-based discovery, this settles the
prefix interpretation for the reader as well as the writer.

## Finding 3: show of a proposed plan reads the other reviewer's verdict
Recommendation: fix now
Was: 4-to95.4
Now: fixed

`atelier-dciw/item.md:31` now permits only a count before the round is
brought onto the mission's branch and requires a two-clone test that no
verdict is printed. The design and `atelier-v958` apply the same distinction
to the Desk. This preserves the existing procedure of gathering reviews
once both reviewers have returned their verdicts.

## Finding 4: a dropped proposed plan still matches the definition of proposed
Recommendation: fix now
Was: 4-to95.5
Now: fixed

`atelier-dciw/item.md:32` explicitly excludes a dropped branch-only mission
from proposed results and requires both `list` and `show` to give its dropped
status and reason. The design's mission-stop decision and the Desk item agree.
Although the general descriptions of proposed missions remain abbreviated,
this specific exception settles their behavior for the case in the finding.
