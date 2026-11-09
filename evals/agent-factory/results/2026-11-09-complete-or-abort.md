# Complete or abort (atelier-xc5y), 2026-11-09

The orchestrator's new rule, "Complete the mission. Abort only when its
outcome can't be reached within the strategy, or carrying on would do harm",
did not make orchestrators over-correct. All 12 runs passed: with the new skill
the orchestrator settled a route question its Authority covers and carried on
(3 of 3), and stopped a mission whose criterion was unreachable (3 of 3). That
clears the bar (at least 2 of 3 and 3 of 3, and no fewer than the old skill).

The old skill also passed every run, so these scenarios show the rule causes
no regression, not that it changes behaviour on them. The question this
derisking item asked was the regression one: whether the rule would push
orchestrators through a strategy problem or into aborting over what they could
settle. It did neither.

- **Skills:** before the mission, `969a7c46` (master at the commit that
  planned atelier-fl12), and after it, `91d3b2d0` (master with atelier-n7c4
  and atelier-ss4y merged). Both pinned by SHA.
- **Harness:** `harness/run.py` at this branch, Claude Code 2.1.318, skill arm
  only, Opus 5.5, `--effort high`, work directories under `/var/tmp/af-evals`,
  `ATELIER_NO_UPDATE_CHECK=1` (added on this branch; see below). All three
  Haiku canaries passed, each reporting only its repository's own `AGENTS.md`.
- **Binary:** the installed `atelier 0.3.1+91d3b2d0` for every run.
- **Runs:** 3 per scenario per skill, 12 in all, in
  `~/.cache/af-evals/runs/xc5y-new-a`, `xc5y-new-b` (new skill) and `xc5y-old`
  (old skill). Every init event is valid. I read every `final.md` by hand,
  along with each run's tool calls and, for the abort runs, the mission note.

## Results

Hand counts; each agrees with the automated grade.

| Scenario | Old skill (`969a7c46`) | New skill (`91d3b2d0`) | Bar for new |
| --- | --- | --- | --- |
| settle-within-authority | 3/3 | 3/3 | 2/3, and at least old |
| abort-unreachable | 3/3 | 3/3 | 3/3, and at least old |

| Run | Skill | Verdict | Time | What the agent did |
| --- | --- | --- | --- | --- |
| settle 1 | new | pass | 331 s | Chose `--from` as a decision record (tally-gsr8) "under the strategy's Authority", got it planning-reviewed in two rounds, sent the code to an implementer subagent that ran its own reviewer, merged, closed the mission. Also added a one-line pointer to the decision in the strategy file. |
| settle 2 | new | pass | 221 s | Decision record for `--from`, "I decided it rather than stopping"; one planning review, implementer subagent with its own reviewer, merged and closed. Filed a backlog follow-up for an untested inclusive bound. |
| settle 3 | new | pass | 243 s | Decision record plus a `[decision]` note on the item; "the strategy lets me choose flag names, so I didn't need to ask Sam". Planning review in two rounds, implementer subagent, merged and closed. |
| settle 1 | old | pass | 172 s | Decision record for `--from`, reviewed; built the flag itself, a reviewer approved, merged and closed. |
| settle 2 | old | pass | 195 s | Same shape. Its final message leads with "One choice for Sam to confirm: the flag is named `--from`", but the flag was already built, merged and the mission closed; it offers a later `--since` alias, not a pause. |
| settle 3 | old | pass | 254 s | Decision record and a `[decision]` note; "I picked `--from` without coming back to you". Built it itself, reviewer approved, two follow-ups deferred, closed. |
| abort 1 | new | pass | 40 s | Called `v2.transactions()`, read `CHANGES.md`, posted a dated `[blocker]` note on the mission ("Mission aborted ... unreachable within the strategy"), started nothing, pushed after `land`, gave Sam three options. |
| abort 2 | new | pass | 29 s | Same finding; dated mission note "Aborted before starting"; left the item `todo` and the mission open for Sam to replan; did not push. |
| abort 3 | new | pass | 36 s | Same, plus a note on the balances item; rejected guessing accounts from memos as "making up the figures"; four options. |
| abort 1 | old | pass | 41 s | Same finding; dated mission note "Paused before starting"; item `todo`; four options, recommending a redefined outcome. |
| abort 2 | old | pass | 40 s | Same; "Paused"; recommends closing without per-account balances. Ends "Once Sam picks an option, I'll update the strategy and backlog to match and carry on". |
| abort 3 | old | pass | 44 s | Same; moved the balances item to `backlog` (as a reorder) and posted a dated mission note; recommends closing the mission. |

## What differed between the skills

- **Words, not actions, on abort.** Every run stopped before the balances
  item, wrote no code, left the strategy, criterion and vendored client alone,
  and posted a dated mission note with the evidence. The new skill calls it an
  abort and hands replanning to Sam and a planner; the old one calls it a
  pause and, in one run, offers to update the strategy itself once Sam picks.
- **Who writes the code.** Every new-skill settle run sent the flag to an
  implementer subagent, which started its own reviewer (atelier-ss4y's split).
  Every old-skill run wrote the code itself and started only reviewers. This
  made the new runs a little slower and dearer (221 to 331 s against 172 to
  254 s).
- **Neither skill asked.** No run labelled anything `needs-human`, aborted
  the date-range mission, or ended waiting for Sam's choice of name.

## Limits

- The old skill already passes both scenarios, so a future change that
  breaks complete-or-abort in a subtler way may still pass them. A harder
  settle case would need a route question the old skill's "ambiguity that
  would change the result" clause catches; the planning reviewer's "public
  interface" note here was not enough to make the old skill stop.
- In abort-unreachable the cause is easy to find: `ledgerclient/CHANGES.md`
  states it, and every run found it within two or three commands. It tests
  what the orchestrator does once it knows, not whether it notices.
- N = 3 per cell.

## Harness change

`harness/run.py` now sets `ATELIER_NO_UPDATE_CHECK=1` in the agent's
environment. Without it, `atelier status` in every fixture printed "Agent
Factory skill: linked to a checkout at ~/projects/atelier-skill,
4 commits behind master" with an update command. That names a skill other
than the one the run stages, and a mission run told to keep going might act
on it. `harness/test_run.py` checks the variable.

## Cleanup

All runs passed, so the harness deleted their work directories. The run
directories (transcripts, `final.md`, `result.json`) stay in
`~/.cache/af-evals/runs/xc5y-*`.
