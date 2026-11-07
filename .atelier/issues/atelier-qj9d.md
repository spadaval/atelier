---
created_at: "2026-11-07T17:18:36.057434340+00:00"
id: "atelier-qj9d"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Cut the agent-factory skill to what a (sub)agent needs to do its role"
updated_at: "2026-11-07T17:18:36.057434340+00:00"
---

## Goal

The agent-factory skill (spadaval/.agents, `skills/agent-factory`) is about 2,220 lines an agent can be sent to read, not counting the constitution (181), `maintainers/` (267) or the evals. On a task an implementer reads about 600 of them and an orchestrator about 1,400. Much of it says the same thing in several places, restates what Atelier's CLI already enforces and lists, or patches edge cases; the simpler-lifecycle mission (atelier-mmpv) made one file longer (references/atelier.md 198 to 234 lines). Sumeet's goal, 2026-11-07: minimise the skill as far as possible while keeping the basic instructions a (sub)agent needs to carry out its role.

## Done when

- [ ] The agent-facing skill (everything under `skills/agent-factory` except `constitution.md` and `maintainers/`) is at most 1,000 lines, and SKILL.md at most 100.
- [ ] Each role's read path, meaning SKILL.md plus its procedure plus every reference that procedure tells it to read, is at most 300 lines, and the orchestrator's at most 450; a table in the Report gives each role's path and count before and after.
- [ ] Nothing on an agent's path sends it to the constitution, which is a manifesto about the skill, not part of it; the one rule agents need from it (strategy changes only by human direction or authority written into the strategy) is stated where it is used.
- [ ] Each rule has one home: no rule is stated in two files, and nothing restates what `atelier --help`, `atelier <cmd> --help` or a refusal message already tells the agent (record sections, flags, transition names); the skill points to the help instead.
- [ ] The four eval scenarios (`evals/agent-factory/scenarios`: refused-land-push, vacuous-test-review, done-when-rewrite, trivial-typo) are run on the skill arm, 3 runs each, at the old skill (main before this change) and the new one, with the same model, and every run is read by hand: the new skill passes at least as many runs per scenario as the old. The runs are recorded in `evals/agent-factory/results/`.
- [ ] A fresh agent given only the new skill, astromech's AGENTS.md and the CLI help states the lifecycle order, what makes an approval stale and when to file a loose end, and finds no contradiction (the legible criterion of atelier-mmpv still holds).

## Scope

`skills/agent-factory` in spadaval/.agents (edited in a worktree, `git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), its evals results, and this item's records. Subskills may be merged, shortened or removed if no role needs them; say which and why. `constitution.md` and `maintainers/` are not edited except to fix links. Not Atelier's code or docs.

## Context

The eval harness and how to run it: `evals/agent-factory/README.md`; method: `skills/agent-factory/maintainers/skill-evaluation.md` (baseline first, read every run by hand). Earlier recorded runs: `evals/agent-factory/results/`. Current file sizes: tracker-records.md 276, atelier.md 234, SKILL.md 191, orchestrate.md 185, plan.md 162, strategic-plans.md 161, branches-and-worktrees.md 121, finding-disposition.md 105, repository-shape.md 89, implement.md 85.
