---
schema: atelier.review
schema_version: 3
round: 2
commit: 31a5d5d85de3ae45e4a87fb2dc9c65983fcf67d3
reviewers:
- thread:delegated-task:command%3Amcp%3Aa0526aa8-7650-4e5d-80ff-0b0535866187%3Adelegate-task%3A331f7cac-64b2-4865-a392-d169227092cf
model: gpt-6-astra
verdict: approve
created_at: 2026-11-12T01:13:48.269441375Z
---

## Summary

Reviewed mission atelier-g961 at 31a5d5d and all three draft items against
all seven carried findings, the existing skill and planner, garden, readiness
and groom procedures, atelier-h201's gap list, atelier-0019's record, and the
referenced product and Desk documentation. Checked scope ownership,
dependencies, authority and acceptance criteria. This is a plan review: the
rewritten procedures and trial do not exist yet. I did not run the trial,
recount the historical drops or independently inspect the first runs' threads.

- **Complexity: medium.** Two shared procedures change, followed by a separate
  gardener trial; there is no new command or subsystem.
- **Value: high.** The plan directly addresses weak gardening evidence and the
  backlog process's dependence on human decisions for provably stale work.
- **Risk of merging it: medium.** Shared instructions expand autonomous drop
  authority, but require outcome-level proof and preserve explicit exceptions.

The scope is worth doing. A smaller wording-only change would not establish
safe autonomous drops or validate the gardener's behavior. All carried
findings are fixed in the plan; no additional fix-now finding remains.
Implementation review must still check that the promised rules survive the
line budgets, and the trial must supply the required evidence.

## Finding 1: A missing code reference does not prove the work is stale
Recommendation: fix now
Was: 1-pxw5.1
Now: fixed

atelier-s47q now requires proof against the Goal and every unfinished Done when line, records it in the drop reason, and preserves partially unfinished work. Missing code names explicitly are not proof.

## Finding 2: Define the test for an item Sumeet directed or parked
Recommendation: fix now
Was: 1-veho.1
Now: fixed

The mission Authority and atelier-s47q now define protected human-directed items by ready.by or explicit body evidence of the request or pick-up condition. Started items and missions led elsewhere remain protected.

## Finding 3: Gone names cannot both prove a drop and justify a rewrite
Recommendation: fix now
Was: 1-veho.2
Now: fixed

atelier-s47q separates obsolete wording from an obsolete need. It requires tracing replacements, retaining unfinished work, and taking unresolved cases to Sumeet.

## Finding 4: Reading every escalation and skip fights the lead's cap on threads
Recommendation: fix now
Was: 1-veho.3
Now: fixed

atelier-wb05 now specifies ordered selection up to a procedure-defined cap, while retaining full reads and claim verification. It no longer requires every escalation or skip transcript.

## Finding 5: Say how a run picks a focus when no area has been looked at
Recommendation: fix now
Was: 1-veho.4
Now: fixed

atelier-wb05 now chooses the first never-recorded area in list order before the least recently recorded area, and explicitly treats older pages without a focus as recording none.

## Finding 6: The trial gardener will not take 0019's window
Recommendation: fix now
Was: 1-veho.5
Now: fixed

atelier-wb05 now requires the newest other garden item with an existing page as the window source. atelier-s0j2 checks that the trial discovers the prior window through that procedure.

## Finding 7: Thread quotes on the page go to public Git
Recommendation: fix now
Was: 1-veho.6
Now: fixed

The mission, atelier-wb05 and atelier-s0j2 now require a checked trip described in the gardener's words with its thread ID, and forbid transcript text, titles, home paths and personal details on the page and in artifacts.
