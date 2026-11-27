# Which part of the skill costs a defect, and two passages for the reviewer (2026-11-19)

Item atelier-oiwy, mission atelier-kw8u. atelier-7h71 found that reviews
with the skill miss the `touched-claims` defect, that `roles/reviewer.md`
is not why, and that the first four parts of `SKILL.md` were the next
suspect. This run cuts each of the four in turn, and tries two passages
for the reviewer: "What to Judge", cut on 2026-11-11 (b22a03359), and the
human's scale of severity. Every run is `sol` (`gpt-6.1-sol` at high,
judged by `claude-opus-5-5`).

**Result.**

- **No one part of `SKILL.md` costs the defect.** With each of the four
  parts cut, ten runs each, the skill found the defect in 1 of 40, against
  3 of 10 with no skill and 3 of 10 with the threads-only skill in 7h71
  (Fisher's exact test, 1 of 40 against 6 of 20: p = 0.004). In `sol`, arms
  that keep the rest of the body have now found it in 3 of 103 runs, and
  arms without it in 8 of 23. Whatever costs it lies in what the four parts
  hold together, or in the role file and the body acting together, and no
  arm here can say which.
- **"What to Judge" is left out, by the item's rule, and it did worse on
  the replays.** It gained one run on one cell (`priority-contract` 5 of 5
  against 4 of 5 in matrix 2) and lost runs on four cases: `costly-hardening`
  4 of 5 against 5 of 5 and `touched-claims` 0 of 5 against 1 of 5 in
  matrix 1, `base-done-claims` 0 and 1 of 5 against 2 and 4 of 5, and
  `quiet-tokens` 0 of 5 against 3 and 2 of 5 in both. Over the four replays
  some review has found it found the known defect 9 times in 40 runs, against
  the skill's 19. It made reviews longer (a median of 127 s against 90 s on
  `costly-hardening`) and raised more fix-now findings (176 against 165 on
  the 14 cases that finished), of which the judge rated 9 useful, as many as
  the skill's.
- **The scale is merged, by the lead's ruling of 2026-11-27, not by the
  item's rule.** On the 14 review cases that finished in both matrices,
  reviews with the skill raised 9 fix-now findings the judge rated useful
  or nit, and reviews with the scale none. The known defects found were 9
  against 10 in matrix 1 and 10 against 10 in matrix 2. The rule also asks
  for the eight replays no review has found, and those ran only in matrix
  1's skill and "What to Judge" arms before both allowances ran out. All 9
  of the skill's useful fix-now findings came on one case, `quiet-tokens`,
  where the scale found the defect in 2 of 10 runs against the skill's 5
  of 10 (p = 0.35). With the scale, reviewers labelled every finding
  themselves, and the judge agreed with the label on 165 of 176.

## The cuts of `SKILL.md`

`touched-claims`, ten runs a cell. The skill arm and the comparison arms
are 7h71's ([2026-11-19-why-touched-claims-missed.md](2026-11-19-why-touched-claims-missed.md)),
on the same binary and the same skill.

| Arm | Found the defect | Median s, tokens in |
| --- | --- | --- |
| Skill without the opening lines (Run `atelier status` first) | 0 of 10 | 156 s, 470k |
| Skill without Your Role | 0 of 10 | 108 s, 446k |
| Skill without Who Works With the Human | 1 of 10 | 133 s, 453k |
| Skill without Independent Review | 0 of 10 | 110 s, 405k |
| Skill (7h71) | 0 of 10 | 130 s, 473k |
| No skill (7h71) | 3 of 10 | 108 s, 448k |
| Threads-only skill (7h71) | 3 of 10 | 112 s, 387k |

The one review that found it, without Who Works With the Human
(`tc-nohuman/…-sol-4-b796`), holds one fix-now finding: a commit touching
one issue that advances a mission reaches the mission and every sibling,
so `land` re-checks their old proof and fails.

## The passages

Two matrices, each with three arms: the skill, the skill with "What to
Judge" and the skill with the scale. Five runs a cell, and three for the
eight replays no review had found. Passes:

| Case | m1 skill | m1 What to Judge | m1 scale | m2 skill | m2 What to Judge | m2 scale |
| --- | --- | --- | --- | --- | --- | --- |
| `vacuous-test-review` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `costly-hardening` | 5 of 5 | 4 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `overbuilt-generality` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `overbuilt-layer` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `overbuilt-robustness` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 ‡ | 5 of 5 |
| `control-large` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `control-lean` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `later-round-cosmetic` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `later-round-wrong` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `later-round-fix-wrong` | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 | 5 of 5 |
| `touched-claims` | 1 of 5 | 0 of 5 | 1 of 5 | 0 of 5 | 0 of 5 | 0 of 5 |
| `base-done-claims` | 2 of 5 | 0 of 5 | 3 of 5 | 4 of 5 | 1 of 5 | 5 of 5 |
| `priority-contract` | 3 of 5 | 3 of 5 | 5 of 5 | 4 of 5 | 5 of 5 | 4 of 5 |
| `quiet-tokens` | 3 of 5 | 0 of 5 | 1 of 5 | 2 of 5 | 0 of 5 | 1 of 5 |
| `approval-follow-up` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `base-record-commits` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `merge-sync` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `brief-transitions` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `hierarchy-rules` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `project-slug` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `readiness-blockers` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |
| `after-merge-checks` | 0 of 3 | 0 of 3 | not run | not run | not run | not run |

