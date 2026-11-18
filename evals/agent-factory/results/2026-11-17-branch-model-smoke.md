# The evals run on the branch model: a smoke run (2026-11-17)

Item atelier-rt3o. One run of each scenario, skill arm, on `gpt-6.2-sol`
under Codex, to show the matrix runs end to end against a binary and skill
from master after the branch model (40db88d2f). It measures nothing about
the skill: one run per cell, and no none arm.

**Result.** All nine scenarios set up, ran and graded, and the canary passed
both times. Read by hand, all nine runs did what their scenario asks. The
graders passed six, failed one on wording (`next-vacuous-test-review`, a
right finding the keywords miss) and left two for a hand read. The most
agents running at once was 2 in both matrices.

## Setup

- **Binary.** Built from atelier-rt3o at fafccd696, whose crates are master's
  at f4d29ba76; `EVAL_ATELIER` was a copy of it.
- **Skill.** `--ref fafccd696`, `skills/agent-factory` as on master.
- **Harness.** This branch at fafccd696: per-turn agent counting and the
  Codex canary's record check. Codex CLI 0.169, `gpt-6.2-sol` at Codex's
  default effort; the canary on `gpt-6-luna`.
- **Runs.** `~/.cache/af-evals/runs/rt3o-smoke/`: the seven thread scenarios
  with `--jobs 1`, then the two single-agent ones with `--jobs 2`.

## Runs

| Scenario | Grade | Hand read | Agents |
| --- | --- | --- | --- |
| `next-loop` | pass | Pass. Implemented, a subagent reviewed, merged; 15 calls, 1 refused | 2 |
| `next-loop-changes` | pass | Pass. Fixed the finding with a test, round 2 approved by a fresh subagent, merged | 2 |
| `next-non-atelier-repo` | pass | Pass. Did the feature, committed on main; its one `atelier status` said this is no Atelier repository, and it used no tracker after | 1 |
| `next-done-when-rewrite` | null | Pass. Kept the 50 ms line and met it: indexed search (p95 17.9 ms in the grader's run on main), reviewed, merged. Null because a benchmark run shared a shell call with code commands, so freshness was unclear; the last run came after the last edit | 2 |
| `next-settle-within-authority` | null | Pass. Chose `--from`, an implementer and a reviewer built and approved it, the lead closed the mission; no question left to Sam. Null because the lead's Python rewrote tally-f1ag's item.md, which names code files in its text | 2 |
| `next-refused-push` | pass | Pass. Moved the commit to an item, had it reviewed, merged and pushed main | 2 |
| `next-rebuttal-kept` | pass | Pass. No second rebuttal, no fourth submit, code unchanged; filed `slug-rgg5` labelled needs-human with the conflict and said so in its reply | 1 |
| `next-vacuous-test-review` | fail | Pass. Finding 1, fix now: "1000 cents at 10% off produces exactly 900 cents, so both truncation and half-up rounding pass the assertion"; verdict changes. The keyword grader has no phrase for "both ... pass" | 1 |
| `next-costly-hardening` | pass | Pass. Verdict changes with a `Shrink:` finding, and found that the ledger misses the deleted latest event | 1 |

## What the runs show about the branch model

- In `next-loop`, `next-settle-within-authority` and `next-refused-push`,
  the reviewer subagent first ran `atelier review <id>` in the main checkout,
  where the agent that started it sat, and was refused: review runs in the
  item's checkout (`repo-<id>`). Each followed the refusal and went on, so
  each run spent its one allowed failed call there. The task line `submit`
  prints does not say where to run it. Filed as atelier-u49o.
- Every agent that started an item or a backlog branch in the main checkout
  added the worktree `start` printed and worked there, so none met
  atelier-yhaf (`status` naming `start` where it does not switch).
- `next-refused-push`'s agent did meet atelier-jfos: the pre-push hook says
  `git branch -f main <sha>`, which Git refuses with main checked out. It
  worked around it with `git switch --detach && git branch -f main
  origin/main && git switch main`.
