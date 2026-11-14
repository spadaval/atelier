# Garden

Gardening keeps a repository easy to navigate, change and validate as agents
work in it, and finds the redesigns that need Sumeet's taste. A run looks
hard at one area of one repository, through the threads of agents who worked
there and the code itself. It brings Sumeet one page with at most three
proposals, each one cause with counted evidence, and settles them with him
in a session. It changes only upkeep and signs, on its own branch. A run is
not a linter: a checklist line not met is a line of the page, not a
proposal. It is not a firefighter: something broken today is filed as a bug
and left. It is not the backlog pass: it says when one looks due.

## Areas

1. **Structure and design drift:** the modules, what each is for, the docs against the code.
2. **Tests:** what they prove, how long they take, which fail at random (CI history).
3. **Dependencies and toolchain:** lockfiles, pinned versions, build and install steps.
4. **Features and code nobody uses:** commands, flags and paths no test, doc or thread reaches.
5. **How work flows:** step 2's numbers from `atelier log`, and the threads behind the worst.
6. **The host and the agents' setup:** worktrees, hooks, skills, `AGENTS.md`, memory files.
7. **Sumeet's time:** how often and how long work waited on him: `needs-human` and sessions.
8. **Cost:** tokens and tool calls per merged item, and the process share.

## 1. Open the Run

**Earlier runs** are the items labelled `garden`, other than this run, whose
Report has the template's `Window:` line. Read each that `atelier list
--label garden` or its `--done` form lists with `atelier show <id>`, which
reads a started item from its branch, where a waiting run's Report is. A labelled
item with no such line, such as the old digest atelier-j86z, is not a run.

**The focus** is the area Sumeet names; else the first area in the list's
order that no earlier run's `Focus:` line records (a page with no such line
records none); else the area whose newest such line is oldest.

On a branch from `atelier start --backlog garden-<YYYY-MM-DD>`, run `atelier
new "Garden <repo>: <focus>, <YYYY-MM-DD>"` and `atelier set <id> --labels
garden`; the run joins no mission. Its Goal gives the focus and why, its
Scope steps 1 to 7 up to the approval, and these Done when lines:

```markdown
- [ ] The numbers follow garden.md's steps, and `artifacts/numbers.md` holds the log lines behind them.
- [ ] The focus area's threads and code were read as garden.md says, and nothing private reaches the page.
- [ ] At most three proposals, each one cause with its evidence counted; one that adds a rule or check names one considered for removal.
- [ ] Every change on the branch is upkeep or a sign: none adds or removes a rule, check, gate, interface or default.
- [ ] Each finding `atelier list --deferred` showed is settled.
- [ ] The page follows garden.md's template in at most 60 lines, with Session empty.
```

Run `atelier ready <id> --by "<who asked for the run>"`, commit, and
`atelier submit` and `atelier merge` that branch. Then run `atelier start
<id>` in a new worktree of the base, never in the base's checkout. Commit the page's `Focus:` line
before taking any number. Every run takes the numbers and the checklist.

## 2. Take the Numbers and the Checklist

**The window** starts at the end the newest earlier run's `Window:` line
gives; a first run's, at the time on the first line `atelier log` prints. It
ends when you take the numbers: run `date -u +%Y-%m-%dT%H:%M:%SZ` once and
use that time for every number. Log times compare as text.

**The items** are those whose `merge` line is after the start and at or
before the end: `atelier log | awk -v s=<start> -v e=<end> '$4 == "merge" &&
$1 > s && $1 <= e {print $3}'`. Copy all of each one's `atelier log <id>`,
its lines before the window too, into `artifacts/numbers.md`.

- **Time in each status.** A line holding `<from> -> <to>` puts the item in
  `<to>` until the next such line. Sum the seconds in `ready` (waiting to
  start), `building`, `in_review`, `changes_requested` and `approved`; a
  status never reached is 0.
- **Rounds** are its `submit` lines, **take-overs** its `take-over` lines,
  and **skips** its `skip <rule>: <reason>` parts, the rule being the text up
  to the first ` (` or `: `. **Disputes** are its lines in `atelier list
  --disputed`, counted by the outcome it prints.

