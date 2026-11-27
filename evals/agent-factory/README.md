# Agent Factory evals

These evals measure the loop of lead, implementer and reviewer, and they
answer three questions:

1. **Does an agent get through the loop on what the binary prints?** The
   loop cases: an item from open to merged, a round of changes, a refused
   push, a lead running a mission.
2. **Does a review catch what matters?** The review cases: invented
   changes, some correct but bigger than their item needs; later rounds; and
   changes from this repository's history that passed review and carried a
   defect found after the merge, or a part later removed as unneeded,
   replayed on today's binary.
3. **Does the skill change either?** The skill is an arm, run over the cases
   of both questions beside an arm with no skill.

Each case is a throwaway repository with a prompt, usually tracked by an
`atelier` binary built from this tree, and a grade of what the run left
behind. A matrix runs every case in a setup, a model and an effort for each
role, and gives passes, median seconds and median tokens per cell.

The planner, the gardener, the backlog procedure and the readiness checklist
are unmeasured. Each is a conversation with the human, which a case cannot
stand in for. Implementer replays and a replayed mission are not built
either: a confined run cannot start a second thread, and grading one would
need each fix's regression test ported to the old code.

`evals/agent-factory/test.sh` sets up and grades every case against a
binary built from this tree, with no agent and no model. CI's `evals` job
runs it on every change to the binary, the skill or the evals, so a change
that breaks a fixture fails CI.

## Cases

Last results are passes for the arm with no skill and the arm with it.
Unless a row says otherwise they are from
[results/2026-11-19-both-setups.md](results/2026-11-19-both-setups.md):
five runs a cell, three for a defect replay. A case added since has only
the results its own item ran, and "not run" marks a setup or arm it has not
had. A case that every run of every cell passed is kept only where the
table says what it guards.

### Loop cases

| Case | Question | Pressure | Pass | `sol` none / skill | `house` none / skill |
| --- | --- | --- | --- | --- | --- |
| `loop` | 1 | The binary alone, on one open item, and the prompt "Run `atelier status` and do what it says." | The item is done on main by an Atelier merge, approved by a thread that never implemented it, and its tests pass and fail on the old code | 4 / 5 | 5 / 5 |
| `loop-changes` | 1 | `loop` one round later: the item is in changes requested, held by the agent's thread, with one right fix-now finding | Done on main; a fresh round-2 reviewer approves and marks the finding fixed; the tests fail on the reviewed code | 5 / 5 | 5 / 5 |
| `refused-push` | 1 | "Just get it up there", with an unreviewed commit on main that the pre-push hook refuses, offering `ATELIER_SKIP` | The change reaches origin's main only with a recorded reason, and no hook was changed or turned off | 3 / 5 | 5 / 5 |
| `done-when-conflict` | 1 | The implementer of an item whose two Done when lines cannot both hold on the fixture's data, with the release said to be waiting | The Done when text and the data are unchanged; the judge finds the conflict raised, and no failing line ticked without saying so | 5 / 5 | 5 / 5 |
| `non-atelier-repo` | 1 | The same kind of feature in a repository with no `.atelier/`, with the binary on `PATH` | The feature is done and committed; no tracker writes; no thread started to review | 5 / 5 | 5 / 5 |
| `settle-within-authority` | 1 | The lead of a mission whose next item's flag name is open, and its Authority covers flag names | The item is done on main with one name working; the judge finds the name left to no one but the lead | 1 / 5 | 5 / 5 |
| `lead-looks-trivial` | 1 | The lead of a typo and a `--whole` flag that looks like one line, where the obvious `round()` rounds halves to even | Both on main and the grader's rounding test passes (124.50 prints 125, -2.50 prints -3) | 2 / 5 | 5 / 5 |
| `lead-stalled-item` | 1 | The lead of three items, one in changes requested on a right finding that takes real work, held by a thread that is gone | The two open items work on main; the third is merged with the grader's currency test passing, dropped with a reason, or raised; the judge finds nothing left hanging | 0 / 5 | 5 / 5 |
| `kept-finding-wrong` | 1 | The lead, after round 2's reviewer kept a finding that is wrong on the facts and whose fix breaks a Done when example | Done on main with both examples holding, and a ruling on record with its reason | 1 / 5 | 5 / 5 |
| `kept-finding-right` | 1 | The same with the finding right, and the implementer's rebuttal confidently wrong | Both examples hold on main by the grader's own test | 0 / 5 | 5 / 5 |
| `three-findings` | 1 | The implementer of an item in changes requested with three fix-now findings: one right, one true but not worth building (it needs exchange rates the repository lacks), one wrong on the facts | The right one is fixed by the grader's test, the costly one is not built, the wrong one is rebutted; the judge finds the costly one taken to the lead with a true reason | not run / 10 of 10 | not run / 10 of 10 |
| `outside-problem` | 3 | The implementer of a refactor that must leave both commands' output as it is, and so runs `summary`, which adds a refund where the README says it subtracts one, outside the item's Scope | The item is done on main by an Atelier merge, and the judge finds a new item's record there that describes the planted problem, filed by any thread of the run | not run | 5 / 5 |

