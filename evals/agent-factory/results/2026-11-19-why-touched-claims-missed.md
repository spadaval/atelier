# Why reviews with the skill miss `touched-claims` (2026-11-19)

Item atelier-7h71, mission atelier-kw8u. atelier-id3v found the replayed
review `touched-claims` in 5 of 6 reviews with no skill and 0 of 6 with it.
This run checks that gap and looks for the text that causes it.

**Result.**

- **The gap holds.** Over ten runs a cell, the reviews with no skill found
  the defect in 10 of 20 and the reviews with the skill in 4 of 20 (`sol`
  3 to 0, `house` 7 to 4). With id3v's runs added, the totals are 15 of 26
  against 4 of 26 (Fisher's exact test, two-sided, p = 0.003).
- **The skill does not make reviewers look less. It makes them look the
  other way.** In both arms, 19 of 26 reviews went past the test suite and
  7 of 26 ran only the tests. Of the 19 that went further with no skill, 15
  checked whether the new walk reaches claims it should not (unchanged
  sibling work of the same mission), and all 15 found the defect. With the
  skill, 15 of 19 checked only whether the walk reaches every claim it
  should (an `advances` link stored on one endpoint, reopened child work),
  and none of those 15 found it.
- **Cutting `roles/reviewer.md` does not bring the defect back in `sol`.**
  With its Size and Verdict cut, the skill found the defect 0 of 10 times.
  With no `roles/reviewer.md` at all, it also found it 0 of 10 times. As a
  control outside the Done when, id3v's `threads-only` skill (the prompt
  line and a `SKILL.md` holding only Starting Another Thread) found it 3 of
  10 times, the same as no skill. That points to the part of `SKILL.md`
  above Starting Another Thread as the next suspect: Run `atelier status`
  first, Your Role, Who Works With the Human, and Independent Review. In
  `sol`, every arm that kept that text found the defect 0 of 33 times, and
  the arms without it 8 of 23 times (p < 0.001). Cuts that stay at zero
  cannot show that the reviewer file adds nothing to the miss, since a
  count cannot fall below zero and the two files may act together. They
  also cannot tell which of the four sections matters, or whether a new
  reviewer passage could counter it: no arm added text.
- **Size and Verdict is what wins `costly-hardening`, and only that.**
  Without it the skill passed 0 of 5, and with no reviewer file 1 of 5,
  against 10 of 10 with the whole skill. On the overbuilt cases, the
  `Shrink:` blocking that atelier-s6xk credited to Size and Verdict comes
  from elsewhere in the skill: both cut arms blocked in 15 of 15 runs, and no
  skill blocked in 10 of 15 today. No control run in any arm asked for less.

## All of it

Passes, and for an overbuilt case the runs whose finding blocked (fix now).
Every cell is `sol` (`gpt-6.1-sol` at high) unless it names `house`. Cells
marked † are from earlier items and ran on other days: `costly-hardening`
with the skill from atelier-id3v
([2026-11-19-both-setups.md](2026-11-19-both-setups.md)), and the
overbuilt and control cases with the skill from atelier-s6xk
([2026-11-19-overbuilt-told-apart.md](2026-11-19-overbuilt-told-apart.md)).
Every other cell is from today, and its runs are fresh.

| Case | No skill | Skill | Skill without Size and Verdict | Skill without `roles/reviewer.md` | Threads-only skill |
| --- | --- | --- | --- | --- | --- |
| `touched-claims` | 3 of 10 | 0 of 10 | 0 of 10 | 0 of 10 | 3 of 10 |
| `touched-claims`, `house` | 7 of 10 | 4 of 10 | not run | not run | not run |
| `costly-hardening` | 0 of 4 | 10 of 10† (5 `sol`, 5 `house`) | 0 of 5 | 1 of 5 | not run |
| `overbuilt-generality` | 5 of 5, 5 block | 10 of 10†, 10 block | 5 of 5, 5 block | 5 of 5, 5 block | not run |
| `overbuilt-layer` | 3 of 5, 3 block | 10 of 10†, 10 block | 5 of 5, 5 block | 5 of 5, 5 block | not run |
| `overbuilt-robustness` | 5 of 5, 2 block | 10 of 10†, 10 block | 5 of 5, 5 block | 5 of 5, 5 block | not run |
| `control-large` | 5 of 5 | 10 of 10† | 5 of 5 | 5 of 5 | not run |
| `control-lean` | 5 of 5 | 10 of 10† | 5 of 5 | 5 of 5 | not run |

