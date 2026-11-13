---
schema: atelier.item
schema_version: 3
id: atelier-qb4f
title: 'Garden atelier: structure, 2026-11-13'
status: changes_requested
mission: null
labels:
- garden
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a128a06-6a57-49de-b3ea-7ef651a4c883%3Adelegate-task%3A5c4d50b4-aed9-49b9-86e1-036314a09ad4
ready:
  by: sumeet
  at: 2026-11-13T08:41:35.493423857Z
trivial: null
caused_by: null
created_at: 2026-11-13T08:41:28.221257700Z
---

## Goal

Garden atelier with structure and design drift as the focus, because Sumeet named it: the modules, what each is for, and the docs against the code. Bring him one page with at most three proposals, and fix only upkeep and signs on this branch. The window starts where atelier-0019's page ends, 2026-11-11T17:56:43Z.

## Done when

- [x] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [x] The focus area's threads and code were read as garden.md says, and nothing private reaches the page.
- [x] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [x] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [x] Each finding `atelier list --deferred` showed is settled.
- [x] The page follows garden.md's template in at most 60 lines, with Session empty.

## Scope

In: `skills/agent-factory/procedures/garden.md` steps 1 to 7 up to the approval, for atelier only.

Out: the session and what it settles, the run's merge, any new or removed rule, check, gate, interface or default, a run of `scripts/scorecard/`, and other items' branches.

## Report

Focus: structure and design drift, named by Sumeet.
Window: 2026-11-11T17:56:43Z to 2026-11-13T08:41:43Z UTC, 3 items merged. Workings: `artifacts/numbers.md`, `checklist.md`, `threads.md`, `structure.md`.

| Measure | This run | Last run | Baseline |
| --- | --- | --- | --- |
| Waiting to start, building, in review, changes requested, approved: median minutes (worst) | 1 (s47q 1), 1 (wb05 6), 3 (wb05 6), 0 (wb05 1), 0 (s47q 0) | 0 (x3n5 27), 11 (6w8d 20), 6 (6w8d 13), 2 (ivbq 3), 0 (6w8d 30) | not measured |
| Rounds: median (worst); take-overs; escalations; skips by rule | 2 (f6dv 2); 0; 0; none | 2 (6w8d 3); 0; 0; `CI passed on the head` 1 | no skip mechanism |
| Open items by status; blocked; unsettled deferred findings | 22 draft, 3 ready, 3 building, 1 approved; 10; 2 | 106 draft, 22 ready, 1 building; 8; 2 | not measured |
| Defects after merge, measured 2026-11-10 | 15 of 277 (5.4 per 100), a floor | carried | 15 of 277 |
| Interventions per mission, measured 2026-11-10 | 0 to 7, 12 missions | carried | 0 to 7 |
| Process share, measured 2026-11-10 | 28.0% of commands, 21.6% of tokens | carried | same |
| Confusion per 100 `atelier` calls, measured 2026-11-10 | 25.2 | carried | 25.2 |
| Tokens per merged item, median, measured 2026-11-10 | 3.06 USD | carried | 3.06 USD |
| Gate precision; review yield, measured 2026-11-10 | 4 of 54; 8 of 20 | carried | same |
| Calls before first edit, median, measured 2026-11-10 | 22 | carried | 22 |
| Rework, measured 2026-11-10 | 10.8% | carried | 10.8% |
| Where caught: review, CI, after merge, measured 2026-11-10 | about 191, 6, 15 | carried | same |
| Non-test lines of Rust, measured 2026-11-10 | 48,989 | carried | 48,989 |

What means nothing yet: the log numbers rest on 3 items, all edits to skill documents and none to code, made in minutes by agents. The draft count fell from 106 because a sweep dropped the cutover's records. The carried rows are still the baseline.
Threads: 3 items read (6 threads), 0 more the order would have picked.
Checklist: 4 met, 5 not met (4: `desk/README.md` builds the server from the deleted `next/`, repaired only on atelier-0019's unmerged branch; 5: the Desk's app has no format or lint command; 7: no merge runs clippy; 8: no verify skill; 9: atelier-k55o and atelier-o91z are bugs with no cause), 0 could not test.
Backlog: a pass looks due once atelier-to83's items ob9u and 2vdq merge, because they change what draft and ready mean. Seen stale: atelier-j86z, which atelier-0019 found 6 of 10 moot.
Worst spots: navigate `crates/atelier-cli/src/` (5 guessed file names, below); change `skills/agent-factory/procedures/garden.md` and `readiness.md`, 1 fix-now finding each; validate the Desk's app, which has no lint, and every feature, with no verify skill.

### Proposals
#### P1 Put every act in atelier-app, and leave atelier-cli to parse and print
Cause: an act lives in atelier-app only when the Desk needed it, so 2 of 13 acts are there (`ready`, `decide`) and 11 are in atelier-cli. Evidence: `artifacts/structure.md`'s table; atelier-to83's ob9u, 2vdq and xbog each scope both crates, and 2vdq edits `plan.rs` in both; atelier-v958 needs the Desk to write what `atelier drop` writes and publish it, but `drop` and the push are in atelier-cli, which the Desk does not link.
Moves: Confusion and calls before first edit, for items that touch acts. Change: before 2vdq and v958 start, move `drop`, `close`, `new`, `set` (atelier-cli `plan.rs`) and the push and pull-request code (`merge.rs`, `forge.rs`) into atelier-app as functions returning data, as `ready` and `decide` already are; then the rest. The CLI keeps arguments and `output.rs`.
#### P2 Name each command's file after the command
Cause: atelier-cli files commands by theme (`commands.rs` holds `status`, `start` and `submit`; `questions.rs` holds `escalate`; `views.rs` holds `show`, `list` and `lead`), so an agent looking for a command guesses a file that does not exist. Evidence: 5 failed path guesses in 2 of 6 threads. f6dv's round-2 reviewer searched `submit.rs` and `status.rs`, named for commands, and `settle.rs` and `next.rs`, named for concepts, under `crates/atelier-cli/src/`, then searched wider and read `plan.rs`; wb05's searched `commands/show.rs`, then ran `atelier show` instead.
Moves: calls before first edit (22, the baseline). Change: one file per command where P1 puts it (`start.rs`, `submit.rs`, `escalate.rs`, `show.rs` and so on). That splits `commands.rs` and `views.rs` and moves their shared helpers, so it is cheapest done with P1, not as renames.
#### P3 Give a document's size as a target the reviewer weighs, not a Done when count
Cause: two items capped procedure documents by exact line count, and the documents are hard-wrapped, so meeting the count meant reflowing paragraphs. Evidence: atelier-f6dv's implementer made 6 more edits to cut 3 lines (positions 13 to 23); atelier-wb05's made about a dozen edits and recounts to reach 296 of 298 lines. 2 of 3 items in the window.
Moves: time in building. Change: planners write "about N lines, and say why if more" in Scope, and the reviewer judges it. Considered removing: the line caps outright; kept as targets, since size is the signal Sumeet asked for.

### Fixed and Settled
- this branch: sign. Six module headers named seven deleted old-tool files as live `crates/` paths; each now says the old tool's, deleted in 819024c75. `docs/design.md`'s crate table now says which acts atelier-app holds.
- atelier-6w8d/3-kvll.1 and atelier-x3n5/1-c6ma.3: settled on atelier-0019's branch (dropped, and taken into atelier-qf7u), which has not merged; settling them here too would conflict with it. Bugs filed: none. Waiting for the merge: none.

### Responses

### Session