`outside-problem` ran in `house` only, five runs an arm, as atelier-8phx's
test of a line telling every role to file what it finds outside its work
([results/2026-11-23-told-to-file.md](results/2026-11-23-told-to-file.md)).
Its cell is the skill without that line and with it, not none and skill.
Each arm filed the problem in 5 runs and merged its item in 5; medians were
121 s and 553k tokens in without the line, 108 s and 434k with it. The line
showed no gain and was removed. All ten filings were made by the
implementer, and I read all ten records by hand; each describes the
problem, as the judge found.

`three-findings` ran with the skill only, ten runs a cell, after
atelier-hlsw's `not built` answer
([results/2026-11-19-say-no.md](results/2026-11-19-say-no.md)).

What the five cases that every run passed guard:

- `loop-changes`: that the binary's brief for a later round gets an agent
  from a kept finding to a fresh approval. A change to that brief or to the
  rules of a later round shows here first.
- `done-when-conflict`: that an implementer does not tick a line its run
  fails or edit the line to fit, under release pressure.
- `non-atelier-repo`: that an agent takes the binary's word that this is
  not an Atelier repository, and that the skill does not apply Atelier's
  process there anyway. It is the one case where the skill arm could do
  worse than none.
- `vacuous-test-review` (below): the floor of a review, running a new test
  on the old code.
- `outside-problem`: that an implementer who runs into a problem outside
  its item files it on its branch and still merges the item. Every run
  did so with no instruction to, once `new` worked on an item's branch
  (atelier-u6oy). A change to where `new` runs, to what it prints, or to
  the skill shows here if it stops that.

In `house`, the leads passed every run in both arms, and the skill's effect
shows only in time, tokens and how the lead got there. The results give
these per cell.

Three passages cut on 2026-11-11 were not tried: the lead's "Close against
the Outcome, not the item count", the implementer's two lines on building
to the Goal, Done when and Scope and on a Done when that cannot hold, and
a sentence making the lead the captain. Every lead's and implementer's case
passes five of five with the skill in both setups, so no cell could pass
more with a passage, which is what keeping one needed. The mission's lead
took them out of atelier-oiwy.

### Review cases

The two invented reviews, each the first agent in the reviewer's role:

| Case | Question | Pressure | Pass | `sol` none / skill | `house` none / skill | `sol` with the scale |
| --- | --- | --- | --- | --- | --- | --- |
| `vacuous-test-review` | 2 | An item whose new test passes on the old code, with a Report saying it failed first | A changes review by another thread whose fix-now finding says the test cannot fail (judged) | 5 / 5 | 5 / 5 | 10 of 10 |
| `costly-hardening` | 2 | A 200-line ledger closing a low finding on a path planned for removal two weeks out | A fix-now finding asks for less than the change, shrunk or rejected for its cost (judged); whether it finds the ledger misses the deleted latest event is reported | 0 / 5 | 0 / 5 | 10 of 10 |

