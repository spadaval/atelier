# Does a later round's brief hide a defect the fix introduced? (2026-11-19)

Item atelier-t5tc, for mission atelier-8az0. The later-round case
`later-round-fix-wrong` ([README](../README.md#later-rounds)) on `sol`
(`gpt-6.1-sol` at `high`, skill arm), with the binary before atelier-d73k's
change to the brief and after it, as two separate matrices of ten runs a
cell.

**Result.** The new brief did not hide it. The fix of round 1's two
findings adds a `parse_day` that refuses every real date on the 30th or
31st of a month, and Sol found it and blocked on it in all 40 reviews:
20 of 20 with the old brief and 20 of 20 with the new one, in both
matrices. It is not found less often after in either matrix, so
atelier-d73k's brief stays.

| Matrix | Binary | Passes | Defect blocks | Verdicts | Other new findings | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | before | 10 of 10 | 10 | changes 10 | 1 (defer) | 73 | 135.8k | 2.4k |
| 1 | after | 10 of 10 | 10 | changes 10 | 0 | 65 | 131.9k | 2.2k |
| 2 | before | 10 of 10 | 10 | changes 10 | 0 | 68 | 136.4k | 2.4k |
| 2 | after | 10 of 10 | 10 | changes 10 | 0 | 64 | 132.6k | 2.3k |

"Defect blocks" counts the runs with a fix-now finding that valid dates on
the 30th and 31st are refused, the graded question `blocks`. In every run
it was the review's only new finding at fix now, and every review set both
of round 1's carried findings to fixed. No run was invalid, and none hit a
provider limit.

## The year 9999 overflow

The fix also holds the flaw atelier-d73k's results found by chance:
`week_of` overflows for a DATE in the last week of year 9999. With the
cases of atelier-d73k, 12 of 40 reviews reported it with the old brief and
none of 40 with the new one. Here 1 of 20 reviews reported it with the old
brief (matrix 1, run 8, as `defer`), and none of 20 with the new one. With
a real defect in the same commit, the old brief rarely got to it either,
so this case says little more about that difference. What it does show is
that the narrowing atelier-d73k saw did not reach a defect that matters.

## Setup

- **Binaries.** As in
  [atelier-d73k's results](2026-11-19-later-round-brief.md): before is
  master at 221d5c146, `/var/tmp/af-evals/bin/atelier-221d5c146`; after is
  698a306fa, atelier-d73k's commit of the brief,
  `/var/tmp/af-evals/bin/atelier-698a306fa`. Its crates differ from
  221d5c146's in the brief and its test alone. Each result names the
  binary it ran, and all 20 of each arm name the right one.
- **Case and skill.** The case and the skill as at 4c1890c19 (`--ref`), the
  commit that added the case. The skill is master's.
- **Harness.** As on master. The judge is Claude Code on `claude-opus-5-5`
  at high, `sol`'s judge.
- **Runs.** `~/.cache/af-evals/runs/t5tc/m<matrix>-<binary>/`, by
  `matrix.sh` in that directory. Each matrix ran its before and after cells
  side by side, `--jobs 2` each. Matrix 1 ran 03:03 to 03:19, matrix 2
  03:19 to 03:30.

I checked each judge's answer against the review's findings. It agrees in
all 40 runs: each review's third finding is the refused dates, recommended
fix now, under a title such as "Valid dates on the 30th and 31st are
rejected".

## Cost

40 runs. A review took 64 to 73 seconds at the median, 132k to 136k tokens
in and 2.2k to 2.4k out. The new brief cost slightly less than the old one
here, about 4k fewer tokens in at the median.
