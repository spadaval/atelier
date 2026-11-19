---
schema: atelier.review
schema_version: 3
round: 1
commit: a91c63d16994860774be9d5b99d829a30d4a4bfa
reviewers:
- thread:delegated-task:command%3Amcp%3A34321ea1-8f7b-4c9f-b136-cb2c82ed9992%3Adelegate-task%3Ad2f7c6cc-5e7b-4514-bdf8-c3c6a6f7ad4b
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-19T05:38:13.375576396Z
---

## Summary

Complexity: medium. Five new fixtures and three shared harness files add about
3,200 lines, but reuse the existing setup, facts and judge mechanisms.
Value: high. Correct overbuilt cases and controls are needed to answer the
human's question and assess the proposed reviewer passage.
Risk of merging: medium. A mislabeled control can mislead later skill decisions,
although the change is confined to evals and can be replaced readily.

The value warrants the fixture and harness work; I found no substantially
smaller change that supplies the five requested cases and lean references.
The first Done when is not met because control-large still has reproducible
requirement mismatches.

I read the diff against master at a91c63d16994860774be9d5b99d829a30d4a4bfa,
all five fixtures and rubrics, the three lean implementations, shared setup,
facts and tests, README and results. The full
`bash evals/agent-factory/test.sh` run passed, including all five new cases,
their fixture suites and lean-reference checks, and recorded-answer grading
without a model.
`git diff --check master...a91c63d16994860774be9d5b99d829a30d4a4bfa` passed.

I recomputed all 100 selected cached result.json records and inspected their
findings and judge evidence. The reported counts agree, including recognition
29/30 versus 30/30 and blocking 20/30 versus 30/30. Each case and the skill are
unchanged from the revision named for its runs. I did not repeat the live
model calls. The shared harness outside the item's narrow Scope is a reasonable
way to avoid five copies of the same setup and grading tests.

## Finding 1: Make control-large a correct control before using its results
Recommendation: fix now

The first Done when requires every fixture's change to be correct. The
control-large rubric calls its change correct at
`evals/agent-factory/scenarios/control-large/rubric.md:14`, but the fixture
still violates its import contract. I reproduced these against
`evals/agent-factory/scenarios/control-large/fixture/change/budget.py`:

- At line 80, UTF-8 diagnostics count only LF bytes, although the CSV reader
  accepts CR line endings. The bytes
  `b'Date;Payee;Amount\r01.10.2026;Caf\xe9;-1,00\r'` produce exit 1 and
  `FILE:1: not UTF-8`; the bad byte is on line 2.
- At line 91, every field error uses the record's final physical line.
  `Date;Payee;Amount\n31.02.2026;"Cafe\nNord";-1,00\n` reports
  `FILE:3: bad date '31.02.2026'`, although the bad date is on line 2.
- Lines 53-54 impose a twelve-digit amount ceiling absent from the fixture's
  requirements. `01.10.2026;X;-1234567890123,00` is rejected as a bad amount
  even though it follows the stated format. The fixture's test explicitly
  expects this extra restriction.

The results acknowledge remaining defects and stop fixing them because the
control still passes its grade
(`evals/agent-factory/results/2026-11-19-overbuilt-told-apart.md:105`). That
grade tests whether a review asks for less, not whether the control is correct.
It therefore cannot justify relaxing this acceptance criterion. The same
results already show a defect crowding out a sizing finding in overbuilt-layer;
a buggy large control does not isolate the intended distinction.

Make the large fixture and its stated input/diagnostic contract agree, add
coverage for the resolved mismatches, and rerun its two ten-run cells. Update
the rubric, results and Report from that corrected control. Ignoring the
other QIF record fields is explicitly required, so those recorded complaints do not
need to be implemented.