‡ Graded 4 of 5 when made: the judge answered pass on one run, quoting a
line that is not in its material, so the harness made it unknown. Judged
again (`judge.py`, `rejudge.json` beside it), it passes with the line
found. The reviewer's finding named the unneeded robustness either way.

Fix-now findings, and how many of them the judge rated useful or nit,
from the reported `levels` question:

| Case | m1 skill | m1 What to Judge | m1 scale | m2 skill | m2 What to Judge | m2 scale |
| --- | --- | --- | --- | --- | --- | --- |
| `vacuous-test-review` | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 |
| `costly-hardening` | 20, 0 | 23, 0 | 19, 0 | 20, 0 | 25, 0 | 19, 0 |
| `overbuilt-generality` | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 |
| `overbuilt-layer` | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 |
| `overbuilt-robustness` | 5, 0 | 10, 0 | 5, 0 | 5, 0 | 10, 0 | 5, 0 |
| `control-large`, `control-lean`, `later-round-cosmetic` | 0, 0 | 0, 0 | 0, 0 | 0, 0 | 0, 0 | 0, 0 |
| `later-round-wrong` | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 |
| `later-round-fix-wrong` | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 | 5, 0 |
| `touched-claims` | 1, 0 | 0, 0 | 1, 0 | 0, 0 | 0, 0 | 0, 0 |
| `base-done-claims` | 7, 0 | 5, 0 | 8, 0 | 9, 0 | 7, 1 | 10, 0 |
| `priority-contract` | 14, 0 | 15, 0 | 19, 0 | 15, 0 | 17, 0 | 16, 0 |
| `quiet-tokens` | 10, 4 | 9, 4 | 5, 0 | 9, 5 | 5, 4 | 5, 0 |
| **The 14 cases** | **82, 4** | **87, 4** | **82, 0** | **83, 5** | **89, 5** | **80, 0** |
| Floor replays (m1) | 62, 6 | 62, 11 | not run | not run | not run | not run |

The judge rated no finding in any arm `wrong` (not real).

What the item's rules ask, on the runs that exist:

- **"What to Judge"** needs some case to pass at least two more runs with
  it in each matrix and none to pass fewer in either. No case gains two
  runs in either matrix; its one gain is a run on `priority-contract` in
  matrix 2. Four cases pass fewer: `costly-hardening` and `touched-claims`
  in matrix 1, and `base-done-claims` and `quiet-tokens` in both. Left
  out.
- **The scale** needs fewer fix-now findings rated useful or nit in both
  matrices (4 to 0, 5 to 0 on the 14 cases) and no fewer known defects
  found in either (9 to 10, 10 to 10 on the four replays some review has
  found). It holds on those. The eight floor replays were not run with it,
  and neither was matrix 2's floor, so the rule is not met as written.

## What the reviews did

**The reviewer's own label against the judge's**, over the 14 cases and
both matrices:

| Arm | Fix now, judge critical or important | Fix now, judge useful | Fix now, `Shrink:` or `Reject:` | Defer, judge useful or nit | Defer, judge important |
| --- | --- | --- | --- | --- | --- |
| Skill | 116 | 9 | 40 | 12 | 2 |
| What to Judge | 128 | 9 | 39 | 12 | 1 |
| Scale | 122 | 0 | 40 | 13 | 1 |

With the scale, 122 findings opened `Critical:` or `Important:` and 13
`Useful:`. The judge agreed with 85 `Critical:` and all 27 `Important:`,
rated 10 `Critical:` findings important (still fix now), and one
`Useful:` finding important. Reviewers did not rate everything high: the
`Useful:` findings were deferred. On the cases with no defect to find, the
counts did not move: the sized cases' `Shrink:` findings, the controls'
silence and the later rounds' one blocking finding each stayed the same.

