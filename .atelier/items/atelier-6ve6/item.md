---
schema: atelier.item
schema_version: 3
id: atelier-6ve6
title: Give the replay judge the base-tree files a finding cites
status: in_review
mission: atelier-kw8u
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A3a9fab39-7f29-4732-8284-d6c07f2deac6%3Adelegate-task%3Ab52bddab-6ee5-4816-8fd4-a3f789c84027
ready:
  by: 'the lead of atelier-kw8u: both change what the replayed reviews measure, so they come before the matrix'
  at: 2026-11-19T02:52:02.027431283Z
trivial: null
caused_by: null
created_at: 2026-11-19T02:50:32.636385898Z
---

## Goal

Taken from deferred finding atelier-z8rg/1-yrne.2, in .atelier/items/atelier-z8rg/reviews/1-yrne.md:

> Finding 2: Give the replay judge the source material needed to assess other findings
> Recommendation: defer
>
> `evals/agent-factory/results/2026-11-18-nine-more-replays.md:58-63` identifies a real limitation, not merely a disagreement with the judge. The retained merge-sync review cites ADR 0019 decision 3 and standards.md's prohibition on Atelier creating commits. I verified both in the fixture's base tree. `replay.judge_context` supplies only the causing and fixing diffs, which omit those unchanged governing lines. The judge consequently labels the finding unsupported because it sees the new receipt-commit contract but not the existing prohibition.
>
> In a shared-harness follow-up, include the item contract and the base-tree material cited by findings, or distinguish missing evidence from unsupported findings. Otherwise a valid finding outside the two diffs can inflate the reported unsupported count. The current report preserves the judge's answer and explains the discrepancy, so its requested smoke results remain reviewable and no change to this item's cases is needed before merge.

## Done when

- [x] The replay judge can read the base tree's files that a finding cites, or answers `unknown` rather than `fail` where the cited material is not given to it.
- [x] Re-judging the kept `merge-sync` run (`harness/judge.py`) answers `others-supported` with 0 of 3 unsupported.
- [x] A test in `replay_tests.py` covers a finding supported only by an unchanged base file.

## Scope

## Report

The judge of a replay case now reads the item and the lines of each file a
review cites, as the submitted branch held them, so a finding that rests on
a rule the change leaves alone can be checked.

- `replay.cited()` finds each path a review names (`path`, `path:N` or
  `path:N-M`, matched as written or as the end of one path in the tree)
  and gives the cited lines with 15 lines each side, or the whole file up
  to 12k characters where no line is named. It reads them at
  `.branch_head`, so an untouched file is the base tree's. It also gives
  the item's record, since `others-supported` asks about the Done when and
  the judge had never been given it. The section is capped at 60k
  characters. `judge.material()` adds it for replay cases only; other
  cases' prompts are unchanged.
- `replay-rubric.md`'s `others-supported` now says the cited files are
  given, and to answer unknown, naming the finding, where a finding rests
  on material the judge was not given.
- Re-judging the kept run
  (`harness/judge.py ~/.cache/af-evals/runs/z8rg-replay/20261118-210518-merge-sync-skill-sol-0-9049`)
  answered `others-supported` pass, "0 of 3 unsupported", quoting
  PRODUCT_INTENT.md's "Atelier never creates commits" and naming
  standards.md:30 and ADR 0019 decision 3. `found` stayed fail, as before.
  The judge took 11 seconds and 32k tokens in. Its `rejudge.json` is
  beside the run's `result.json`.
- `replay_tests.py` has
  `test_a_finding_that_rests_on_an_unchanged_base_file_can_be_checked`.
  It picks a line of a `docs/` file the change does not touch and no diff
  holds, cites it in a fix-now finding, and checks that the line reaches
  the prompt and that a pass quoting it is accepted. Without the cited
  section the same quote is answered unknown.
- `evals/agent-factory/test.sh` passes with atelier 0.7.0: the harness's
  83 tests and every scenario's, the new test in each of the 12 replay
  cases.
- The README's replay section and the nine-replays result note describe
  the change.

A path a review names only in words, such as "ADR 0019" with no file, is
not found. The rubric's unknown line covers that case.
