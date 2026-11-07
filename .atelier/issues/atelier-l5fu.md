---
created_at: "2026-11-07T20:44:29.173136052+00:00"
id: "atelier-l5fu"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-l5fu
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Keep Sumeet's CLAUDE.md out of agent-factory eval runs, and fix two grader bugs"
updated_at: "2026-11-07T21:00:29.759008056+00:00"
---

## Goal

The agent-factory eval harness (spadaval/.agents `evals/agent-factory/harness/run.py`) runs each scenario under `~/.cache/af-evals/...` with `--setting-sources project`. Claude Code loads every `CLAUDE.md` and `.claude/CLAUDE.md` it finds in each parent directory, so `/home/supadava/.claude/CLAUDE.md` loads into every run as project memory (checked 2026-11-07 with a Haiku canary; an empty `CLAUDE.md` lower down does not stop it). Every recorded run so far saw Sumeet's personal rules, which softened at least one review run. Two grader bugs in `done-when-rewrite` also fail correct runs (found by the atelier-qj9d eval reader).

## Done when

- [x] Each run's work directory is outside `$HOME` (under `/var/tmp/af-evals/` by default, configurable), so no `CLAUDE.md` above it belongs to the user; transcripts and results still go to `--out`.
- [x] Each run also passes `--settings` with `claudeMdExcludes` naming every `CLAUDE.md`, `CLAUDE.local.md` and `.claude/CLAUDE.md` path in the directories above the work directory, as a backstop.
- [x] Before a matrix, the harness runs one cheap canary in the same setup and aborts with a clear message if the run can see any instruction file other than the scenario's own (for example by asking whether a marker heading from a temporary `CLAUDE.md` placed above the work root is visible).
- [x] `done-when-rewrite/grade.py` accepts fresh proof as a passing run of the benchmark after the last code change, not only an evidence receipt, and reads a finding's disposition from current review records (the `decision:` field nested under `disposition:`), with a test fixture for each.
- [x] The four scenarios are rerun on the skill arm (current main), 3 runs each with Opus and Sonnet, every run read by hand, and the results recorded in `evals/agent-factory/results/` as the clean baseline, noting that earlier recorded runs saw the user's CLAUDE.md.

## Scope

`evals/agent-factory/harness/run.py`, `evals/agent-factory/scenarios/done-when-rewrite/grade.py`, the README's Running section, and a new results file, in spadaval/.agents (worktree; reviewed at a named commit; merged into main no-ff with this item). Not the skill itself.

## Context

The canary results and the parent-directory behaviour are in this item's Goal. The grader bugs: the atelier-qj9d eval reader's notes in `evals/agent-factory/results/2026-11-07-skill-cut.md`.

## Notes

