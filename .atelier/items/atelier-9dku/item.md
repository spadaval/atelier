---
schema: atelier.item
schema_version: 3
id: atelier-9dku
title: Rewrite the gardener around the scorecard, three tiers of fix and a one-page report, and fold audit into it
status: in_review
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by:
- atelier-8b0a
- atelier-vl5n
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Ab88e5ac4-cc78-4fea-add2-d618a1992e35
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:41:08.552141251Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:11.236912970Z
---

## Goal

Make a gardener run start from numbers and end with one page Sumeet can settle in one sitting. Today `garden.md` collects incidents and files a digest that asks for a yes or no on each; its one digest, atelier-j86z, is ten entries that reduce to four causes, and it has waited unanswered since 2026-11-08.

## Done when

- [x] `skills/agent-factory/procedures/garden.md` is rewritten as the procedure of one run, in this order: open the run as an item, take the numbers, run the readiness checklist (`readiness.md`), read for causes, fix what the run may fix, write the page, hold the session. `audit.md` is deleted, and what it said that a gardener needs is in `garden.md`: read the code against its docs, and look for half-finished work. `SKILL.md` sends a garden or an audit task to `garden.md` and says the gardener's session is with Sumeet. `groom.md` is unchanged.
- [x] **The numbers.** The procedure gives steps exact enough that two agents get the same numbers from the same `atelier log` output: for the items merged in the window, the time in each status from `ready` on (waiting to start, building, in review, changes requested, approved), rounds, take-overs, escalations, and skips by rule. Each is given as a count or a median with the worst item named. It says which other numbers a run takes (open and blocked work and unsettled deferred findings from `atelier list`; the checklist's lines met and not met) and which it carries forward with the date they were measured (every Scorecard measure that needs T3's events, the transcripts, a rater or a snapshot). A run takes no snapshot and runs none of `scripts/scorecard/`.
- [x] **The window.** The page states its window's start and end, and the end is the time the run took its numbers. The next run starts at that end, read from the last run's page, whether that run is merged or still waits for its session; a first run starts at the first line `atelier log` prints. An item belongs to the run whose window holds its `merge` line, after the start and up to the end, so each is counted once: one merged while a run waited for its session is in the next run. The procedure's worked example includes such an item. A run compares its numbers with the last run's page, and a first run with the November baseline where the repository has one.
- [x] **Causes, not incidents.** The procedure has the gardener group its trips and its numbers by cause before it writes, and bring at most three proposals. Each proposal names one root cause, the trips or numbers that are its evidence with how often each happened, the number it should move, and the change proposed. One that adds a rule or a check names one the gardener considered removing. A trip that fits no cause and happened once is left out. Something broken today is filed as a bug item by whoever hits it and takes no room on the page.
- [x] **Transcripts last.** The procedure has the gardener read a thread only to explain a number that moved or the window's worst item, naming the thread by its ID from the log, and at most three threads for a run's navigation reading.
- [x] **Three tiers.** The procedure says what the gardener does itself on the run's branch, reviewed with the run: upkeep whose correctness is evident from the diff, and fixes to a sign an agent reads where it trips (a line of `AGENTS.md`, a doc, a skill, a script's message), neither adding nor removing a rule, check, gate, interface or default. It says what it only proposes: a new or removed rule or check, a changed default, a restructure, and any change to what the product does. It settles each deferred finding `atelier list --deferred` shows with `atelier new --from` or `atelier drop`, on the run's branch.
- [x] **The page** is the run item's Report, in a layout the procedure gives as a template: the window; the scorecard (this run's numbers, the last run's, the baseline's, and the carried measures with their dates); the checklist's result in a line or two; the worst spots under each of the three properties, as the mission's design lists them, or one line saying what cannot be shown yet and why; the proposals; what the run fixed and settled; and a Session section left empty. It is at most 60 lines, and the workings behind a number go in a file under the item's `artifacts/`, which the page names. No line asks for an answer per trip.
- [x] **The run as an item.** The procedure gives the commands that open it (`new`, `set --labels garden`, `ready --by` whoever asked for the run, `start`), the Done when lines the gardener writes into it, so the run's reviewer gets its checks from the brief, and where it stops: the gardener submits the page for review like any change and, once it is approved, holds the session. In the session it records under Session what Sumeet agreed, changed or declined for each proposal, files each agreed one as an item on the base, and runs `atelier merge`. A proposal still open is carried by the next run once.
- [x] **Shown to work on real output.** The Report holds the numbers of the second line for this repository since the cutover, taken by the procedure's steps from `atelier log`, with the lines behind one item's numbers, so the reviewer can take them again.
- [x] The Report gives the lines of `garden.md` and `audit.md` before (63 and 33) and of `garden.md` after, and says what each part that grew is for. About 120 lines is the indication, not a limit.

