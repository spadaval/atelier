# Three reviews from atelier's history, replayed (2026-11-18)

Item atelier-1z52. One review of each replay case by the reviewer of `sol`
and of `house`, skill arm, to learn what one such review costs before more
cases are built. It measures nothing about the skill: one run per cell, and
no none arm.

**Result.** Both reviewers found the defect in one case of three, the same
one (`base-done-claims`), with a reproduction. Neither found it in the other
two, where `sol` and `house` both approved `approval-follow-up` with no
findings. The judge found every other fix-now finding supported (7 of 7).
A review took 1 to 3.5 minutes and 0.26M to 0.71M tokens in, most of them
cached. No run was invalid: no transcript showed the fix, and each run's
proxy refused only Codex's own telemetry host.

## Setup

- **Binary.** Built from atelier-1z52 at 8c14d4c16 (master's crates at
  b7d11962b); `EVAL_ATELIER` a copy of it.
- **Skill.** `--ref 8c14d4c16`, `skills/agent-factory` as on master.
- **Harness.** atelier-1z52 at 8c14d4c16: the replay setup and its
  confinement. Codex CLI 0.169; the judge, for both setups, Claude Code
  2.1.325 on `claude-opus-5-5` at high.
- **Reviewers.** `sol`: Codex `gpt-6.1-sol` at high. `house`: Codex
  `gpt-6.1-sol` at xhigh. On a review case the two setups differ only in
  effort.
- **Runs.** `~/.cache/af-evals/runs/1z52-replay/`, `--jobs 1 --keep`, `sol`
  then `house`. Every reviewer built the old workspace with cargo inside its
  sandbox (4 to 7 cargo commands each), offline, in its own `target/`.

## Runs

"Found" is the judge's answer to `found`, confirmed by reading each review.
"Unsupported" is the judge's count for `others-supported`. Minutes are the
reviewer's; tokens are in (cached part) and out. Each judge took 3 to 11
seconds and 12k to 15k tokens in.

| Case | Setup | Verdict | Fix-now findings | Found | Unsupported | Minutes | Tokens in (cached) | Tokens out |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `approval-follow-up` | sol | approve | 0 | no | 0 of 0 | 1.1 | 262k (187k) | 2.1k |
| `approval-follow-up` | house | approve | 0 | no | 0 of 0 | 1.8 | 339k (277k) | 3.6k |
| `base-record-commits` | sol | changes | 2 | no | 0 of 2 | 2.2 | 516k (454k) | 4.7k |
| `base-record-commits` | house | changes | 3 | no | 0 of 3 | 3.4 | 712k (655k) | 7.6k |
| `base-done-claims` | sol | changes | 2 | yes | 0 of 1 | 1.8 | 408k (358k) | 3.9k |
| `base-done-claims` | house | changes | 2 | yes | 0 of 1 | 2.8 | 603k (555k) | 6.7k |

- `approval-follow-up`: both read `allowed_edge`'s new rule, ran the merge
  gate tests (39 pass) and approved. The luna smoke run before the matrix
  wrote that existing records "still stale approval" and took that for
  right: the defect was in view, read as intended.
- `base-record-commits`: both asked for real fixes. Both found that any
  approval's pin is trusted with no independence check, which the original
  round 1 also found, and that a renamed strategy's old path counts as
  unreviewed, which it did not (it found a `./` path mismatch in the same
  lines). `house` added that a pruned approval drops its coverage. None
  said a record-only commit can take a work item to done.
- `base-done-claims`: Finding 1 in both reproduces it with the old binary:
  approve, close, `atelier merge` passes `review.current`, then `land` on the
  base refuses with `land_base_done_claim`. Finding 2 in both (recovery
  advice tells you to `atelier start` an item already done) is what the
  original round 1 also found.
- The `house` judge of `base-record-commits` first answered
  `others-supported` unknown: it quoted a line of the change's diff, which
  the harness checked only against the run's own material. Fixed
  (8fecb9a59); `judge.py` re-judged
  the run: pass, 0 of 3 unsupported, kept in its `rejudge.json`.

## What a larger matrix costs

Wall time per run, setup and judge included, was about 110 s in `sol` and
165 s in `house`. The cases run one at a time, since each reviewer builds
with cargo. Five runs of ten cases like these, one arm:

| Setup | Reviews | Wall time | Reviewer tokens in (uncached) | Reviewer tokens out | Judge tokens in |
| --- | --- | --- | --- | --- | --- |
| `sol` | 50 | about 1.5 hours | about 20M (3M) | about 0.18M | about 0.7M |
| `house` | 50 | about 2.3 hours | about 28M (3M) | about 0.30M | about 0.7M |

Both arms double it. These three diffs are 68 to 218 lines on one
workspace that builds in under a minute here; larger diffs or slower builds
cost more. The reviewer tokens come from the ChatGPT Pro allowance, the
judge's from Claude.
