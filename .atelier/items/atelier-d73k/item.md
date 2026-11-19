---
schema: atelier.item
schema_version: 3
id: atelier-d73k
title: Say in a later round's brief what the round is for, and show with two cases that it narrows without hiding
status: approved
mission: atelier-8az0
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3A5dd6a795-3174-4859-9ff2-d69a1d4f5ff4
ready:
  by: 'the human, by a side chat on 2026-11-18: tell the main thread about all of this'
  at: 2026-11-19T04:37:19.810241548Z
trivial: null
caused_by: null
created_at: 2026-11-19T04:12:27.403171799Z
---

## Goal

A reviewer in round 4 is told what a reviewer in round 1 is told. The brief
names the round and repeats round 1's instruction word for word, with the
diff of the whole change first. In a survey of 143 Sol reviews on
2026-11-18 the share of findings that were critical or important fell from
75% in round 1 to 39% from round 3 on, and 18 findings were another case of
a gap already reported on the same item. Seven of the 18 findings from
round 3 on were still important, so the brief must narrow where a reviewer
looks without hiding a real defect.

## Done when

- [x] For a round after the first, the brief says how many rounds have
  reviewed the whole change, and that this round checks the earlier
  findings and what changed since. The diff since the last reviewed commit
  comes before the diff of the whole change, and both are given.
- [x] It says a new finding in code that did not change blocks only if the
  work would come out wrong without it, and is otherwise for later work.
- [x] Round 1's brief is as it was, and a test covers the brief of rounds
  1, 2 and 3 of a fixture item.
- [x] Two round-2 review cases are added, each a fixture, facts and a
  rubric: round 1's findings are fixed, and text that did not change holds
  a flaw. In one the flaw is cosmetic, and the case passes when the review
  approves or does not block on it. In the other the flaw makes the work
  come out wrong, and the case passes when the review blocks on it.
  `test.sh` runs both with no model.
- [x] The brief's wording is settled before any run and not changed
  after one. The results give both cases in `sol`, with the binary before
  this change and after, as two separate matrices of ten runs a cell.
- [x] The results also give how often the cosmetic flaw blocks, before
  and after. Where it blocks no less often after in both matrices, the
  Report says the brief did not narrow.
- [x] If the second flaw is found less often after in both matrices, the
  sentence on a new finding in code that did not change is taken out, the
  brief keeps the count of rounds and the order of the diffs, and the
  Report says so. The wording is not tried again in this item.
- [x] `docs/design.md` says what a later round's brief holds and why.

## Scope

The review brief in `crates/` and its tests; two new directories under
`evals/agent-factory/scenarios/`; `evals/agent-factory/results/`;
`docs/design.md`.

## Report

**The planted flaws show neither narrowing nor hiding, so the brief did
not narrow on the cosmetic case.** In 80 Sol reviews, the cosmetic flaw
blocked in 0 of 20 with the old brief and 0 of 20 with the new one, and
the defect blocked in 20 of 20 with each. That holds in both matrices of
ten runs a cell. The cosmetic flaw blocks no less often after in both
matrices, so by the Done when's rule the brief did not narrow there. The
defect is not found less often after in either, so the sentence on a new
finding in code that did not change stays. Results:
`evals/agent-factory/results/2026-11-19-later-round-brief.md`.

**One difference the cases did not plant: the new brief stopped every
report of a real crash in changed code.** `week_of` in the fix commit
overflows for a DATE in the last week of year 9999. With the old brief,
12 of 40 reviews reported it (4 as fix now, 8 as defer). With the new
brief, 0 of 40 did. The rule on blocking covers only code that did not
change, and this line changed, so the brief narrowed further than its
words say. The finding is of little worth, but the cases cannot show
whether an important defect in changed code would be dropped the same
way, since neither plants one. Under the Done when I did not change the
wording after a run. A case with a defect in the fix commit would answer
the question, and that is for the lead to order.

What changed:

- `crates/atelier-cli/src/output.rs`: from round 2 on, the reviewer's
  brief opens with how many rounds have reviewed the whole change, says
  this round checks the earlier findings and what changed since the last
  round's review, and says a new finding in code that did not change since
  then blocks only if the work would come out wrong without it, otherwise
  `defer`. The diff since the last reviewed commit comes first, then the
  whole change's. Round 1's brief is byte for byte as before.
  `a_later_rounds_brief_says_what_the_round_is_for_and_puts_the_new_diff_first`
  in `tests/scenarios.rs` checks rounds 1, 2 and 3 of a fixture item.
  `cargo nextest run -p atelier-cli`: 220 passed.
- `evals/agent-factory/scenarios/later-round-cosmetic` and
  `later-round-wrong`: one item in round 2 with both round-1 findings
  fixed. They differ only in `week_lines`, which round 1 reviewed and the
  fix left alone. In one its docstring is wrong and its code right; in the
  other its code sorts projects by hours where the Done when asks for
  alphabetical order, which no test can see. `test.sh` runs both with no
  model: all eval fixtures and harness tests pass.
- `docs/design.md`: what a later round's brief holds, and a Decisions row
  with the survey and these results.

Beyond the Scope's list: the cases share a setup, grader and tests in
`evals/agent-factory/harness/` (`later_round.sh`, `later_round.py`,
`later_round_tests.py`, `later-round/`), as the sized cases do. The
README's scenario table, Running list and a Later rounds section name
them. After the runs I corrected both rubrics' claim that the fix holds
no defect. No graded question depends on it, so no run was judged again.

The binaries: before is master at 221d5c146, and after is 698a306fa,
whose crates differ from it only in the brief and its test. The cases and
the skill were staged from 697a6d23a.