Give each status's time and the rounds as a median over the items (the mean
of the middle two for an even count), the times in whole minutes rounded from
the median's seconds, halves up, and the rest as totals. Name the item
with the most beside each, comparing seconds; on a tie, the one merged first.
Count, including the run's own item, open items by status (`atelier list |
grep -oE '^ +[^ ]+ \([a-z_]+' | awk '{print $2}' | sort | uniq -c`), blocked
items (`atelier list --blocked`, counted the same way) and unsettled deferred
findings (the `<item>/<finding>:` lines of `atelier list --deferred`).

Every repository's other Scorecard measures are those in atelier's
`docs/product.md`. Carry each from the last page into This run with the
date it was measured and "carried" under Last run, running nothing in
`scripts/scorecard/`; a first run carries its column of atelier's
`docs/architecture/quality/scorecard-baseline-2026-11.md`, dated 2026-11-10,
or "not measured". Then check [readiness.md](readiness.md) as it says, into
`artifacts/checklist.md`.

## 3. Read the Threads

Threads are the main evidence. A trip is where an agent went wrong or wasted
effort: a misread refusal, a rule worked around, a retry loop, files read
again and again before the first edit, a mistake review keeps catching. Pick
up to six items from the window, in this order, skipping one already picked:
those with a dispute; the item longest in each status, in step 2's order;
those with more than one round, most first; those with a skip. With nothing
merged in the window, take the newest merged items before it; with none in
the log, read the newest done records and Git history instead.

For each, read two threads in full, by their IDs as `atelier log <id>` prints
them (under T3 Code, with the thread-read tool): the implementer's, the one
that submitted last after a take-over; and the reviewer's of the latest
round, or of the round the dispute or skip happened in when that is why
it was picked. Cheaper agents may read threads in bulk and extract trips;
check each claim against the thread before it counts as evidence. Read too
the window's fix-now findings, counting once one a later round carries, and
the base's failed CI runs.

**Nothing private on the page.** The page and `artifacts/` reach public Git:
no text copied from a transcript, no thread title, no home path, no personal
detail. Evidence from a thread is its ID and the trip in your words: the
command, file or refusal, and what the agent did next.

## 4. Read the Code and Group by Cause

In the focus area, read the code itself, not only its docs: what each module
is for, where two parts solve one problem two ways, what has outgrown its
design, and half-finished work, such as a migration that reached only some
call sites. Elsewhere, read the code where a trip leads.

Group the trips, the code's problems and the numbers by cause, and bring at
most three causes as proposals. A proposal about structure names the files
and what the change would make easier. One that adds a rule or check names
one you considered removing. One that only restates a line of
`readiness.md` goes in the page's Checklist line, and takes a place only
when a thread or a number shows what the gap cost. A proposal the last page
left open is brought once more, marked carried, and then left out. Leave out
a trip that fits no cause and happened once. Something broken today you file
as a draft on a backlog branch, with `atelier set <id> --labels bug --caused-by
<item or commit>`; the session makes it ready or drops it on Sumeet's word.

## 5. Fix What the Run May Fix

On the run's branch, reviewed with the run, make two kinds of change: upkeep
whose correctness is evident from the diff (dead code, a finished
migration's residue, a doc line the code contradicts, a broken link), and a
fix to a sign an agent reads where it trips (a line of `AGENTS.md`, a doc, a
skill, a script's message). Neither adds nor removes a rule, check, gate,
interface or default; such a change, a restructure, or any change to what
the product does is a proposal.

Settle each finding `atelier list --deferred` shows, on the branch, with
`atelier new "<title>" --from <finding>` or `atelier drop <finding> --reason
"<why>"` (`"fixed in <commit>"` for one the run fixed). An item taken this
way gets its labels and `--caused-by` on a backlog branch after the run merges. A
fix or a settled finding already on an earlier run's unmerged branch is not
made again: the page names that run instead.

## 6. Write the Page

The page is the run item's `## Report`, at most 60 lines without `###
Responses`, in this layout; workings go in `artifacts/`. A window with
nothing merged says "0 items merged" and "none merged" in its rows; a first
run's Last run column says "first run". No line asks for an answer per trip.

```markdown
Focus: <area>, <named by Sumeet | never a focus | last a focus on <date>>.
Window: <start> to <end> UTC, <n> items merged. Workings: `artifacts/numbers.md`.

| Measure | This run | Last run | Baseline |
| --- | --- | --- | --- |
| Waiting to start, building, in review, changes requested, approved: median minutes (worst) | | | |
| Rounds: median (worst); take-overs; disputes by outcome; skips by rule | | | |
| Open items by status; blocked; unsettled deferred findings | | | |
| <each carried measure> | <value>, measured <date> | | |

What means nothing yet: <which numbers, and why>.
Threads: <n> items read, <n> more the order would have picked.
Checklist: <n> met, <n> not met (<lines>), <n> could not test.
Backlog: a pass looks due <now, or when>, because <why>. Seen stale: <items, or none>.
Worst spots: navigate <files the threads read most before their first edit>; change <files the fix-now findings name most>; validate <features with no proof command>.

### Proposals
#### P1 <the change, in a few words>
Cause: <one root cause>. Evidence: <thread ID and trip, or number>, <how often>; ...
Moves: <the number>. Change: <what to do; for structure, which files>. Considered removing: <rule or check>.

### Fixed and Settled
- <commit>: <upkeep or sign>. <finding>: taken into <item>, or dropped: <why>. Bugs filed: <items>. Waiting for the merge to get labels and a cause: <items>.

### Responses

### Session
```

## 7. Submit and Hold the Session

Run `atelier submit` and start the reviewer it prints. Once approved, the run
waits for the session with Sumeet, though `atelier status` offers `atelier
merge`. The session is in whichever thread he talks to; a new thread takes the
run over with `atelier start <id>`. Under Session write, for each proposal,
what he agreed, changed or declined; file each agreed one as an item on a
backlog branch, and merge it. Editing the Report keeps the approval; any other
change needs `atelier submit` again. Then run `atelier merge`.
