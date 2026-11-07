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
updated_at: "2026-11-07T17:21:52.753591621+00:00"
---

## Goal

The agent-factory skill (spadaval/.agents, `skills/agent-factory`) is about 2,220 lines an agent can be sent to read, not counting the constitution (181), `maintainers/` (267) or the evals. Much of it says the same thing in several places, restates what Atelier's CLI already enforces and lists, or patches edge cases; the simpler-lifecycle mission (atelier-mmpv) made one file longer (references/atelier.md 198 to 234 lines). Sumeet's goal, 2026-11-07: minimise the skill as far as possible while keeping the basic instructions a (sub)agent needs to carry out its role.

## Done when

- [ ] The agent-facing skill (everything under `skills/agent-factory` except `constitution.md` and `maintainers/`) is at most 1,000 lines, and SKILL.md at most 100, counted at the current ~80-column wrap.
- [ ] Each role's read path is at most 300 lines, and the orchestrator's at most 450. A role is each subskill that remains; its read path is SKILL.md, its procedure, and every file the procedure links (counted whole, conditional links included), plus the Atelier reference for every role that runs `atelier` commands. A table in the Report gives each role's path and count at 4d88ffb (recounted by this rule) and after.
- [ ] Nothing on an agent's path links to or tells it to read the constitution, which is a manifesto about the skill, not part of it. The one rule agents need from it (strategy changes only by human direction or authority written into the strategy) is stated once, in SKILL.md's core rules, and the four links to it from plan, orchestrate, decide and strategic-plans go.
- [ ] Each rule has one home: no rule is stated in two files, and nothing restates what `atelier --help`, `atelier <cmd> --help` or a refusal message already tells the agent (record sections, flags, transition names); the skill points to the help instead. Where the skill now relies on the help, the Report names the help text it relies on, confirmed to exist.
- [ ] The four eval scenarios (`evals/agent-factory/scenarios`: refused-land-push, vacuous-test-review, done-when-rewrite, trivial-typo) run on the skill arm, 3 runs each with Opus and with Sonnet, at the old skill (main 4d88ffb) and the new one, with the same installed `atelier` binary, every run read by hand: the new skill passes at least as many runs per scenario and model as the old. If a cell drops, read the failing run: if removed guidance caused it, restore that guidance; if not, rerun that cell at 6 runs per arm and compare again. Results are recorded in `evals/agent-factory/results/`.
- [ ] The mission-shaping micro-test (`evals/agent-factory/results/2026-11-04-mission-shaping.md`, its input file and prompt) is rerun old skill against new, 2 runs each with Opus and Sonnet, read by hand: the new skill keeps the behaviours that test showed (a transcript measurement is not made a mission criterion; criteria do not restate single items). Recorded alongside.
- [ ] A fresh agent given only the new skill, astromech's AGENTS.md and the CLI help answers, with no contradiction found: the lifecycle order, what makes an approval stale and when to file a loose end (atelier-mmpv's legible criterion); and, as an orchestrator, who arranges review, what the reviewer is given and not given, who decides findings on an author's own work, and who records the approval.

## Scope

`skills/agent-factory` in spadaval/.agents (edited in a worktree, `git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/<id> -b <id> main`; the live checkout never changes branch), its evals results, and this item's records. Subskills may be merged, shortened or removed if no role needs them; say which and why. `constitution.md` and `maintainers/` are not edited except to fix links. Not Atelier's code or docs. Do not reinstall `atelier` between the old and new eval runs.

## Context

The eval harness: `evals/agent-factory/README.md`; method: `skills/agent-factory/maintainers/skill-evaluation.md` (baseline first, read every run by hand). The old-skill runs of the four scenarios are being taken by the orchestrator now, into `~/.cache/af-evals/runs/qj9d-baseline/{opus,sonnet}`. Current file sizes: tracker-records.md 276, atelier.md 234, SKILL.md 191, orchestrate.md 185, plan.md 162, strategic-plans.md 161, branches-and-worktrees.md 121, finding-disposition.md 105, repository-shape.md 89, implement.md 85.
