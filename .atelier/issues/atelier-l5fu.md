---
created_at: "2026-11-07T20:44:29.173136052+00:00"
id: "atelier-l5fu"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Keep Sumeet's CLAUDE.md out of agent-factory eval runs, and fix two grader bugs"
updated_at: "2026-11-07T20:44:29.173136052+00:00"
---

## Goal

The agent-factory eval harness (spadaval/.agents `evals/agent-factory/harness/run.py`) runs each scenario under `~/.cache/af-evals/...` with `--setting-sources project`. Claude Code loads every `CLAUDE.md` and `.claude/CLAUDE.md` it finds in each parent directory, so `/home/supadava/.claude/CLAUDE.md` loads into every run as project memory (checked 2026-11-07 with a Haiku canary; an empty `CLAUDE.md` lower down does not stop it). Every recorded run so far saw Sumeet's personal rules, which softened at least one review run. Two grader bugs in `done-when-rewrite` also fail correct runs (found by the atelier-qj9d eval reader).

## Done when

- [ ] Each run's work directory is outside `$HOME` (under `/var/tmp/af-evals/` by default, configurable), so no `CLAUDE.md` above it belongs to the user; transcripts and results still go to `--out`.
- [ ] Each run also passes `--settings` with `claudeMdExcludes` naming every `CLAUDE.md`, `CLAUDE.local.md` and `.claude/CLAUDE.md` path in the directories above the work directory, as a backstop.
- [ ] Before a matrix, the harness runs one cheap canary in the same setup and aborts with a clear message if the run can see any instruction file other than the scenario's own (for example by asking whether a marker heading from a temporary `CLAUDE.md` placed above the work root is visible).
- [ ] `done-when-rewrite/grade.py` accepts fresh proof as a passing run of the benchmark after the last code change, not only an evidence receipt, and reads a finding's disposition from current review records (the `decision:` field nested under `disposition:`), with a test fixture for each.
- [ ] The four scenarios are rerun on the skill arm (current main), 3 runs each with Opus and Sonnet, every run read by hand, and the results recorded in `evals/agent-factory/results/` as the clean baseline, noting that earlier recorded runs saw the user's CLAUDE.md.

## Scope

`evals/agent-factory/harness/run.py`, `evals/agent-factory/scenarios/done-when-rewrite/grade.py`, the README's Running section, and a new results file, in spadaval/.agents (worktree; reviewed at a named commit; merged into main no-ff with this item). Not the skill itself.

## Context

The canary results and the parent-directory behaviour are in this item's Goal. The grader bugs: the atelier-qj9d eval reader's notes in `evals/agent-factory/results/2026-11-07-skill-cut.md`.
