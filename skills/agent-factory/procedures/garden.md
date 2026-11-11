# Garden

A run measures one repository, fixes what it may, and brings Sumeet one page
with at most three proposals, which you settle with him in a session. It
changes nothing on another item's branch.

## 1. Open the Run

On the base, `atelier new "Garden <repo>, <YYYY-MM-DD>"` and `atelier set <id>
--labels garden`; the run joins no mission. In the item, give the window's
start (step 2) in the Goal, its Scope in a line (steps 1 to 7 up to the
approval, not the session or merge), and these Done when lines:

```markdown
- [ ] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [ ] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [ ] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [ ] Each finding `atelier list --deferred` showed is settled.
- [ ] The page follows garden.md's template in at most 60 lines, with Session empty.
```

Commit the draft with `git commit`, then `atelier ready <id> --by "<who asked
for the run>"` and `git push`. Run `atelier start <id>` in a worktree of the
base made for the run, never in the checkout that has the base, which `start`
would switch to the run's branch.

## 2. Take the Numbers

**The window.** It starts at the end the last run's page gives: the newest
item labelled `garden`, merged or still waiting for its session (`atelier
list --label garden`, `atelier list --done --label garden`). A first run starts
at the time on the first line `atelier log` prints. It ends when you take the
numbers: run `date -u +%Y-%m-%dT%H:%M:%SZ` once and use that time for every
number. Log times compare as text.

**The items.** An item is the run's when its `merge` line is after the start
and at or before the end: `atelier log | awk -v s=<start> -v e=<end> '$4 ==
"merge" && $1 > s && $1 <= e {print $3}'`. For each, read all of `atelier log
<id>`, its lines before the window too, and copy them into
`.atelier/items/<id>/artifacts/numbers.md`.

- **Time in each status.** A line holding `<from> -> <to>` puts the item in
  `<to>` at its time, until the next such line. Sum the seconds in `ready`
  (waiting to start), `building`, `in_review`, `changes_requested` and
  `approved`, then give whole minutes, halves rounded up. A status never
  reached is 0.
- **Rounds** are its `submit` lines, **take-overs** its `take-over` lines and
  **escalations** its `escalate` lines.
- **Skips** are its `skip <rule>: <reason>` parts. The rule is the text after
  `skip ` up to the first ` (` or `: `.

Across the items, give each status's time and the rounds as a median, the
mean of the middle two when the count is even, and take-overs, escalations
and each rule's skips as a total. Name the item with the most beside each,
comparing seconds, not minutes; on a tie, the one merged first.

From `atelier list`, count open items by status (`atelier list | grep -oE '^
+[^ ]+ \([a-z_]+' | awk '{print $2}' | sort | uniq -c`), blocked items
(`atelier list --blocked`, counted the same way) and unsettled deferred
findings (the `<item>/<finding>:` lines of `atelier list --deferred`).

Carry every other Scorecard measure in `docs/product.md` from the last page,
with the date it was measured. Take no snapshot and run nothing in
`scripts/scorecard/`. Compare with the last run's page. A first run compares
with the November 2026 baseline where it covers the repository, and carries
its measures dated 2026-11-10:
`docs/architecture/quality/scorecard-baseline-2026-11.md` in atelier.

Worked example, from atelier's log. atelier-ivbq's lines read `15:43:38
ready draft -> ready`, `15:43:57 start ready -> building`, `15:56:52 submit
building -> in_review`, `16:00:25 verdict ... in_review -> changes_requested`,
`16:03:26 submit changes_requested -> in_review`, `16:05:39 verdict ...
in_review -> approved` and `16:06:11 merge approved -> done`. Waiting is 19 s
(0 minutes), building 12:55 (13), in review 3:33 + 2:13 (6), changes
requested 3:01 (3), approved 0:32 (1), and rounds 2. A first run whose
numbers were taken at 2026-11-11T16:30:00Z counts atelier-2mlh and
atelier-ivbq. atelier-6w8d merged at 16:47:06 while that run waited for its
session, so the next run, whose window starts at 16:30:00Z, counts it.

## 3. Run the Checklist

Check [readiness.md](readiness.md) as it says, and record each line as met,
not met or could not test in `artifacts/checklist.md`.

## 4. Read for Causes