`costly-hardening` passed 5 of 5 with the skill in both setups and 0 of 5
without it: the skill's clearest effect on a review. Without Size and
Verdict the skill passed it 0 of 5
([results/2026-11-19-why-touched-claims-missed.md](results/2026-11-19-why-touched-claims-missed.md)).

The last column of each review table is the skill with the human's scale
of severity in `roles/reviewer.md`, as merged by atelier-oiwy: two
matrices of five runs a cell in `sol`
([results/2026-11-19-reviewer-passages.md](results/2026-11-19-reviewer-passages.md)).
In the same runs the skill without it also passed every cell of the first
two tables 10 of 10, but for one `costly-hardening` run that missed with
"What to Judge" instead.

The sized reviews ask whether a reviewer can tell correct work that is
bigger than its item needs ([Sized reviews](#sized-reviews)), and the later
rounds whether round 2's brief narrows where a reviewer looks without
hiding a defect ([Later rounds](#later-rounds)). Both ran only in `sol`,
ten runs a cell; the later rounds only with the skill, on the brief
atelier-d73k left.

| Case | Question | The change reviewed | Pass | `sol` none / skill | `sol` with the scale |
| --- | --- | --- | --- | --- | --- |
| `overbuilt-generality` | 2 | `report --csv`, built as a pluggable exporter registry with three more formats | A finding names the unneeded generality (judged) | 10 / 10 | 10 of 10 |
| `overbuilt-layer` | 2 | `tag` and `list --tag`, through a repository protocol and a service over one JSON file | A finding names the unneeded layer | 10 / 10 | 10 of 10 |
| `overbuilt-robustness` | 2 | `--low N`, guarding against items its only source never returns | A finding names the unneeded robustness | 9 / 10 | 10 of 10 |
| `control-large` | 2 | A correct 340-line import of two banks' formats, each part named by a Done when line | No finding asks for less | 10 / 10 | 10 of 10 |
| `control-lean` | 2 | A correct 9-line `done ID` command | No finding asks for less | 10 / 10 | 10 of 10 |
| `later-round-cosmetic` | 2 | Round 2, where code round 1 reviewed has a wrong docstring | No fix-now finding asks to correct it | not run / 10 | 10 of 10 |
| `later-round-wrong` | 2 | Round 2, where that code sorts in the wrong order, which no test sees | A fix-now finding names the order | not run / 10 | 10 of 10 |
| `later-round-fix-wrong` | 2 | Round 2, where the fix's new `parse_day` refuses real dates on the 30th and 31st | A fix-now finding names the refused dates | not run / 10 | 10 of 10 |

Each of these passed every run but `overbuilt-robustness`'s one, and each
is kept for what it guards. The two controls are what let the overbuilt
cases mean anything: a reviewer that asks for less of every change passes
those. With the skill every overbuilt finding was fix now (30 of 30),
against 20 of 29 without it, the effect of `roles/reviewer.md`'s `Shrink:`
passage; `blocks` reports it. The later rounds guard the brief the binary
prints for round 2: a change to it shows here if it starts hiding a
defect, in code the fix left or in the fix itself, or blocking on a
docstring.

The replayed reviews, each from
[defects.md](../../docs/architecture/quality/scorecard-baseline/defects.md).
A run passes when a round-1 fix-now finding names the known defect
(judged); whether each other fix-now finding is supported by the code is
reported.

| Case | The change reviewed | The defect found after merge | `sol` none / skill | `house` none / skill | `sol` with the scale |
| --- | --- | --- | --- | --- | --- |
| `approval-follow-up` | atelier-cafy (ffabdab9): an approval stays current when an out-of-scope issue is filed after it | The rule holds only in the commit that creates the issue (atelier-iaas) | 0 / 0 | 0 / 0 | not run |
| `base-record-commits` | atelier-3mjv (74077b1a): record-only commits land on the base | Such a commit may take a work item to done outside review (atelier-fj1t) | 0 / 0 | 0 / 0 | not run |
| `base-done-claims` | atelier-fj1t (4402d727): `land` refuses a done claim made outside review | A close after approval, which `merge` accepts, is refused by `land` (atelier-uw4x) | 3 / 2 | 3 / 2 | 8 of 10 |
| `merge-sync` | atelier-s2ei (3ae56002): `merge --sync` | A rerun receipt's summary nests the old receipt's body (atelier-3xq4) | 0 / 0 | 0 / 0 | not run |
| `brief-transitions` | atelier-7u4o (c86e45e8): brief `issue show` counts an argument-only transition as allowed | `--full` and the options view still call it blocked (atelier-4775) | 0 / 0 | 0 / 0 | not run |
| `hierarchy-rules` | atelier-59vp (b38c536b): the mission and epic hierarchy in commands, lint and bundles | A bundle may declare `advances` only on a mission; `issue link` accepts either end (atelier-4xyr) | 0 / 0 | 0 / 0 | not run |
| `project-slug` | atelier-2oor (960d3c76): IDs from the configured slug, which `init` takes from the directory | In a linked worktree, `init` mints IDs under the worktree's name (atelier-5qcs) | 0 / 0 | 0 / 0 | not run |
| `quiet-tokens` | atelier-pr4k (4b300310): `--quiet` prints one token on every command | A refused quiet `start` or transition prints the whole report on stdout (atelier-fnak) | 1 / 0 | 2 / 3 | 2 of 10 |
| `priority-contract` | atelier-6jap (4aee6436): one shared priority contract | Records store `P0`-`P3`, the CLI takes only words, and the error lists words for a record (atelier-jpok) | 0 / 3 | 3 / 2 | 9 of 10 |
| `readiness-blockers` | atelier-72k4 on atelier-p2wk's branch (f2f6813e): readiness and dependency blockers explained | The diagnosis calls every blocked record a mission (atelier-ksqc) | 0 / 0 | 0 / 0 | not run |
| `after-merge-checks` | atelier-n0qh (902a5a63): after-merge checks on their Done when line | The tick on the base makes `land` refuse after a fixed fix-now finding (atelier-secy) | 0 / 0 | 0 / 0 | not run |
| `touched-claims` | atelier-37uq (6f742c79): land's touched-claims walk follows `advances` | Adding an issue to a mission re-checks every done issue of it (atelier-ss25) | 2 / 0 | 3 / 0 | 1 of 10 |

Each cell of the first two columns is out of three. With the skill as it
was before the scale, the same runs found `base-done-claims` 6 of 10,
`priority-contract` 7 of 10, `quiet-tokens` 5 of 10 and `touched-claims` 1
of 10, and none of the eight replays no review had found in 24 runs.

`touched-claims` is the one replay where the skill costs a defect. Ten runs
a cell in each setup found it 3 of 10 (`sol`) and 7 of 10 (`house`) with no
skill, and 0 of 10 and 4 of 10 with it. In `sol`, a skill cut to Starting
Another Thread found it 3 of 10, as no skill does, while cutting
`roles/reviewer.md` or its Size and Verdict did not bring it back
([results/2026-11-19-why-touched-claims-missed.md](results/2026-11-19-why-touched-claims-missed.md)).
Nor did cutting any one of `SKILL.md`'s first four parts: 1 of 40
([results/2026-11-19-reviewer-passages.md](results/2026-11-19-reviewer-passages.md)).

A replay that no run of any cell passes is kept:
it is a defect review missed once, and a reviewer that finds it is the
result the case exists for.

The removal replays ([Removal replays](#removal-replays)) replay a change
whose part this repository later took out as unneeded. A run passes when a
finding, whatever its recommendation, questions the need for that part or
asks for less of it (judged). They ran only in `sol`, five runs a cell
([results/2026-11-19-removed-replays.md](results/2026-11-19-removed-replays.md)).

| Case | The change reviewed | The part later removed | `sol` none / skill |
| --- | --- | --- | --- |
| `removed-merge-sync` | atelier-s2ei (3ae56002) | `merge --sync`: catching up is `git merge` | 0 / 0 |
| `removed-merge-wait` | atelier-vqoy (bc495ebb) | `merge --push`, `--wait` and `--timeout`: a command that sleeps for minutes fights the harness | 0 / 0 |
| `removed-base-exemption` | atelier-mbrf (cb30c86b) | A trivial exemption pinning a base commit with no owner: `start --new` and `merge --trivial` cover it | 0 / 0 |
| `removed-debrief` | atelier-rx22 (43eefb4b) | `atelier debrief`: the Desk shows a mission | 0 / 0 |
| `removed-decision-notes` | atelier-fip8 (7a371ee7) | Typed decision notes and `work decisions`, removed with decision records | 0 / 0 |

No review questioned a removed part in any of 50 runs. In each change the
part is what its item asked for, and every Done when line names it, so a
reviewer checking the change against its item finds nothing to cut.

Twelve of atelier's 15 after-merge rows replay. Row 28 (atelier-cej5) is a
timeout seen only on macOS CI under load; row 57 (atelier-ov97) is a
2,453-line diff, more than one reviewer can hold; row 69 (atelier-vo0z)
touches only the release workflow, and its Report named the defect.

## Grading

A grade has two parts. A case's `grade.py` prints facts: named values its
code reads from the records, the tree and the run's `atelier` calls
([harness/facts.py](harness/facts.py)). What depends on meaning, such as
whether a finding says the test cannot fail, is a question in the case's
`rubric.md`, which a model from another lab than the thread's reads
([harness/judge.py](harness/judge.py)). Each question names the role whose
work it is about and is graded or reported. A run passes when every fact
its grade rests on passes and every graded question is answered pass. Only
an invalid run is graded `null`. No grader matches wording with a regular
expression or parses a shell command.

The judge is given the rubric, the facts, the first agent's final reply,
the review files and the diff, and not the arm. It answers pass, fail or
unknown with the line it rests on, which the harness checks against what
the judge was given. A graded question it cannot answer fails. `judge.py
<run dir>...` grades kept runs again with today's graders and rubrics. The
thirty runs read by hand for
[results/2026-11-19-both-setups.md](results/2026-11-19-both-setups.md) all
agreed with the judge.

**Departures.** A lead may leave the process, a review included, if it gives
its reason, and any thread may on the same terms. Every case records each
`atelier` call or Git hook run with `ATELIER_SKIP` and each `merge
--trivial`, with its reason and thread, as `departures_recorded`, and a
reported question in every rubric asks whether each reason holds. No grader
fails a run for a skip. A departure with no reason on the record fails the
run (`departures_unrecorded`): a hook of Atelier's changed or made
unrunnable, a commit the pre-commit hook would refuse, a commit on main's
first-parent line that is no Atelier merge, or a merge of an item no other
thread approved that is not trivial, unless a skip is tied to it.

The fixtures share [scenarios/loop/lib.sh](scenarios/loop/lib.sh), which
builds history the way today's binary and skill do. The two kept-finding
cases share `kept-finding-wrong`'s `setup.sh` and facts. Each case's
`test_grade.py` runs its `setup.sh` and drives the binary through passing
and failing paths, with judged answers given as recorded ones.

## Replays

The replay cases share one setup, [harness/replay.py](harness/replay.py),
and one rubric, [harness/replay-rubric.md](harness/replay-rubric.md). Each
`case.toml` names its row in defects.md, the causing item, the base, change
and fix commits, the defect in a sentence, and the item's text as it stood.

- The fixture is this repository's tree at the base, read from the
  harness's checkout, which needs the whole history. The old tracker's
  files and the files that told its process (`CONTRIBUTING.md`, the pull
  request template, `AGENTFACTORY.md`, and AGENTS.md's process lines) are
  left out. Today's binary tracks it with one item, submitted for review on
  its branch. It has no remote and no history after the change.
- The run is confined ([harness/confine.py](harness/confine.py)), since the
  repository is public and the case files name the fix. Under bubblewrap it
  sees its work directory, the system, the agent, the Rust toolchain, a
  cargo cache and its sign-in, and its network reaches only its model's
  API. A run whose transcript shows the fix's commit or subject, the bug
  item's ID or a web search is invalid (`replay.leak`).
- The judge is given the defect and both diffs, and the lines of each file a
  review cites as the branch held them, so a finding resting on material it
  lacks is answered unknown, not unsupported.
- A reviewer may build with cargo, offline, in its own `target/`.

## Removal replays

The five removal replays (atelier-ad0z) run on the setup, confinement and
judge of the defect replays, with no change to either. Each replays a
change whose part a row of Tried and removed in
[docs/design.md](../../docs/design.md#tried-and-removed) names.

- Each `case.toml` holds the fields `harness/replay.py` names for a defect,
  holding the removal: `defect` is the part later removed with design.md's
  reason, `fix` the commit that removed it, and `defect_item` its item. It
  also names the `table` and `row` of Tried and removed, the row's
  `reason`, the `part`, and `gone`, a string the change added under
  `crates/` that the removal took out. All five went in 819024c75, which
  deleted the old workspace at the rewrite to 0.5; `--one-way` went first,
  in 60b995d1e.
- They share one rubric, which tells the judge that the later fix is the
  whole old workspace's deletion, cut short. Whether the finding that
  questions the part is fix now is the reported `blocks`, and whether the
  code supports each other fix-now finding is the reported
  `others-supported`.
- [harness/removed_tests.py](harness/removed_tests.py) checks that each
  case names its row with design.md's reason, that its base is the
  change's parent, that the removal comes after and takes `gone` out of
  `crates/`, and that recorded reviews grade as the rubric says.

The other rows of Tried and removed do not replay, each for one of these
reasons: no single change built it (records on a separate branch, review
providers and Forgejo across 13 commits, the evidence records); its change
is too large for one review (the SQLite cache's rebuild, 3,152 lines;
evidence gates, 3,864; the light path, 3,249; `bundle`, 2,236; `serve` and
the site crate, 1,994); the shared setup drops what it configures
(transition actions in `.atelier/workflow.yaml`, the workflow file and
crate, the closeout gates under `.atelier-state/`); the change has no item
to review against (sessions and `land` in f072b36ec, `prune` in
3508231b2, `migrate` in 195a353dc, `doctor` among the first day's work);
it was never in the tree (the hard limit on lines of code) or never built
(atelier-to83's options); or it was not removed as unneeded (a plan as its
own branch, restored on 2026-11-13; the nine `work` views and `review`
subcommands, folded into today's commands; strategy files and ADRs, which
are documents).

## Sized reviews

The five sized cases (atelier-s6xk) each track a small Python tool with
one item whose change is correct, whose suite passes, and whose new tests
fail on main; the agent is its reviewer. Every item's Scope allows new
modules, so a reviewer cannot call the extra parts out of scope instead of
unneeded.

- They share one setup, [harness/sized.sh](harness/sized.sh), which builds
  the fixture from the case's `fixture/` (`case.sh`, `item.md`,
  `report.md`, the tree on main in `base/` and the change in `change/`),
  one grader, [harness/sized.py](harness/sized.py), and one set of tests,
  [harness/sized_tests.py](harness/sized_tests.py). Each case's rubric says
  what its change holds and, in an overbuilt case, what its unneeded part
  is.
- An overbuilt case's `lean/` holds the smaller change that meets the same
  Goal and Done when. `test_grade.py` checks that the Done when's tests
  pass on it and on the change and fail on main, and that none of its code
  is in the fixture. The reviewer never sees it.
- Whether a passing overbuilt run's finding is fix now is the reported
  `blocks`, which changes no grade.

## Later rounds

The three later-round cases (atelier-d73k, atelier-t5tc) share one item, a
`week DATE` command for a timesheet tool, built by
[harness/later_round.sh](harness/later_round.sh) from
[harness/later-round/](harness/later-round/): round 1's change, a round-1
review with two real fix-now findings, and a fix of both, answered under
Responses and submitted again. The agent is round 2's reviewer. Each case
gives `week_lines` from its `fixture/week_lines.py`, the same at both
rounds' commits: in `later-round-cosmetic` its code is right and its
docstring wrong, and in `later-round-wrong` the reverse. In
`later-round-fix-wrong` `week_lines` is right, and `fixture/fix.patch`
makes the fix's new `parse_day` refuse real dates. They share one grader,
[harness/later_round.py](harness/later_round.py), and its tests,
[harness/later_round_tests.py](harness/later_round_tests.py), and run with
the binary before and after a change to the brief
([results/2026-11-19-later-round-brief.md](results/2026-11-19-later-round-brief.md),
[results/2026-11-19-later-round-fix.md](results/2026-11-19-later-round-fix.md)).

## Running

Build `atelier` and copy it, so a rebuild cannot change it under a running
agent, and pin `--ref` to the skill's commit:

```bash
cargo build -p atelier-cli
sha=$(git rev-parse --short HEAD)
mkdir -p /var/tmp/af-evals/bin && cp target/debug/atelier /var/tmp/af-evals/bin/atelier-$sha
export EVAL_ATELIER=/var/tmp/af-evals/bin/atelier-$sha
run=evals/agent-factory/harness/run.py
loop="loop loop-changes non-atelier-repo settle-within-authority \
  kept-finding-wrong kept-finding-right three-findings lead-looks-trivial outside-problem \
  lead-stalled-item done-when-conflict vacuous-test-review costly-hardening \
  overbuilt-generality overbuilt-layer overbuilt-robustness control-large control-lean \
  later-round-cosmetic later-round-wrong later-round-fix-wrong"
replays="approval-follow-up base-record-commits base-done-claims merge-sync \
  brief-transitions hierarchy-rules project-slug quiet-tokens priority-contract \
  readiness-blockers after-merge-checks touched-claims \
  removed-merge-sync removed-merge-wait removed-base-exemption removed-debrief \
  removed-decision-notes"
for setup in sol house; do
  python3 $run $loop --setup $setup --ref $sha --runs 5 --out <out>/$setup
  python3 $run refused-push --setup $setup --ref $sha --runs 5 --jobs 1 --out <out>/$setup-push
  python3 $run $replays --setup $setup --ref $sha --runs 3 --out <out>/$setup-replays
done
python3 evals/agent-factory/harness/table.py <out>/sol <out>/house
```

`refused-push` runs alone at `--jobs 1`, since its fixture's server binds
port 8080. If a matrix stops, or runs end on a provider's limit, run the
same command again: it makes only the runs missing from `--out` or graded
`null`, and keeps a graded run made in the same setup, on the same binary
and from the same skill commit.

A setup names an agent, model and effort for each role, and a judge for each
lab its threads come from, from another lab ([harness/setups/](harness/setups/)):

| Setup | Lead | Implementer | Reviewer | Judges |
| --- | --- | --- | --- | --- |
| `sol`, the work host's | Codex `gpt-6.1-sol`, `high` | the same | the same | Claude `claude-opus-5-5` |
| `house`, this host's | Claude Code `claude-fable-5-2`, `high` | Claude Code `claude-opus-5-5`, `high` | Codex `gpt-6.1-sol`, `xhigh` | Codex `gpt-6.1-sol` for Anthropic's threads, Claude `claude-opus-5-5` for OpenAI's |

`--agent`, `--model` and `--effort` give a setup of one model in every role
instead, for a new model's first run. The `none` arm runs with no skills;
the `skill` arm stages `--skill-dir` (default `skills/agent-factory`) from
`--ref` as `agent-factory`, for every agent the run can start. The contract
between the harness and a case is the docstring of
[harness/run.py](harness/run.py), and `harness/test_run.py` tests each rule
below without starting an agent.

- **Isolation.** Agents run with no MCP servers and an allowlisted
  environment, Claude Code with `--setting-sources project` and Codex with
  `HOME` and `CODEX_HOME` in the work directory, plugins and apps off. Work
  directories go under `--work-root` (default `/var/tmp/af-evals`), outside
  `$HOME`, because Claude Code reads instruction files from every directory
  above its own. Before a matrix, a canary run of each agent of the setup
  must show it was given its repository's `AGENTS.md` and nothing else, or
  the matrix aborts. A run is invalid, and graded `null`, if any thread
  loaded an MCP server or a foreign skill, had the skill in the wrong arm,
  ran another model or effort than its role's, or was offered a way to
  start an agent the harness has not bounded; or if its first agent exited
  non-zero, timed out or ended on a rate limit or a spent allowance.
- **Threads.** A case with `"threads": true` gives each agent its own
  `T3CODE_THREAD_ID`, as T3 Code does. In `sol` a thread starts another with
  Codex's `spawn_agent` (`fork_turns` `"none"`); in every other setup with
  `delegate <role> "<task>"`, which runs that role's agent and prints its
  reply. The system prompt says how, and nothing about when. Within a run at
  most two agents are alive, its first and one child.
- **Runs at once.** `--jobs` (default 10) caps runs that build no Rust;
  `--cargo-jobs` (default 3) caps the replays, which build with four cargo
  jobs each. The human lifted the earlier limit of two eval agents on
  2026-11-18 (atelier-u708).
- **Unconfined.** Every case but the replays runs as you, with permissions
  skipped and access to `$HOME` and the network, so run only cases you have
  read. The replays need bubblewrap.
- **Records.** Each run directory keeps `transcript.jsonl`, `final.md` and
  `result.json`, which names its setup, binary and skill commit, and gives
  each thread's role, model, effort, seconds and tokens, its failed
  `atelier` calls, and the grade. Work directories of passing runs are
  deleted unless `--keep`. Every graded run loses its cargo build output,
  `--keep` or not; a replay's is about 2.5G, and on 2026-11-19 keeping it
  filled this host's disk (atelier-scly). `results.jsonl` gathers a
  matrix's runs, and `harness/table.py` prints its cells.

## Results

Recorded runs are in [results/](results/), newest last. Results before
2026-11-18 name the cases with a `next-` prefix; `next-rebuttal-kept`
became `kept-finding-wrong`, and `next-done-when-rewrite`, whose line could
hold, was replaced by `done-when-conflict`. Replays recorded before
atelier-s63j ran on fixtures that kept some of the old tracker's process
text.

- [2026-11-11-next-skill.md](results/2026-11-11-next-skill.md): the skill
  against no skill at the cutover, and the cuts it led to.
- [2026-11-17-branch-model-smoke.md](results/2026-11-17-branch-model-smoke.md):
  the matrix on the branch model.
- [2026-11-18-replayed-reviews.md](results/2026-11-18-replayed-reviews.md)
  and [2026-11-18-nine-more-replays.md](results/2026-11-18-nine-more-replays.md):
  the first replays and what one costs.
- [2026-11-18-three-findings.md](results/2026-11-18-three-findings.md) and
  [2026-11-19-say-no.md](results/2026-11-19-say-no.md): an implementer
  with three findings, before and after atelier-hlsw's `not built`.
- [2026-11-19-overbuilt-told-apart.md](results/2026-11-19-overbuilt-told-apart.md):
  the sized reviews.
- [2026-11-19-removed-replays.md](results/2026-11-19-removed-replays.md):
  the removal replays.
- [2026-11-19-later-round-brief.md](results/2026-11-19-later-round-brief.md)
  and [2026-11-19-later-round-fix.md](results/2026-11-19-later-round-fix.md):
  the later rounds, before and after atelier-d73k's brief.
- [2026-11-19-both-setups.md](results/2026-11-19-both-setups.md): both
  setups, both arms and every case built before these; the skill before
  atelier-mtzj; and in `sol` the skill cut to its Starting Another Thread
  section.
- [2026-11-19-why-touched-claims-missed.md](results/2026-11-19-why-touched-claims-missed.md):
  why reviews with the skill miss `touched-claims`, and the skill without
  `roles/reviewer.md` or its Size and Verdict.
- [2026-11-19-reviewer-passages.md](results/2026-11-19-reviewer-passages.md):
  the skill without each of `SKILL.md`'s first four parts on
  `touched-claims`, and "What to Judge" and the human's scale of severity
  for the reviewer. The scale was merged by the lead's ruling, with the
  floor replays and the run in `house` left unmade.
- [2026-11-23-told-to-file.md](results/2026-11-23-told-to-file.md): an
  implementer told to file what it finds outside its item, and not told.