**`quiet-tokens`.** The 9 useful fix-now findings of the skill were all on
this replay. Each one the judge quoted made the same point: quiet mode
gives way to the hook JSON format when both are asked for ("Quiet mode
silently yields to the hook JSON format"), which the judge rated useful.
With the scale, 8 of 10 reviews filed a `Useful:` finding at `defer`, and
those the judge quoted make that point ("Document the hook-protocol
exception to quiet tokens"). The scale arm also found the known defect, a refused quiet
`start` printing the whole report, less often: 2 of 10 against 5 of 10.
Its gains elsewhere (`base-done-claims` 8 of 10 against 6 of 10,
`priority-contract` 9 of 10 against 7 of 10) make the totals equal or
better, but on this case the drop is unexplained and the counts are small.

**"What to Judge".** Its reviewers checked the change against its Goal
and Scope, as the passage asks, and found fewer of the known defects: 3
against 9 in matrix 1 and 6 against 10 in matrix 2. They raised more
findings: on
`overbuilt-robustness` a second fix-now finding beside the `Shrink:` in
every run (10 against 5), and on `costly-hardening` 48 against 40. The one
`costly-hardening` review that failed asked for the ledger's three defects
to be fixed and none for it to be shrunk for its cost, the failure 7h71
saw without Size and Verdict. On `touched-claims` it found the defect in
none of 10 runs, so comparing with the Goal, in these words, does not
send a reviewer to test the walk against its boundary.

Median seconds and tokens in, both matrices together:

| Case | Skill | What to Judge | Scale |
| --- | --- | --- | --- |
| `vacuous-test-review` | 51 s, 104k | 51 s, 104k | 50 s, 108k |
| `costly-hardening` | 90 s, 176k | 127 s, 246k | 100 s, 194k |
| `overbuilt-generality` | 68 s, 137k | 88 s, 158k | 66 s, 148k |
| `overbuilt-layer` | 64 s, 132k | 80 s, 154k | 68 s, 134k |
| `overbuilt-robustness` | 68 s, 131k | 92 s, 145k | 66 s, 144k |
| `control-large` | 88 s, 184k | 91 s, 194k | 83 s, 188k |
| `control-lean` | 50 s, 111k | 58 s, 121k | 51 s, 113k |
| `later-round-cosmetic` | 61 s, 132k | 83 s, 134k | 60 s, 131k |
| `later-round-wrong` | 63 s, 131k | 70 s, 143k | 62 s, 136k |
| `later-round-fix-wrong` | 62 s, 131k | 72 s, 142k | 63 s, 132k |
| `touched-claims` | 131 s, 479k | 132 s, 551k | 108 s, 426k |
| `base-done-claims` | 158 s, 566k | 159 s, 521k | 136 s, 493k |
| `priority-contract` | 196 s, 1,155k | 252 s, 1,206k | 204 s, 1,149k |
| `quiet-tokens` | 193 s, 1,122k | 218 s, 1,256k | 195 s, 1,084k |
| Floor replays (m1), all 24 runs | 220 s, 957k | 237 s, 1,046k | not run |

## What was not run

- The floor replays in matrix 1's scale arm and all of matrix 2's, 96
  runs, and any top-up to five runs. No review in the 48 floor runs made
  found a floor defect, so none would have been topped up so far.
- The run in `house` of a kept passage.
- Both stopped when the allowances ran out on 2026-11-19. The human ruled
  on 2026-11-27 that the item closes on what it has.

## Setup

- **Binary.** `/var/tmp/af-evals/bin/atelier-cd93c5f3a`, 7h71's: `crates/`
  did not change from cd93c5f3a to 6cda5c821, where this item started.
- **Skill.** `skill`: `--ref 6cda5c821`, master's when the item started,
  whose `skills/` equals cd93c5f3a's. Cut arms, each `SKILL.md` with one
  part deleted: `eval/skill-no-opening` 91bf874fd,
  `eval/skill-no-your-role` 8cdff28ed, `eval/skill-no-human` 9ac80d13c,
  `eval/skill-no-review` 1ec05862d. Passages, each adding one section to
  `roles/reviewer.md`: `eval/skill-what-to-judge` 6f921eb59 (before Size
  and Verdict) and `eval/skill-severity-scale` 41fb74211 (before
  Rebuttals). Each differs from 6cda5c821 in its one change. None of the
  six branches is to be merged. All are pushed.
- **Since.** By the time this merged, master's `SKILL.md`, `roles/lead.md`,
  `roles/implementer.md` and several fixtures had changed. The scale was
  measured on the skill as it stood at 6cda5c821, and `roles/reviewer.md`
  had not changed on master before it went in.
- **Harness.** The cut arms ran from 6cda5c821. The passage matrices ran
  from 78847d8e6, this item's branch with the `levels` question added to
  the ten review rubrics; nothing else in the harness differs.
- **Runs.** `~/.cache/af-evals/runs/oiwy/`, one directory per step
  (`tc-*` for the cuts, `m1-*` and `m2-*` for the matrices), driven by
  `matrix.sh` and `chain.sh`, with `watchdog.sh` stopping the matrix at the
  first run invalid on a provider's limit. `cells.py` and `pairs.py` read
  them. Replays ran at `--cargo-jobs 3` and the other cases at `--jobs 8`.
- **Made again.** 28 replays in matrix 2 were invalid when the judge,
  `claude-opus-5-5`, hit its weekly limit (from 10:42; it reset at 14:00
  New York time). All 28 were made again into the same outputs: 8 with
  "What to Judge" and 20 with the scale. The ChatGPT Pro allowance, which
  every `sol` reviewer runs on, then ran out at 15:44 with 4 runs of matrix
  1's floor scale step invalid, and the watchdog stopped the matrix. They
  were not made again.
- **Load.** The host's one-minute load passed 10 in 58 of 278 minutes
  logged, for at most 9 minutes in a row, mostly in the floor replays
  (peak 38.7 at 15:22). Free memory never fell under 6.8 GB, nor free disk
  under 147 GB.