## Scope

In: `skills/agent-factory/procedures/garden.md`, `skills/agent-factory/procedures/audit.md` (deleted), `skills/agent-factory/SKILL.md`, and any line of `docs/` or `AGENTS.md` the change makes wrong.

Out, and a review finding that asks for one of these is escalated, not fixed:

- A command, script or page that takes the numbers, runs the checklist or starts a run. The gardener reads two commands' output and does the arithmetic the procedure gives.
- A scheduler for runs (atelier-jy8k), `groom.md`, and an eval scenario for the gardener. No eval covers the procedures the skill carries, and adding one is its own item.
- A change to `readiness.md`'s lines, to any measure's definition in `docs/product.md`, or under `crates/` or `scripts/`.
- A run. The first run in each repository is atelier-613a, and atelier's goes first as this procedure's trial. Nothing here opens a `garden` item or touches atelier-j86z.
- Scoring an agent, a pass mark, or a target for any number.

## Design

Settled here; change one only with a reason in the Report.

- **The session comes before the merge.** A done item's directory can no longer change, and the page must record what was agreed. So the run waits approved until the session, where `status` shows it with since when, and the session's thread merges it, taking the item over with `atelier start <id>` if the gardener's thread is gone. An edit to the Report does not make the approval stale; a change outside the item's directory does.
- **Carried measures.** Process share, confusion, tokens per merged item, tool calls before first edit, rework, gate precision, review yield and defects found after merge need a frozen snapshot of about 7.5 GB or a rater. A run does not take them. It shows the last measured value with its date, and a measurement item such as atelier-pwhn refreshes them.
- **Navigation's worst spots come from a small reading, said to be one.** The gardener reads up to three implementer threads the log names, those of the items longest in `building`, up to their first edit, and lists the files they read. In T3 Code a thread is read by its ID with the thread-read tool. Where no item has run in the window, the page says so and shows the checklist's navigation lines instead.
- **Change's and validation's worst spots come from the records.** For change: the files most named in the window's fix-now findings. Rework is lines deleted within two weeks of being written (`docs/product.md`), and no routine run can rank files by it, so the page carries the repository's rework share with its date and says rework by file is not measured. It does not rank files by how much they changed, which a large new feature would lead with no rework at all. For validation: the features the verify skill gives no proof command for, and the bug items with their `caused_by`.
- **Size.** Every paragraph of the procedure must change what a gardener does. Explanation of why the design is so stays in the mission's `design.md`.

## Context

Mission: `.atelier/missions/atelier-295u/` (design.md, Shape, holds each rule above in a sentence). Specifications: `docs/product.md` (Words: Gardener; Scorecard; Where records change) and `docs/design.md` (The log, with the line layout; Records, for deferred findings). The checklist: `skills/agent-factory/procedures/readiness.md`. The baseline: `docs/architecture/quality/scorecard-baseline-2026-11.md`.

The skill is 319 lines after mission atelier-qr5s cut it by an eval, and an agent reads this procedure at the start of every run. Keep it to what a run needs.

## Report

A gardener now has one procedure that starts from `atelier log` and `atelier list`, counts the same numbers any other agent would from the same output, and ends in one page of at most three proposals that Sumeet settles in a session before the run merges. `skills/agent-factory/procedures/garden.md` is rewritten in the seven steps the item gives, `procedures/audit.md` is deleted, and `SKILL.md` sends a garden or audit task to `garden.md` and says the gardener's session is with Sumeet. `groom.md` and `readiness.md` are unchanged.

### Size

Before: `garden.md` 63 lines and `audit.md` 33, 96 together. After: `garden.md` 176 lines and no `audit.md`. What audit said that a gardener needs (read the code against its docs; the forms of half-finished work) is the first paragraph of step 4, five lines. The rest of audit went: its findings format and its retrospective method are replaced by the page and by step 4's rule for reading threads. The growth over the indication of about 120 is in parts the Done when lines ask for and an agent cannot work without:

- Step 1, 16 lines: the commands that open the run and the five Done when lines the gardener writes, so the run's reviewer gets its checks from the brief.
- Step 2, 51 lines: the window, the `awk` that picks the merged items, the arithmetic for each number, the `atelier list` counts, what is carried, and the worked example (10 lines). This is what makes two agents get the same numbers; the old procedure took no numbers.
- Step 6, 36 lines: the page's template (27 of them) and the rule for a carried proposal.
- Step 7, 9 lines: submit, the session, and the merge.

