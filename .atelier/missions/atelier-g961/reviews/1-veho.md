---
schema: atelier.review
schema_version: 3
round: 1
commit: bafc413fc011abbe76e20dbd2f8babb2d292eb25
reviewers:
- 01a1cd49-05ea-73a0-9d6c-b792d5def575
model: grok-4.6
verdict: changes
created_at: 2026-11-12T01:05:02.996453237Z
---

## Summary

Reviewed the plan as commit bafc413fc011abbe76e20dbd2f8babb2d292eb25 holds it: the mission, all three draft items, and the files they rewrite (`garden.md`, `readiness.md`, `groom.md`, `planner.md`, `SKILL.md`) plus the named lines in `docs/product.md`, `docs/product/desk.md` and `docs/architecture/desk.md`. Checked the Why against atelier-613a's Report, atelier-h201 (dropped into wb05), atelier-8i86's close, atelier-0019, atelier-jy8k, and the 295u gardener design. Counted today's line budgets with `wc -l` (garden 188 + readiness 110 = 298; planner 11 + groom 85 = 96). Did not start a gardener, read another reviewer's file, or replay the 2026-11-11 sweep item by item (117 `Drop atelier` commits that day; the mission says 104).

- Complexity: medium. Two procedure rewrites and one trial; no new command or record kind.
- Value: high. 613a's nine proposals and 8i86's 21-entry digest show the two human-facing jobs failing in the way the Why states.
- Risk of merging it: medium. A drop on the base is visible and reversible, and a bad garden page is caught in session; a drop rule agents can follow two ways will silently remove work Sumeet still wants.

The split is worth doing. No smaller change gets the outcome. Six contract gaps need closing before the mission starts: the planner cannot tell what it may drop, the gardener cannot tell which threads or which focus, and the trial's "nothing else" task will not produce the window or the quotes the criteria demand.

## Finding 1: Define the test for an item Sumeet directed or parked
Recommendation: fix now

Mission Authority forbids the planner to drop "an item Sumeet directed or parked himself", and `.atelier/items/atelier-s47q/item.md` line 32 repeats that as a Done when line, "whatever its state". `docs/product.md` lines 90–99 give the item statuses: draft, ready, building, in_review, changes_requested, approved, done, dropped. There is no parked status. The only remaining uses of "parked" in the skill are `groom.md` lines 20 and 35, leftover from the old tool.

"Directed" has no test either. `ready.by` on converted items is `conversion` (atelier-jy8k, atelier-8i86). Items Sumeet readied name him. `start --new` is the command for work he asked for directly (`docs/product.md` line 192). Without saying which of those counts, two planners will drop different sets, and the drops-without-asking criterion cannot be checked.

Say the test in `backlog.md`: for example, do not drop an item whose `ready.by` names Sumeet, one created with `start --new`, or one whose body still carries a pick-up condition he wrote. Translate "parked" into those records, and stop using a status the tracker does not have.

## Finding 2: Gone names cannot both prove a drop and justify a rewrite
Recommendation: fix now

`.atelier/items/atelier-s47q/item.md` line 31 lists four proofs for a drop the planner runs itself, including "a search of the code that finds nothing the item names". The same sentence says the planner "may also ... rewrite a kept item that names removed code so it describes the same work against today's code". Those are two different acts on the same evidence.

A rename or a finished migration both make a search find nothing. The first is still work to do; the second is stale. The 2026-11-11 sweep used drop commits whose reasons named the replacing work, which matches the other proofs on that line (a commit that did the work, or the replacing ID). A search-only proof does not.

Split the proofs: drop when a commit or a replacing item shows the work is done or duplicated; rewrite, or put on the seven-call list, when named code is gone and the work may still be wanted. A search that finds nothing is evidence for that call, not a drop by itself.

## Finding 3: Reading every escalation and skip fights the lead's cap on threads
Recommendation: fix now

`.atelier/items/atelier-wb05/item.md` line 32 requires the run to choose threads including "every escalation and every skip" and read them in full. Mission Authority, `mission.md` lines 94–95, says the lead may choose how many threads a run reads. Those cannot both hold. The mission criterion `transcripts-first` (`mission.md` lines 59–61) only needs a numbered choice, a full read, and no first-edit cap.

"Every" also has no bound. A window with many skips becomes a transcript slog, which is the cost the cheaper-agent clause is meant to cut, and the item still requires the gardener to read each of those threads in full after the extractors.

Put the selection rule in `garden.md` as a numbered choice the lead can cap (longest in each status, most rounds, then escalations and skips until the cap). Keep the full-read and the check-each-claim rule. Drop "every" from the item.

## Finding 4: Say how a run picks a focus when no area has been looked at
Recommendation: fix now

Criterion `purpose-and-focus` (`mission.md` lines 55–58) requires that a thread told only "I want to do some gardening" can say what it will look at and why before it measures. `.atelier/items/atelier-wb05/item.md` line 31 says the default is "the area the earlier pages show longest unlooked at".

The first run under the new procedure has no page that recorded a focus. atelier-0019's page has none of the eight areas. After that, several areas can be equally unlooked at. The item gives no tie-break, so two gardeners given the same prompt will pick different areas, and the criterion is not checkable.

Name the default in `garden.md` before step 2: Sumeet's named focus; else the area absent from the most recent pages; else a listed tie-break (for example structure). Record it in the run item's Goal, then take the numbers.

## Finding 5: The trial gardener will not take 0019's window
Recommendation: fix now

`.atelier/items/atelier-s0j2/item.md` line 23 starts the gardener with the task "Garden this repository; focus on structure" and nothing else. Line 43 says in Context that the window starts where atelier-0019's page ends. The gardener never reads that Context. It reads `garden.md`.

`garden.md` lines 29–31 start the window at the end the last run's page gives, found as "the newest item labelled `garden`, merged or still waiting for its session". Step 1 (lines 8–10) has already created and labelled this run. The newest waiting garden item is then this run, which has no page, or whichever of this run and 0019 sorts as newest. The gardener told nothing else has no instruction to skip itself and use 0019.

The rewrite must say the previous run is the newest *other* garden item that already has a page, and that the window starts at that page's end, not at this run's creation. Put that in `garden.md` (wb05). s0j2 can keep the 0019 note as a check, not as the only place the rule lives.

## Finding 6: Thread quotes on the page go to public Git
Recommendation: fix now

Criterion `trial` (`mission.md` lines 62–65) and `.atelier/items/atelier-s0j2/item.md` line 31 require at least one proposal to rest on evidence quoted from a thread, named by its ID. The run item's Report is the page, and it reaches `master` on merge. This repository is public (`gh repo view spadaval/atelier`: visibility PUBLIC).

atelier-613a's Design (lines 54–55) already forbade pages and artifacts to hold text quoted from a transcript, a thread title, a home path, or a personal detail, because a page ends up in Git. s0j2 does not repeat that rule and instead requires a quote. `garden.md`'s page template has no such limit either.

Say what "quoted from a thread" may be on a public page: a thread ID and a trip the gardener checked (command, file, refusal), with no transcript body, no title, no home path and no personal detail. Put that in `garden.md` and in s0j2's Done when, so the trial can pass without committing T3 transcripts.