Read the window's fix-now findings, counting once one a later round carries,
escalations and skips, the base's failed CI runs, and the code against its
docs: a doc the code contradicts, and half-finished work, such as old and new
paths with no clear boundary, a migration that reached only some call sites,
tests that keep a stale assumption, or a done item's cleanup with no owner.

Read transcripts last. Read a thread only to explain a number that moved since
the last page, or an item named worst beside one, by its ID exactly as the log
prints it; under T3 Code, with the thread-read tool. For navigation, read at
most three implementer threads, those of the items longest in `building`, up
to their first edit, and list the files each read.

A trip is where an agent went wrong or wasted effort: a misread refusal, a
rule worked around, a retry loop, a mistake review keeps catching. Group the
trips and the numbers by cause before you write anything, and bring at most
three causes as proposals. A proposal that adds a rule or check names one
you considered removing. Leave out a trip that fits no cause and happened
once. Something broken today, such as a command that fails or an error that
hides its reason, you file on the base as an item, with `atelier set <id>
--labels bug --caused-by <item or commit>`, and it takes no room on the page.

The worst spots, one or two under each property:

- **Easy to navigate:** the files your three threads read most before their
  first edit, said to be a sample. With no item merged in the window, the
  checklist's navigation lines instead.
- **Easy and reliable to change:** the files the window's fix-now findings
  name most. Carry the rework share with its date and say rework by file is
  not measured; never rank files by how much they changed.
- **Easy to validate:** features the verify skill gives no proof command for,
  and the window's bug items with their `caused_by`.

Where a property cannot be shown yet, say why in one line.

## 5. Fix What the Run May Fix

On the run's branch, reviewed with the run, make two kinds of change: upkeep
whose correctness is evident from the diff (dead code, a finished
migration's residue, a doc line the code contradicts, a broken link), and a
fix to a sign an agent reads where it trips (a line of `AGENTS.md`, a doc, a
skill, a script's message). Neither adds nor removes a rule, check, gate,
interface or default. A new or removed rule or check, a changed default, a
restructure, or any change to what the product does is a proposal.

Settle each finding `atelier list --deferred` shows, on the branch: `atelier
new "<title>" --from <finding>` to take it into an item, or `atelier drop
<finding> --reason "<why>"`, with `"fixed in <commit>"` for one the run fixed.
`atelier set` runs only on the base, so an item taken this way gets its
labels and `--caused-by` there after the run merges.

## 6. Write the Page

The page is the run item's `## Report`, at most 60 lines, in this layout, not
counting `### Responses`, which `atelier submit` needs after a review.
Workings go in `artifacts/`, and the page names the file.

```markdown
Window: <start> to <end> UTC, <n> items merged. Workings: `artifacts/numbers.md`.

| Measure | This run | Last run | Baseline |
| --- | --- | --- | --- |
| Waiting to start, building, in review, changes requested, approved: median minutes (worst) | | | |
| Rounds: median (worst); take-overs; escalations; skips by rule | | | |
| Open items by status; blocked; unsettled deferred findings | | | |
| <each carried measure> | <value>, measured <date> | | |

What means nothing yet: <which numbers, and why>.

Checklist: <n> met, <n> not met (<lines>), <n> could not test.

### Worst Spots
- Navigate: ...
- Change: ...
- Validate: ...

### Proposals
#### P1 <the change, in a few words>
Cause: <one root cause>. Evidence: <trip or number>, <how often>; ...
Moves: <the number>. Change: <what to do>. Considered removing: <rule or check>.

### Fixed and Settled
- <commit>: <upkeep or sign>. <finding>: taken into <item>, or dropped: <why>. Items waiting for the merge to get labels and a cause: <items>.

### Responses

### Session
```

A proposal the last page left open is brought once more, as one of the
three, marked carried; after that it is left out. No line asks for an answer
per trip.

## 7. Submit and Hold the Session

Run `atelier submit` and start the reviewer it prints. Once approved, the run
waits for Sumeet; `atelier status` shows since when, and the `atelier merge`
it offers waits for the session. The session is with him, in whichever thread
he talks to; a new thread takes the run over with `atelier start <id>`. Under
Session write, for each proposal, what he agreed, changed or declined. File
each agreed one as an item on the base, commit it and push. Editing the Report
keeps the approval; any other change needs `atelier submit` again. Then run
`atelier merge`.