The old Digest section (19 lines) and Fixing at the Right Level (16) are gone; step 5 (13 lines) holds the three tiers and the deferred findings.

### Numbers for atelier since the cutover, taken by the procedure's steps

Window: 2026-11-11T15:20:09Z (the first line `atelier log` prints, the conversion commit) to 2026-11-11T17:42:30Z, when I took the numbers. The `awk` of step 2 gives five merged items, in merge order: atelier-2mlh, atelier-ivbq, atelier-6w8d, atelier-vl5n, atelier-8b0a.

| Item | Waiting | Building | In review | Changes requested | Approved | Rounds | Take-overs | Escalations | Skips |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| atelier-2mlh | 0 (15 s) | 3 (163 s) | 2 (135 s) | 0 | 0 (18 s) | 1 | 0 | 0 | 0 |
| atelier-ivbq | 0 (19 s) | 13 (775 s) | 6 (346 s) | 3 (181 s) | 1 (32 s) | 2 | 0 | 0 | 0 |
| atelier-6w8d | 0 (16 s) | 20 (1183 s) | 13 (768 s) | 1 (48 s) | 30 (1793 s) | 3 | 0 | 0 | 1 |
| atelier-vl5n | 1 (44 s) | 10 (618 s) | 7 (398 s) | 2 (112 s) | 0 (16 s) | 3 | 0 | 0 | 0 |
| atelier-8b0a | 1 (44 s) | 16 (986 s) | 6 (354 s) | 2 (145 s) | 0 (11 s) | 2 | 0 | 0 | 0 |

Times are minutes, with the summed seconds beside them. Across the five:

- Waiting to start: median 0 minutes, worst atelier-vl5n (1; tied with atelier-8b0a, and atelier-vl5n merged first).
- Building: median 13, worst atelier-6w8d (20).
- In review: median 6, worst atelier-6w8d (13).
- Changes requested: median 2, worst atelier-ivbq (3).
- Approved: median 0, worst atelier-6w8d (30). Its merge line carries the one skip: CI's desk job could not install Playwright's packages three times, so it waited half an hour approved.
- Rounds: median 2, worst atelier-6w8d (3; tied with atelier-vl5n, and atelier-6w8d merged first).
- Take-overs 0, escalations 0. Skips by rule: `CI passed on the head` 1, on atelier-6w8d.

From `atelier list` at 2026-11-11T17:46Z: 130 open items (107 draft, 21 ready, 2 building), 10 blocked (7 draft, 3 ready), and 1 unsettled deferred finding, atelier-6w8d/3-kvll.1. The checklist's lines met and not met are not taken here: running it is part of a run, and atelier's first run is atelier-613a.

The lines behind atelier-ivbq's numbers, from `atelier log atelier-ivbq` with the thread column cut:

```
2026-11-11T15:43:38Z 96d02deb7 atelier-ivbq ready draft -> ready; by sumeet
2026-11-11T15:43:57Z 4ad06abae atelier-ivbq start ready -> building
2026-11-11T15:56:52Z 122c0f5a7 atelier-ivbq submit building -> in_review
2026-11-11T16:00:25Z 5dec1e884 atelier-ivbq review 1-o9m9
2026-11-11T16:00:25Z 5dec1e884 atelier-ivbq verdict 1-o9m9 changes; in_review -> changes_requested
2026-11-11T16:03:26Z 6dd7fc218 atelier-ivbq submit changes_requested -> in_review
2026-11-11T16:05:39Z 6318ffd9d atelier-ivbq review 2-6zsp
2026-11-11T16:05:39Z 6318ffd9d atelier-ivbq verdict 2-6zsp approve; in_review -> approved
2026-11-11T16:06:11Z 730e5bf2c atelier-ivbq merge approved -> done
```

Ready 15:43:38 to start 15:43:57 is 19 s; building to 15:56:52 is 775 s; in review 213 s then 133 s, 346 s; changes requested 181 s; approved 32 s; two `submit` lines.

### Choices

- The page's numbers are per item in whole minutes before the median, so a reviewer can check each from one item's lines. Skips are counted on the merged items' lines, as the Done when line groups them.
- A carried proposal counts as one of the three, so the page never holds more than three.
- The procedure gives the commands for `atelier list` counts and the merged items as one-line `grep` and `awk` filters, which narrow the output as `atelier log --help` suggests. It adds no script.
