# Does a later round's brief narrow without hiding? (2026-11-19)

Item atelier-d73k, for mission atelier-8az0's `narrows-without-hiding`.
The two later-round cases ([README](../README.md#later-rounds)) on `sol`
(`gpt-6.1-sol` at `high`, skill arm), with the binary before the brief
change and after it, as two separate matrices of ten runs a cell.

**Result.** The planted flaws cannot tell the two briefs apart. Sol did
not block on the cosmetic flaw with either brief, and found and blocked on
the defect with both. So, by the item's rule, the brief did not narrow on
the cosmetic flaw. It did not hide the defect either, so the sentence on a
new finding in code that did not change stays. One difference showed up
outside what the cases planted: with the old brief, 12 of 40 reviews
reported a real but far-fetched crash in code the fix changed, and with
the new brief none of 40 did (see [An unplanted finding](#an-unplanted-finding)).

| Matrix | Case | Binary | Passes | Flaw blocks | Flaw named | Verdicts | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `later-round-cosmetic` | before | 10 of 10 | 0 | 10 (defer 4, no action 6) | approve 10 | 70 | 129.1k | 2.3k |
| 1 | `later-round-cosmetic` | after | 10 of 10 | 0 | 10 (defer 10) | approve 10 | 68 | 131.0k | 2.3k |
| 1 | `later-round-wrong` | before | 10 of 10 | 10 | 10 (fix now 10) | changes 10 | 71 | 130.4k | 2.4k |
| 1 | `later-round-wrong` | after | 10 of 10 | 10 | 10 (fix now 10) | changes 10 | 71 | 131.1k | 2.3k |
| 2 | `later-round-cosmetic` | before | 10 of 10 | 0 | 10 (defer 4, no action 6) | approve 10 | 68 | 129.5k | 2.3k |
| 2 | `later-round-cosmetic` | after | 10 of 10 | 0 | 10 (defer 10) | approve 10 | 69 | 129.5k | 2.2k |
| 2 | `later-round-wrong` | before | 10 of 10 | 10 | 10 (fix now 10) | changes 10 | 69 | 131.8k | 2.4k |
| 2 | `later-round-wrong` | after | 10 of 10 | 10 | 10 (fix now 10) | changes 10 | 66 | 131.1k | 2.3k |

"Flaw blocks" counts the runs with a fix-now finding on the planted flaw:
for the cosmetic case it is the runs the graded question fails, and for
the wrong case the runs it passes. "Flaw named" counts the runs where any
finding named it, with that finding's recommendation. No run was invalid,
and none hit a provider limit. Every review set both of round 1's carried
findings to fixed.

## What the item's rules give

- **Cosmetic flaw blocks, before and after:** 0 of 10 and 0 of 10 in
  matrix 1, and the same in matrix 2. It blocks no less often after in
  both matrices, so by the item's rule **the brief did not narrow** on
  this case. Sol at round 2 already kept the docstring from blocking with
  the old brief. The one change is the recommendation: with the old
  brief, 12 of 20 reviews marked it `no action` and 8 `defer`. With the
  new brief, all 20 marked it `defer`, as the brief says to record a
  finding that does not block.
- **The defect, before and after:** found and blocked in 10 of 10 in every
  cell. It is not found less often after in either matrix, so the
  sentence on a new finding in code that did not change stays.

## An unplanted finding

`week_of`'s Sunday is `monday + timedelta(days=6)`, from the fix commit,
so it is code that changed since round 1. For a DATE in the last week
`datetime.date` can hold (9999-12-27 to 9999-12-31), it overflows:
`hours.py week 9999-12-31` ends in `OverflowError` instead of a week's
report. The fixture did not mean to hold this, and the rubric said the fix
held no defect. The rubric now says it does. That changes no graded
question, so no run was judged again.

| Matrix | Binary | Reviews naming the overflow | As fix now | As defer |
| --- | --- | --- | --- | --- |
| 1 | before | 6 of 20 | 3 | 3 |
| 1 | after | 0 of 20 | 0 | 0 |
| 2 | before | 6 of 20 | 1 | 5 |
| 2 | after | 0 of 20 | 0 | 0 |

Ten of the twelve were in `later-round-wrong`, two in
`later-round-cosmetic`. With the new brief no review's findings or final
reply mentions the overflow or the year 9999.

This is the brief narrowing where it was not meant to. The new rule on
blocking covers only code that did not change, and this line changed. The
cases cannot say which words did it. The brief now asks the reviewer to
"check that the commits since then are correct", and a reviewer may read
that as checking each fix against its finding. Or the rule on unchanged
code may make the whole review more cautious. The finding itself is of
little worth: a crash on a date nobody keeps hours for, which 8 of the 12
reviews that found it deferred. The cases show the new brief drops a
low-value finding. They do not show whether it would drop an important
one in changed code, since neither case plants a defect there.

## Setup

- **Binaries.** Before: master at 221d5c146, built in its own worktree and
  copied as `/var/tmp/af-evals/bin/atelier-221d5c146`. After: 698a306fa,
  this item's commit of the brief, whose only parent is 221d5c146 and whose
  crates differ from it in the brief and its test alone, copied as
  `/var/tmp/af-evals/bin/atelier-698a306fa`. Each result names the binary
  it ran (`atelier`), and all 40 of each arm name the right one.
- **Brief.** The wording is 698a306fa's, written before the first run and
  not changed after.
- **Cases and skill.** Both cases and the skill as at 697a6d23a (`--ref`),
  the commit that added the cases. The skill is master's.
- **Harness.** As on master. Codex CLI 0.169; the judge is Claude Code on
  `claude-opus-5-5` at high, `sol`'s judge.
- **Runs.** `~/.cache/af-evals/runs/d73k/m<matrix>-<binary>/`, by
  `matrix.sh` in that directory. Each matrix ran its before and after cells
  side by side, `--jobs 2` each, since atelier-id3v's matrix was using the
  same allowance. Matrix 1 ran 02:16 to 02:34, matrix 2 02:34 to 02:49.

I checked each judge's answer against the review's findings by hand. It
agrees in all 80 runs: every cosmetic run's docstring finding is `defer`
or `no action`, and every wrong run holds a fix-now finding that the
projects are sorted by hours instead of alphabetically.

## Cost

80 runs. A review took 66 to 71 seconds at the median, 129k to 132k
tokens in and 2.2k to 2.4k out. The new brief cost no more than the old
one.
