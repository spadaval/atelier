---
schema: atelier.mission
schema_version: 3
id: atelier-295u
title: Give the gardener `atelier log` and a readiness checklist, so a run starts from facts
status: done
labels:
- redesign
lead: thread:delegated-task:command%3Amcp%3A386345c9-a3d2-4f5d-b47b-a12ac38636b5%3Adelegate-task%3A5850a386-2088-4c43-87fe-0dec52d3ebbe
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
created_at: 2026-11-10T00:56:04.888418157Z
---

## Why

The gardener is the role meant to keep a codebase easy to navigate, easy to
change and easy to validate. As written it collects incidents. It has no
numbers, so it cannot tell whether its last fixes worked. It hears refusals
and retry loops and misses slow costs, such as an agent reading 40 files
before it can edit. Its one digest, atelier-j86z, is ten entries about the
tool and none about a codebase, and it waits in a queue where 6 of 63 items
have been answered.

It also works like a firefighter. Its unit is the trip, and it asks Sumeet
for a yes or no on each one, which gives every incident the same weight. A
trip is a data point. Read by cause, the ten entries of atelier-j86z are four
causes and one stray: records that live on the base while agents read
elsewhere (two entries), docs and help that lag the code (three), checks that
run in the wrong place (three), and an error the tool hides (one). Four of
the ten are help or doc fixes that needed no decision.

After the cutover the acts that move an item leave their mark in its records,
and so in Git: a status, a holder, a verdict, an answered question. The
history of what happened to each item can be read from there. Nothing reads
it yet.

## Outcome

`atelier log` prints the history of acts. A gardener run starts from that,
from `atelier list`, and from a readiness checklist for the repository, and
its procedure says which numbers to take from them. It reads transcripts
only to explain a number that moved. It settles the deferred findings. It
ends with one page for Sumeet: the numbers against the baseline, the worst
spots, and at most three proposals, each a root cause with the trips that
show it. One run has been made in each of the
five repositories.

## Success criteria

- one-page: a gardener run in each of the five repositories produces a report
  of one page that holds the scorecard, the worst spots under each of the
  three codebase properties, and at most three proposals. Each proposal
  names one root cause, the trips or numbers that are its evidence, and the
  number it should move. No page asks for an answer per trip.
- from-log: the scorecard's numbers about what happened to items, which are
  time in each status from `ready` on, rounds, roles taken over, escalations
  and the skips `log` shows, are computed from the output of `atelier log`
  by the steps the
  gardener's procedure gives, and no script walks record history itself.
  The scripts that remain tie a thread to an item by the thread IDs in the
  log, with no matching on thread titles.

## Scope

- In: `atelier log`, the scripts that remain, the checklist, the gardener
  procedure, the first runs.
- Out: a command that computes the scorecard; a scorecard page on the site;
  a command that starts a run or runs the checklist; a scheduler for the
  gardener; the groom procedure; scoring agents on any measure; the
  comparison two weeks after the cutover, which is its own item.
- Target environment: the five repositories on this host.

## Authority

The orchestrator may choose the checklist's lines, the report's layout, the
layout of a log line, and which transcript reading a run needs. It may not
add a command beyond `log` or a site page, add a rule or check to any
repository from a gardener run, or change a measure's definition in the
product specification.

## Result

A gardener run now starts from `atelier log`, `atelier list` and a readiness checklist, and ends with one page. One run was made in each of the five repositories on 2026-11-11. Both criteria hold. As of 2026-11-13 all five runs are approved and still wait for their sessions with Sumeet; their merges are his and are not part of this mission.

**What was built.** Five pull requests, each reviewed by GPT-6.1 Sol, with the plan reviewed by GPT-6 Astra at each step's boundary (five rounds, four findings, all fixed before work started):

- `atelier log`, the one command added (atelier-8b0a, pull request 145): one line per act, read from Git in one walk, and the same history as data from `atelier-app` for the Desk. In astromech it printed 88 lines in 0.07 seconds.
- The scorecard scripts read a frozen `atelier log` (atelier-x3n5, pull request 147). They no longer read the old record layout, walk commit messages or record versions, or match an item ID in a thread's title. Outside the tests they gained 168 lines and lost 171. Only the scripts at commit df9c84a43 reproduce the November baseline.
- The readiness checklist, nine lines under the three properties, in `skills/agent-factory/procedures/readiness.md` (atelier-vl5n, pull request 144). A repository's own lines live in its `.atelier/readiness.md`.
- `garden.md`, rewritten as the procedure of one run, with `audit.md` folded into it (atelier-9dku, pull request 146), and the fixes atelier's trial run showed it needed (atelier-f6dv, pull request 149).

**one-page.** The five pages are 36 to 46 lines against a limit of 60, with nine proposals in all, each naming one cause, its counted evidence and the number it should move. No page asks for an answer per trip. The run items are atelier-0019 (pull request 148), astromech-qjle, worldcrafter-gq2k, lor-j26w and cosplay-studio-ktgi; atelier-613a's Report lists their worktrees and proposals. Only atelier and demiurge had merged an item since the cutover, so three pages report the baseline, the checklist and what the records show, and say so.

**from-log.** `garden.md` gives the steps that take time in each status from `ready` on, rounds, take-overs, escalations and skips from `atelier log`, and atelier's run took them that way for seven merged items. No script walks record history, and the scripts tie a thread to an item by the log's thread IDs.

**What the runs showed.** The checklist said more than the log on day one. In all five repositories no merge runs every documented check, and no bug item names its cause (39, 6, 2, 7 and 2 items); seven of the nine proposals are about those two. The cutover left each repository's own docs, skills and scripts naming removed commands and records, and one astromech script broke silently (astromech-xj3u).

**Left open, outside the mission.** The five sessions, which settle the proposals, close atelier-j86z if Sumeet agrees, and merge each run with its small doc fixes. atelier-qf7u, a deferred finding taken on atelier-0019's branch, reaches the base with that merge. Follow-ups: atelier-52g7 (the cutover reports what it leaves stale; it blocks the work host's adoption, atelier-7w9s), atelier-c1z3 and atelier-pwhn (the rerun and the comparison two weeks after the cutover), atelier-y0ba (`status` says an approved run waits for its session, which Sumeet agreed on 2026-11-13 in atelier-e7iy), and atelier-wb05, which took over atelier-h201's list of what four runs had to guess in the procedure.

