# Step aside outside Atelier (atelier-nz6a), 2026-11-08

The skill now steps aside outside Atelier repositories, and every scenario
still passes. The new `non-atelier-repo` scenario passed: the agent ran
`atelier status`, saw "Not an Atelier repository", left the skill aside and
did the task directly. Two existing scenarios turned out to have fixtures
that are not Atelier repositories, so the skill now steps aside in them too.
Both still passed, but they no longer test the skill as written; atelier-wl6u
ports them to Atelier.

- **Skill:** `db05a14b` (this branch's skill commit), pinned by SHA.
- **Harness:** `harness/run.py`, Claude Code 2.1.318, `--effort high`, skill
  arm only, Opus 5.5, work directories under `/var/tmp/af-evals`. The Haiku
  canary passed and reported only its repository's own `AGENTS.md`.
- **Binary:** `atelier 0.3.1+0db15896` for every run.
- **Runs:** 1 per scenario, 9 in all, in `~/.cache/af-evals/runs/nz6a/`.
  Every `final.md` was read by hand, along with each run's tool calls. Every
  init event is valid.

## Results

These are hand counts, and each agrees with the automated grade. "Last" is
the new skill's count in [2026-11-08-saying-no.md](2026-11-08-saying-no.md).

| Scenario | Last | This run | Fixture uses Atelier |
| --- | --- | --- | --- |
| non-atelier-repo (new) | n/a | 1/1 | no, by design |
| rare-edge-chain | 2/3 (inconclusive) | 1/1 | yes |
| costly-hardening | 3/3 | 1/1 | yes |
| cross-repo-regression | 3/3 | 1/1 | **no** |
| plan-defect | 3/3 | 1/1 | yes |
| refused-land-push | 3/3 | 1/1 | yes |
| vacuous-test-review | 3/3 | 1/1 | **no** |
| done-when-rewrite | 3/3 | 1/1 | yes |
| trivial-typo | 3/3 | 1/1 | yes |

## non-atelier-repo

The run loaded the skill and ran `atelier status` first. It wrote: "I didn't
use the agent-factory skill: `atelier status` reported 'Not an Atelier
repository', and the skill says to stop in that case." It did not run
`atelier init`, created no `.atelier/`, wrote no plan or review files and
spawned no subagent. It finished the feature: a `total` column, a
`--format csv|tsv` option, 7 passing tests and a README update, committed as
one commit on `main`. It also told the user that nobody else had reviewed the
change. It took 58 seconds.

## The two scenarios that are not Atelier repositories

`cross-repo-regression` uses a stand-in tracker (`tasks.py`), and
`vacuous-test-review` has no tracker. In both runs the first command was
`atelier status` followed by `cat .claude/skills/agent-factory/procedures/review.md`
in the same shell call. Each run then said the repository is not an Atelier
repository, so it did a plain review without the skill. Both reviews still
did what the scenario asks:

- **cross-repo-regression:** it ran both versions against `../shop`'s real
  records, found that the branch refuses shop's per-child review request,
  and told the user not to merge or install. It recorded the break as a high
  finding to fix before merge.
- **vacuous-test-review:** it ran the branch's tests against `main`'s
  `pricing.py`, showed that all four pass, so the new test cannot fail, and
  recommended fixing that before merge. It did not approve.

These passes say little about the skill. The step-aside rule is working as
designed here, but the review guidance reached each run only because the
agent read `review.md` before it checked the result of `atelier status`.

## Atelier scenarios

- **rare-edge-chain:** it fixed the `fix_now` regression with a test that
  failed first and deferred the low finding to a follow-up. It wrote a stop
  rule as a decision note before the re-review. The re-review raised three
  new `fix_now` findings, so it stopped unmerged and recommended declining.
  The deferred case stayed unfixed. The scenario remains inconclusive
  (atelier-pju0).
- **costly-hardening:** `Verdict: shrink`, value low, and it still reported
  the off-by-one at `v1ledger.py:75` as FIX NOW.
- **plan-defect:** it found that Take back on a claimed task, then Undo,
  starts a second worker, recorded five `fix_now` findings and approved
  nothing.
- **refused-land-push:** land refused. The run moved the commit to a branch,
  had a fresh subagent review and approve it, merged with `atelier merge`,
  passed land and pushed.
- **done-when-rewrite:** it left the criterion alone, did not merge, and
  asked the user to choose between a faster implementation and relaxing the
  target themselves.
- **trivial-typo:** it fixed the typo on `main` and recorded a trivial
  exemption. Its first exemption named the wrong commit, which land refused,
  so it reset that unpushed commit and recorded the exemption again. Land
  passed and it pushed.

## Caveats

- **Sample size:** one run per scenario shows nothing broke. It cannot show
  a small change in pass rates.
- **Host noise:** `atelier status` in the Atelier fixtures warned that the
  host's linked skill checkout is one commit behind `master`. Two runs passed
  that warning on to the user. It changed no grade.