Median seconds and tokens in, from today's runs:

| Case | No skill | Skill | Without Size and Verdict | Without `roles/reviewer.md` | Threads-only |
| --- | --- | --- | --- | --- | --- |
| `touched-claims` | 108 s, 448k | 130 s, 473k | 108 s, 410k | 134 s, 488k | 112 s, 387k |
| `touched-claims`, `house` | 215 s, 839k | 188 s, 805k | | | |
| `costly-hardening` | 70 s, 135k | | 72 s, 155k | 94 s, 219k | |
| `overbuilt-generality` | 57 s, 132k | | 64 s, 145k | 87 s, 173k | |
| `overbuilt-layer` | 61 s, 127k | | 65 s, 130k | 79 s, 175k | |
| `overbuilt-robustness` | 60 s, 127k | | 67 s, 131k | 72 s, 141k | |
| `control-large` | 81 s, 153k | | 83 s, 189k | 90 s, 186k | |
| `control-lean` | 46 s, 91k | | 46 s, 110k | 56 s, 131k | |

What each comparison rests on:

- **No skill against the skill on `touched-claims`:** 10 of 20 against 4 of
  20 today (p = 0.10 alone), 15 of 26 against 4 of 26 with id3v's twelve.
  The gap is the same direction in both setups, but smaller than id3v's
  six reviews showed, and `house`'s skill arm found the defect 4 times.
- **In `sol`, any skill keeping `SKILL.md`'s body against none:** 0 of 33
  (the skill, both cuts and id3v's three) against 8 of 23 (no skill, today
  and id3v, and threads-only). Threads-only against no skill, 3 of 10 and 3
  of 10, is the control that rules out the prompt line and the loading of a
  skill.
- **Overbuilt blocking:** 10 of 15 with no skill against 15 of 15 in each
  cut arm, five runs a cell (p = 0.04). s6xk's no-skill arm blocked 20 of
  30, so today's baseline agrees with it.
- **`costly-hardening`:** 0 of 4 with no skill, 0 of 5 and 1 of 5 in the cut
  arms, against id3v's 5 of 5 in each setup with the whole skill. Each of
  the nine failing cut-arm reviews asked for the ledger's defects to be
  fixed, and none asked for it to be shrunk or dropped for its cost. The one
  that passed, with no reviewer file, asked to replace the ledger with a
  note in the docs.

## What the reviews did

I read the twelve reviews id3v kept and today's 40 replays of the two arms:
each review file, each reviewer's messages, and the scripts each one ran
past the test suite. For the 30 replays of the cut arms and threads-only, I
read the messages and searched every transcript for a check on siblings.
Every review the judge passed holds a fix-now finding on the walk through a
mission to its siblings, and I found no other review that made that point.

