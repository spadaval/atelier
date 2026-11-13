---
schema: atelier.item
schema_version: 3
id: atelier-wb05
title: Rewrite garden.md so a run has a stated purpose, a focus and transcripts as its main evidence
status: changes_requested
mission: atelier-g961
labels:
- agent-factory
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Abe2146c6-f4c6-4217-8f3a-ee1dc9eec577%3Adelegate-task%3A5cec947d-b329-4fc8-a2a7-de5ceac8e5db
ready:
  by: sumeet, in conversation on 2026-11-13
  at: 2026-11-13T08:27:11.885174987Z
trivial: null
caused_by: null
created_at: 2026-11-12T01:03:20.840469739Z
---

## Goal

Rewrite the gardener's procedure so a run is a look at a repository's health
with Sumeet in the loop, and not a measuring routine. Today `garden.md` never
states its purpose, reads threads last and only up to their first edit, and
gives the code one sentence. Seven of the nine proposals from the first five
runs restate a checklist line.

## Done when

- [x] **Purpose.** `garden.md` opens, before step 1, with at most 15 lines saying what gardening is for (a repository that stays easy to navigate, change and validate, and the redesigns that need Sumeet's taste), what a run produces, and what it is not: a linter, a firefighter, or the backlog pass.
- [x] **Areas.** It lists the areas a run may focus on, each in a line saying what to read: structure and design drift; tests (what they prove, speed, flakiness); dependencies and toolchain; features and code nobody uses; how work flows (the `atelier log` numbers); the host and the agents' setup (worktrees, hooks, skills, memory files); Sumeet's time (how often and how long work waited on him); cost.
- [x] **Focus.** A run starts with a focus, chosen by a rule two gardeners would apply the same way: the one Sumeet names; else the first area in the list's order that no earlier page records as a focus; else the one recorded longest ago. A page with no focus line, such as each of the first five, counts as recording none. The run item's Goal and the page's first line record it before the numbers are taken. The numbers and the checklist are taken every run whatever the focus.
- [x] **Window.** The window starts at the end the last run's page gives. The last run is the newest item labelled `garden`, other than this run, whose Report has the template's `Window:` line. The procedure says to read each candidate with `atelier show <id>`, which reads a started item from its branch, since a run waiting for its session has no Report on the base. An item that shares the label and has no such line, such as the old digest atelier-j86z, is not a run.
- [x] **Threads first.** Step 4 has the run choose threads by the numbers in a stated order (the item longest in each status, then the items with the most rounds, then those with escalations, then those with skips) up to a number of items the procedure gives. For each item picked it reads two threads in full: the implementer's, and the reviewer's of the latest round, or of the round the escalation or skip happened in when that is why the item was picked. The page says how many were read and how many the order left out. The cap of three threads read to their first edit is gone. It says cheaper agents may extract trips from threads in bulk, and that the gardener checks each claim against the thread before it counts as evidence.
- [x] **Nothing private on the page.** The procedure says the page and `artifacts/` hold no text copied from a transcript, no thread title, no home path and no personal detail, because they reach public Git. Evidence from a thread is its ID and the trip in the gardener's words: the command, file or refusal, and what the agent did next.
- [x] **Code.** In the focus area the run reads the code itself, not only docs against code: what each module is for, where two parts solve one problem two ways, what has outgrown its design. A proposal about structure names the files and what the change would make easier.
- [x] **Kept.** These are unchanged in meaning: a run changes only upkeep and signs on its own branch; anything that adds or removes a rule, check, gate, interface or default is a proposal; at most three proposals, each one cause with counted evidence; the page is at most 60 lines; the run ends in a session with Sumeet.
- [x] **Not from the checklist alone.** The procedure says a proposal that only restates a line of `readiness.md` goes in the page's Checklist line, and takes one of the three places only when a thread or a number shows what the gap cost.
- [x] **Backlog.** The page gives the open counts as today, and says in one line when a backlog pass looks due and why. The run does not do that pass.
- [x] **The first runs' guesses.** Each gap atelier-h201 listed is closed in `garden.md` or `readiness.md`, or the Report says which change made it moot: where a repository other than atelier finds the Scorecard's measures; whether open counts include the run's own item; the wording for a window with nothing merged and for a first run's Last run column; what a first run with nothing merged reads; what becomes of a bug the run files; where a page names items found stale; a checklist line with mixed results, and whether it is judged on the base; the four lines two runs read differently (2 and 4, 7, 8, 9); and line 3's script reporting home paths, URLs and routes as missing files.
- [x] **Docs.** The Gardener row of the table in `docs/product.md` and the "Who Works With Sumeet" section of `SKILL.md` match the new procedure, including that Sumeet may set the focus at the start.
- [x] **Size.** `garden.md` and `readiness.md` together are no longer than 298 lines, and the Report gives both counts before and after.

## Scope

In: `skills/agent-factory/procedures/garden.md` and `readiness.md`; the "Who Works With Sumeet" section of `SKILL.md`; the Gardener row and the paragraph on the gardener's procedure in `docs/product.md`.

Out: `SKILL.md`'s description and "Your Role" section, which atelier-s47q changes; any script or command; the definition of any Scorecard measure; a schedule for runs.

## Context

The first runs' full lists of guesses are in atelier-613a's Report; atelier-h201 summarised them and was dropped into this item. The design note for the current procedure is `.atelier/missions/atelier-295u/design.md`; its "Numbers first, transcripts second" is what this item reverses, for the reason in this mission's Why. `procedures/groom.md` already shows the pattern of bulk readers with each claim checked.

## Report

A gardener now learns from the top of `garden.md` what a run is for and what it is not, picks one of eight focus areas by a fixed rule, reads six items' threads in full before anything else, and reads the focus area's code. `garden.md` and `readiness.md` are 298 lines together, as before: `garden.md` 188 to 192, `readiness.md` 110 to 106.

### What changed in `garden.md`

- **Purpose** (lines 3 to 11): keeping a repository easy to navigate, change and validate and finding the redesigns that need Sumeet's taste; one page with at most three proposals settled in a session; not a linter, a firefighter or the backlog pass.
- **Areas:** the eight areas the item lists, one line each with what to read.
- **Step 1** defines earlier runs (label `garden`, a `Window:` line in the Report, read with `atelier show`, both `list` forms, atelier-j86z named as not a run) and the focus rule. The Goal records the focus at `atelier new`, and the page's first line, `Focus:`, is committed before any number is taken. The run's Done when lines gain one for reading threads and code with nothing private on the page.
- **Step 2** keeps the window, status times, rounds, take-overs, escalations, skips and open counts, now counting the run's own item. The worked example shrank to one status's sum. The checklist moved into this step.
- **Step 3, Read the Threads:** up to six items, in the order longest in each status, more than one round, escalations, skips; two full threads each (the last implementer, and the reviewer of the latest round or of the round that got it picked); the page's `Threads:` line gives read and left-out counts. Cheaper agents may extract trips; each claim is checked against the thread. The three-threads-to-first-edit cap is gone. The privacy rule is its own paragraph.
- **Step 4, Read the Code and Group by Cause:** the code itself in the focus area; a structure proposal names files and what it would make easier; a proposal that only restates a `readiness.md` line goes in the Checklist line unless a thread or number shows its cost.
- **The page** gains `Focus:`, `Threads:` and `Backlog:` lines (when a backlog pass looks due and why, and items seen stale). Worst Spots became one line to stay within budget.
- **Kept:** upkeep and signs only, on the run's branch; any rule, check, gate, interface or default change is a proposal; at most three proposals, each one cause with counted evidence; 60 lines; the session with Sumeet, including that the merge `status` offers waits for it (atelier-e7iy cites this).

### The first runs' guesses (atelier-h201)

| Gap | Where it is settled |
| --- | --- |
| Where another repository finds the Scorecard's measures | `garden.md` step 2: atelier's `docs/product.md` for every repository, a first run carrying its column of the baseline or "not measured" |
| Whether open counts include the run's own item | Step 2: they do |
| A window with nothing merged; a first run's Last run column | Step 6: "0 items merged", "none merged" in the rows, "first run" |
| What a first run with nothing merged reads | Step 3: the newest merged items before the window; with none in the log, the newest done records and Git history |
| What becomes of a bug the run files | Step 4: a draft on the base, made ready or dropped in the session on Sumeet's word, listed under Fixed and Settled |
| Where a page names items found stale | The page's `Backlog:` line, `Seen stale:` |
| A checklist line with mixed results; judged on the base | `readiness.md`, How to Check: the worst part's result, naming it; judged on the base before the run's fixes |
| Lines 2 and 4: a command only in a file `AGENTS.md` does not link | How to Check: documented means named in `AGENTS.md` or a file it names or links; line 2 now says "documented" |
| Line 7 with no forge | Only the merge checks count |
| Line 8 with no verify skill | Not met |
| Line 9, a cause outside the repository | Met when the bug's body says so; the command may print such items |
| Line 3's script reporting home paths, URLs and routes | `~` paths are excluded, and a name starting with `/` or with a dotted first part counts only if its first part exists here; links starting with `/` or `~` are skipped. Checked on a sample naming `docs/gone.md`, `gone/x.rs`, `~/x/y.md`, `/home/b/z.md`, `/api/items`, `github.com/a/b` and four links: it reported only `docs/gone.md`, `gone/x.rs` and the missing relative link. It prints nothing on this repository. |

### Other files

- `docs/product.md`: the Gardener row says a run brings the redesigns that need Sumeet's taste and looks at one focus area, which he may name at the start, through threads and code. The paragraph on the procedure now names which threads to read.
- `SKILL.md`, "Who Works With Sumeet": a gardener takes its focus from Sumeet when he names one at the start.

### Trimmed to fit the budget

To stay at 298 lines: `readiness.md` line 1 lost its explanation of what Claude Code reads without `CLAUDE.md`, the intro and "A Repository's Own Lines" were shortened (the latter no longer mentions `atelier check` accepting the file), and `garden.md` step 7 no longer says `status` shows how long the run has waited.