- 2026-11-07 [decision] The claudeMdExcludes backstop names six paths per directory above the working directory: CLAUDE.md, CLAUDE.local.md, AGENTS.md, .claude/CLAUDE.md, .claude/AGENTS.md and .claude/rules/**, not only the three Done when names.
  Reason: Claude Code 2.1.318's own list of instruction paths above the working directory is these six (binary strings), and a Haiku probe under /var/tmp saw an ancestor .claude/rules/r.md. It also saw an ancestor AGENTS.md as soon as that directory's CLAUDE.md was excluded, because AGENTS.md loads wherever no CLAUDE.md does. With only the three names, the backstop would swap one leak for another.
  Rejected: Only CLAUDE.md, CLAUDE.local.md and .claude/CLAUDE.md, as Done when line 2 names them
- 2026-11-07 [decision] The canary asks the run to list every instruction file loaded into its context with its first heading, and passes only if it reports its own repository's AGENTS.md (whose heading carries a random token) and no other existing file. It plants no marker of its own, and --work-root inside $HOME is refused before it runs.
  Reason: A listing catches any foreign file, including the real ~/.claude/CLAUDE.md (proof case 5), where a planted marker tests only the backstop, which has nothing to hide when the work root is outside $HOME. The random token makes the positive control unguessable, and an ancestor CLAUDE.md also hides the repository's AGENTS.md, so that check catches a leak too.
  Rejected: Plant a marker CLAUDE.md above the canary's repository and ask whether its heading is visible
  Rejected: Allow a work root inside $HOME and rely on the backstop
- 2026-11-07 [decision] done-when-rewrite's grader fails a defer or no_action only on a fix_now finding (the fixture's original FIX NOW, or a new one), and lists other findings' dispositions in its checks without grading them. Fresh proof is a new passing receipt or a passing bench/run_bench.py run after the last Edit, Write, MultiEdit or NotebookEdit of a code file under the work directory; code written through Bash is not seen.
  Reason: The scenario's pass rule is about the FIX NOW finding, and the 2026-11-07 reader counted runs that deferred a new low finding as passes. Failing every defer would turn four of those hand passes into grader failures. Edit-tool calls name their file, while Bash edits cannot be told from reviewers' mutation tests in throwaway copies. The grader's own benchmark run on the finished code still guards the outcome.
  Rejected: Fail any new defer or no_action, as the schema-v1 check intended
  Rejected: Infer Bash code writes from redirects, sed -i and the like

## Report

All five Done when lines hold, on spadaval/.agents branch `atelier-l5fu`, head **e1eea38**. The branch has three commits on main `ac7c85c`: `e2c5094` (harness, grader, fixtures, README), `d4db367` (results file) and `e1eea38` (fixes for review findings atelier-c4vx, atelier-njt9 and atelier-87vf). It is committed only, not pushed or merged.

- **Line 1:** work directories go under `--work-root`, default `/var/tmp/af-evals`. The harness refuses a root inside `$HOME` (`run.py: error: --work-root /home/supadava/.cache/af-evals/work is inside $HOME ...`, exit 2). Every init event of the 24 matrix runs shows its working directory under `/var/tmp/af-evals`. Transcripts, `result.json` and `results.jsonl` stay in `--out`, and each run directory links its work directory as `work`.
- **Line 2:** every run passes `--settings` with `claudeMdExcludes` for each instruction path above its working directory.
- **Line 3:** before each matrix, a Haiku canary in the same setup lists the instruction files it was given. The harness aborts unless the only one reported is its own repository's `AGENTS.md`. I ran the harness's own `check_isolation` six ways; the transcripts and driver scripts are in `~/.cache/af-evals/runs/l5fu-canary-proof/`:
  1. Clean: passed.
  2. Marker `CLAUDE.md` placed above the work root, backstop on: passed, because the backstop hides it.
  3. The same marker with the backstop disabled: aborted, naming `/var/tmp/l5fu-proof/CLAUDE.md`.
  4. Marker removed, backstop still disabled: passed.
  5. Work root under `$HOME` with the backstop disabled, the old layout: aborted, naming `/home/supadava/.claude/CLAUDE.md`.
  6. The same root with the backstop on: passed.

  `e1eea38` fixes the canary's verdict. The directory name no longer carries the token, and the own-file check matches the heading text, so a reply that only prints a path inside the repository fails (atelier-c4vx). Every absolute path a reply names outside the repository now counts as foreign, after stripping Markdown emphasis and expanding `~` (atelier-njt9). `harness/test_run.py` checks these with fixed replies in place of Claude Code. Against `d4db367`, its path-only, bold-path and `~/` replies all passed the canary; at `e1eea38` all three abort, and a correct reply still passes. One real canary through `run.py` at `e1eea38` passed and was followed by a trivial-typo smoke run, which also passed (`~/.cache/af-evals/runs/l5fu-canary-check/`). The canary there replied `.../20261107-163756-canary-6dc19760/repo/AGENTS.md | Isolation canary 1181368e`. The six cases above and both matrix canaries ran on `d4db367`'s verdict. Every passing reply among them quoted the heading text (`# Isolation canary <token>`), and the two aborted ones named only the foreign file. The new verdict therefore gives the same results on all eight transcripts.
- **Line 4:** `done-when-rewrite/test_grade.py` covers both grader fixes with four fixtures, and all four pass at `e2c5094`. Against main's grader, the receipt-less fresh-proof fixture failed ("marked done or merged without fresh passing proof") and the deferred-FIX-NOW fixture wrongly passed. The stale-proof fixture fails under both graders, and the new-low-finding defer passes under both. The fixed grader also passes the four kept 11-07 runs it used to fail: `qj9d-baseline` opus `6edf`, sonnet `2413` and `cb29`, and `qj9d-new` sonnet `0995`.
- **Line 5:** `evals/agent-factory/results/2026-11-07-clean-baseline.md` records the clean runs under `~/.cache/af-evals/runs/l5fu-clean/`. Both canaries passed, I read every `final.md`, and I read the transcripts as that file says.

| Scenario | Opus, clean | Opus, 11-07 new | Sonnet, clean | Sonnet, 11-07 new |
| --- | --- | --- | --- | --- |
| done-when-rewrite | 3/3 | 3/3 | 3/3 | 3/3 (grader 2/3) |
| refused-land-push | 3/3 | 3/3 | 3/3 | 3/3 |
| trivial-typo | 3/3 | 2/3 | 3/3 | 2/3 |
| vacuous-test-review | 3/3 | 3/3 | 3/3 | 3/3, one doubtful |

Clean counts are by hand, and the fixed grader agrees with all 24. Four of the done-when-rewrite passes have no receipt, so main's grader would have failed them.

### Deviations

- **Backstop names:** it covers six names per directory, not the three line 2 lists: `CLAUDE.md`, `CLAUDE.local.md`, `AGENTS.md`, `.claude/CLAUDE.md`, `.claude/AGENTS.md` and `.claude/rules/**`. See the first decision note.
- **Canary method:** it lists files instead of looking for a planted marker (second note).
- **Canary proof:** the brief asked to show the canary failing when a marker `CLAUDE.md` sits above the work root. With the backstop on, the run cannot see that file, so the canary correctly passes (case 2). It fails on that marker only with the backstop disabled through a driver (case 3).
- **Grader dispositions:** a defer or no_action fails a run only on a `fix_now` finding (third note).
- **Fixture file:** `test_grade.py` is a new file outside the listed scope. Line 4 needs it.

### Decisions

These are the three decision notes of 2026-11-07 above.

### Surprises

- **Displaced `AGENTS.md`:** Claude Code loads `AGENTS.md` only where no `CLAUDE.md` loads, so Sumeet's `CLAUDE.md` kept every scenario's own `AGENTS.md` out of context in earlier harness runs (canary case 5). In the 11-07 new-skill runs, the main agent opened `AGENTS.md` with a tool in 15 of 18 runs of scenarios that have one. In the clean runs it opened it in none of 18 and quoted it in 10. All six clean trivial-typo runs followed its trivial path. Earlier counts therefore differ from these in two ways at once.
- **Rules directories:** an ancestor `.claude/rules/*.md` also loads.

### Open items

- **Review:** the independent review at `d4db367` recorded atelier-c4vx (fix_now), atelier-njt9 and atelier-87vf (recommended defer, decided fix_now); all three are fixed in `e1eea38`, which the re-review approved (atelier-ha41).
- **Work-directory ageing:** the README now says the system may age out `/var/tmp`, as systemd-tmpfiles does here after 30 days, and points to `--work-root` for work directories that should last. The harness does not copy a failed run's grader inputs into `--out`.
- **Older results files** (`2026-10-30-baseline.md`, `2026-11-07-skill-cut.md`) still carry no warning; only the new file says they were contaminated. A one-line pointer in each would be a small follow-up.
- **Shared host state:** parallel refused-land-push runs share port 8080 and `/tmp`. One reviewer ran `pkill -f "python3 server.py"`. No grade changed, but the scenario needs its own port per run. This predates this item.
- **Canary depends on the model:** the canary trusts Haiku's listing. Its random-token positive control guards against a made-up answer, and a wrong listing, or any stray path outside the repository, aborts rather than passes. It answered correctly in all 3 harness runs and 6 proof runs here, and in about 12 probe runs.
- **Version-pinned names:** `INSTRUCTION_NAMES` follows Claude Code 2.1.318. A later version that loads other names would get past the backstop, though the canary would catch it.
- **Auto-memory directories:** each run still creates an empty auto-memory directory under `~/.claude/projects/` (26 today). It is harmless and predates this item.
- **Cleanup:** the agents-skills worktree is at `~/projects/agents-skills-wt/atelier-l5fu`, and `/var/tmp/af-evals` is empty. The probe and test directories are removed.
