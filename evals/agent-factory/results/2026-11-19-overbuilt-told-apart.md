# Can Sol tell correct work that is overbuilt? (2026-11-19)

Item atelier-s6xk, for mission atelier-8az0's `overbuilt-told-apart`. The
five sized cases ([README](../README.md#sized-reviews)) on `sol`
(`gpt-6.1-sol` at `high`), with no skill and with the skill, ten runs a
cell.

**Result.** Yes, and the skill makes it block. With no skill, Sol named
the unneeded part in 29 of 30 overbuilt runs, but as fix now in only 20.
With the skill it named it in 30 of 30, every time as a fix-now `Shrink:`
finding. Neither arm asked for less in any of the 40 control runs, and
every one of them approved. The control and overbuilt cases are told apart
in both arms. The cases cannot show that the skill helps a reviewer *see*
overbuilt work, since Sol sees it without the skill. They show that the
skill's sizing passage (`roles/reviewer.md`, "If a much smaller change
gets most of it, record a fix-now finding whose title opens `Shrink:`")
turns a deferred remark into a blocking finding.

| Case | Arm | Passes | Blocking (fix now) | Verdicts | Median s | Median tokens in | Median tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `overbuilt-generality` | none | 10 of 10 | 10 | changes 10 | 54 | 112.6k | 2.0k |
| `overbuilt-generality` | skill | 10 of 10 | 10 | changes 10 | 62 | 147.0k | 2.3k |
| `overbuilt-layer` | none | 10 of 10 | 6 | changes 6, approve 4 | 59 | 111.5k | 2.2k |
| `overbuilt-layer` | skill | 10 of 10 | 10 | changes 10 | 65 | 145.7k | 2.2k |
| `overbuilt-robustness` | none | 9 of 10 | 4 | changes 4, approve 6 | 60 | 110.1k | 2.2k |
| `overbuilt-robustness` | skill | 10 of 10 | 10 | changes 10 | 64 | 142.5k | 2.3k |
| `control-large` | none | 10 of 10 | none asked for less | approve 10 | 84 | 149.2k | 2.9k |
| `control-large` | skill | 10 of 10 | none asked for less | approve 10 | 85 | 168.5k | 2.9k |
| `control-lean` | none | 10 of 10 | none asked for less | approve 10 | 42 | 90.8k | 1.4k |
| `control-lean` | skill | 10 of 10 | none asked for less | approve 10 | 48 | 110.1k | 1.7k |

"Blocking" is the reported `blocks` answer: of the runs that pass, how many
named the unneeded part in a fix-now finding. It changes no grade. No run
was invalid, and no run hit a provider limit.

## Setup

- **Binary.** Built from this branch, whose crates are master's
  (c33dbba7e), and copied once per commit below as
  `/var/tmp/af-evals/bin/atelier-<commit>`. The copies are byte-identical.
- **Skill.** `skills/agent-factory` as on master, staged from the commit
  each case ran at. The skill does not change between those commits.
- **Harness.** As on master. Codex CLI 0.169; the judge is Claude Code on
  `claude-opus-5-5` at high, `sol`'s judge.
- **Runs.** `--jobs 6`, since another eval was using the same allowance.
  Each case's cells come from the commit that last changed its fixture:

  | Case | Commit | Runs |
  | --- | --- | --- |
  | `overbuilt-robustness`, `control-lean` | 6854b8b6f | `~/.cache/af-evals/runs/s6xk/` |
  | `overbuilt-generality`, `overbuilt-layer` | 6004a053b | `~/.cache/af-evals/runs/s6xk-2/` |
  | `control-large` | 457374786 | `~/.cache/af-evals/runs/s6xk-8/` |

## What the reviews said

- **Generality.** Every review, in both arms, asked as fix now for `--csv`
  over `csv.writer` and the exporter framework gone. No skill: "Remove the
  speculative export framework and unrequested formats", "Keep the CSV
  request focused instead of adding a format framework". Skill: "Shrink:
  implement the requested --csv branch with csv.writer".
- **Layer.** With no skill, 6 asked for the layer's removal as fix now
  ("Remove the unnecessary service and repository layers") and 4 named it
  only as defer ("Storage abstraction adds maintenance overhead to a small
  file-based feature") and approved. With the skill, all 10 asked as fix
  now to implement tagging in `notes.py`.
- **Robustness.** With no skill, 4 asked as fix now to drop the guards
  ("Use the loader's validated Item contract instead of adding a second
  input policy"). 5 said the same as defer ("Low-stock selection
  duplicates the loader's validation contract") and approved. One approved
  with no finding. With the skill, all 10 asked as fix now to "filter and
  sort the validated Items".
- **Controls.** Every review of both controls approved with no finding.

I checked the judge's answer against each review's findings. It agrees
with every one: each pass names the unneeded part in a finding's title or
body, and the one failure is an approval with no finding.

## Superseded runs: defects in the fixtures

The cases mean each change to be correct. The first matrix (6854b8b6f, all
five cases) showed that three were not, and the reviews spent findings on
the defects:

- `overbuilt-layer` passed 1 of 10 with no skill. `list --tag ""` listed
  every note (`if args.tag`), and every no-skill review led with that
  fix-now finding; only one also named the layer. The change's
  `test_notes.py` had also lost its `unittest.main()` block. With the skill,
  10 of 10 named the layer beside the defect. Fixed in 6004a053b, after
  which no-skill passed 10 of 10. A real defect crowds out the sizing
  finding without the skill, and not with it.
- `overbuilt-generality`: `--delimiter` with more than one character
  crashed `csv.writer`, which two reviews found. The option was dropped in
  6004a053b; the cell's passes did not change (10 of 10 in both arms).
- `control-large` took seven rounds of fixes, each found by the reviews of
  the round before or by the item's own review (1-1way.1), each with a
  test: an empty `.qif` passing the header check, `^x` ending a record,
  quotes Python's `csv` accepted, a byte that is not UTF-8 giving a
  traceback or the wrong line, a stripped payee no longer matching the
  ledger, `str.splitlines` splitting a payee, an amount ceiling, errors
  naming the wrong line of a record that spans lines, a quoted empty row
  read as blank, a bad header placed on line 1, and ignored QIF lines
  opening a record. The item's wording on ignored QIF lines also read as a
  closed list of `M` and `N`, and now says "such as". Python's `csv` module
  was the cause of most of them: it accepts stray quotes and reports only
  where it stopped. `nordbank_records` reads the format directly instead.
  Every one of the 140 superseded runs passed: no finding asked for less.
  In the last round (457374786) all 20 reviews approved with no finding.

## Cost

280 runs: 100 in the first matrix, 60 in the second, and 20 in each of six
reruns of `control-large`. A review took 42 to 85 seconds at the median in
the cells above, 91k to 169k tokens in and 1.4k to 2.9k out. The skill arm
read 19k to 35k more tokens in a run.