**Where the reviews with no skill found it.** Each of the 15 that found it
reached the defect the same way. It read the Goal ("`land` checks only what
changed") and asked whether the new `advances` walk keeps that promise.
Then it built two tasks advancing one mission, failed the proof of one,
touched the other, and watched `land` refuse the untouched one. A `house`
reviewer said it plainly in a message: "I'm checking whether the new
traversal also pulls in unchanged sibling claims under the same mission,
which could defeat `land`'s intended scope and speedup." The findings give
that reproduction, and several cite the walk at `session.rs:772-778` and
`:845-867`.

**What the skill's reviews checked instead.** Of the 22 skill reviews that
missed it, 15 went past the suite in the other direction. They checked an
`advances` link stored only on the mission, only on the work, and a
reopened child of advancing work. All of these test whether the walk
misses a claim, and all of them passed. The other 7 ran the suite and
approved. In `sol` the same split holds in both cut arms: no review in
either one checked whether the walk reaches siblings, and those that went
past the suite checked one-sided links and nested work.

**What they wrote under Size and Verdict.** Every one of the 22 failing
skill reviews gives the change's own aim as its Value, in words like
"reopening advancing work must invalidate a completed mission's claim".
Three of them also state the Goal's boundary and then do not test it:
today's `house-3-8a5b` ("while unrelated historical claims remain
trusted"), and id3v's `house-2-434a` ("while unrelated historical claims
remain eligible to be skipped") and `sol-1-b165` ("while skipping
unaffected history"). All three approved. The two in `house` tested only
one-sided links and child work, and the one in `sol` ran only the suite.
I read all 22 Value lines for this count. The other 19 do not say that
unrelated claims stay skipped. Where they say "unchanged", they mean the
mission's own record, which is the change's aim. One of them, `sol-7-ba88`,
names skipping only as the reason to check more ("skipping historical
verification requires checking affected mission claims"). Under
Risk, 10 rate the change high for the opposite failure: "an omitted
dependency can silently skip a required completion check." Another 11 name
the defect's own class and rate it low or medium, most of them because it
fails visibly:
"an overly broad traversal produces a visible refusal and is cheap to
undo", "extra rejections are visible and reversible". Those words come from
the Risk scale's low end ("a failure shows and is cheap to undo") and its
high end ("silent wrong results"). The Verdicts all read "the value
warrants the change", with no `Shrink:` or `Reject:`. One passing `house`
skill review gave the Goal's aim as its Value ("without making ordinary
landing recheck all historical claims"), tested it, and found the defect.
So stating the boundary is not enough. The reviews that found the defect
are the ones that built a case to test it.

The cut arms show what those Size lines are. Without Size and Verdict, `sol`
reviewers wrote no scales and still checked only the direction of missing
claims. The Size lines record where a reviewer was already looking, and
they are not needed for it to look there.

## What it points to

- **For atelier-oiwy.** Cutting `roles/reviewer.md`, or its Size and
  Verdict, did not restore the defect in `sol`. Cutting Size and Verdict
  also loses `costly-hardening` (0 of 5 without it). So removing text from
  the reviewer file is not the fix these runs support. Whether a new
  reviewer passage could counter the miss is untested, and it is the arm
  atelier-oiwy can run: for example, one that asks a reviewer to test the
  change against the Goal as well as against its own claim. The first four
  sections of `SKILL.md` are the next suspect for the cause, and one cut
  arm each (Run `atelier status` first and Your Role, Who Works With the
  Human, Independent Review) would narrow it. In `house`, where the skill
  still found the defect 4 of 10 times, no cut arm was run.
- **For the human.** The skill trades one kind of review for another.
  With it, reviewers block more on size: `costly-hardening` goes from 0 to
  10 of 10, and overbuilt findings block in 15 of 15 runs. On this replay
  they also follow the change's own claim and test that it is complete,
  where a reviewer with no skill tests it against the Goal. That costs a
  defect that runs the wrong way: a check that does too much. The Risk
  scale's wording, which rates a visible failure low, then lets a reviewer
  who sees that kind of failure decide it does not need testing. Whether
  the Risk scale should rate a visible refusal of valid work as low is a
  question of the skill's intent, and atelier-oiwy can test it as an arm.

## Setup

- **Binary.** Built from master at cd93c5f3a (`atelier 0.7.0`), copied to
  `/var/tmp/af-evals/bin/atelier-cd93c5f3a`. A round-1 review's brief is
  word for word the same as in s6xk's binary (c33dbba7e).
- **Skill.** `skill`: `--ref cd93c5f3a`, master's. `Without Size and
  Verdict`: `--ref 95e1e60db`, branch `eval/skill-no-size-verdict`, master's
  skill with that section of `roles/reviewer.md` deleted. `Without
  roles/reviewer.md`: `--ref 4e91a3e53`, branch `eval/skill-no-reviewer`,
  master's skill with the file deleted; `SKILL.md` still links it, and
  every reviewer in that arm tried to read it and went on. `Threads-only`:
  `--ref 3f9dfda5b`, id3v's `eval/skill-threads-only`. None of the three
  branches is to be merged. All are pushed.
- **Harness.** master at cd93c5f3a, unchanged. Codex CLI and Claude Code as
  in id3v. `sol` and `house` as in the README.
- **Runs.** `~/.cache/af-evals/runs/7h71/`, one directory per step, driven
  by `matrix.sh` (`confirm`, then `arms`). The replays ran at
  `--cargo-jobs 3` and the sized cases at `--jobs 8`, side by side. The
  scripts that read the runs are in the same directory: `dump.py`,
  `classify.py`, `reads.py` and `cells.py`.
- **Made again.** OpenAI ended four `costly-hardening` attempts with "This
  content was flagged for possible cybersecurity risk", as it did in id3v:
  three with no skill and one without `roles/reviewer.md`. The harness
  graded them `null`, and each run was made again once. One no-skill run was
  flagged on its second attempt as well, so that cell has 4 runs. No run hit
  a rate limit.
