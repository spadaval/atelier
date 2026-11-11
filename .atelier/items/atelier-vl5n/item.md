---
schema: atelier.item
schema_version: 3
id: atelier-vl5n
title: Write the readiness checklist, mapped to the three codebase properties
status: building
mission: atelier-295u
labels:
- redesign
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A1e4a355d-d905-4146-a4d8-89e6984f49f1%3Adelegate-task%3Acfed5884-1a86-461b-9b6a-d4a2a1a326ee
ready:
  by: sumeet, relayed by the task that started the lead
  at: 2026-11-11T17:10:10.404909184Z
trivial: null
caused_by: null
created_at: 2026-11-10T00:56:10.972580408Z
---

## Goal

Give the gardener a short static list of preconditions to check before it reads any numbers: can an agent find its way, build, change, test and launch in this repository. The scorecard shows effects weeks later; the checklist shows the cause on the day.

## Done when

- [ ] `skills/agent-factory/procedures/readiness.md` holds the default checklist: at most twelve lines, grouped under the three properties (easy to navigate, easy and reliable to change, easy to validate). Each line gives the precondition, the command or observation that tests it, and the measure that would show its failure, named as `docs/product.md`'s Scorecard names it or as one of the log's numbers (time in each status, rounds, take-overs, escalations, skips). A candidate with no such measure is not on the list.
- [ ] Every test on the list can be made by an agent without changing the repository, and a command it runs finishes inside ten minutes at `-j 4`. Where the honest test is longer, the line is an observation instead, such as the last CI run on the base.
- [ ] The file says a repository's own lines live in `.atelier/readiness.md` in that repository, in the same form, checked after the defaults, and that a line is added or removed there by a reviewed change. It adds that file to no repository.
- [ ] The list is drawn from Factory's agent-readiness pillars without the team-shaped criteria. The Report names each pillar and what was taken from it or why nothing was.
- [ ] Trial: every line was tested in atelier and in demiurge, read only. The Report holds the result of each line in each (met, not met, could not test), and a line that could not be tested as written was reworded or cut.

## Scope

In: the one file `skills/agent-factory/procedures/readiness.md`.

Out, and a review finding that asks for one of these is escalated, not fixed:

- `garden.md`, `audit.md` and `SKILL.md`. atelier-9dku rewrites the gardener's procedure and links this file from it.
- A command, script or check that runs the checklist. An agent reads the list and runs each line.
- Any change in a product repository, and any fix for a line a repository does not meet. What the trial finds is recorded in the Report for the first runs (atelier-613a).
- Levels, scores or a pass mark. A line is met or not met.

## Starting candidates

The lead may choose the lines (mission Authority); these are a start, to cut and reword freely. Fewer is better.

- Navigate (tool calls before an agent's first edit): `AGENTS.md` is loaded by each harness in use and names the build, test, lint and launch commands; the relative links in `AGENTS.md` and the docs it names resolve; the docs it names describe directories that exist.
- Change (rework; rounds per item; time in `building`): a fresh worktree builds with the documented command and no manual step; format and lint run from one documented command and pass on the base; `.atelier/config.toml` lists them as merge checks.
- Validate (defects found after merge; where each defect was caught; skips): the tests run from one documented command and pass on the base; a merge check or CI runs them on every merge; a verify skill says how to launch the product and prove a feature, and the launch works; `atelier status` reports the clone's hooks installed; a bug is filed as an item with its `caused-by`.

## Context

Mission: `.atelier/missions/atelier-295u/` (design.md: Two layers; Risks, The checklist grows). `atelier check` accepts a file at `.atelier/readiness.md`, and the pre-push hook counts it as code, so it reaches the base only through a reviewed merge.

The trial is read only: use each repository's base checkout or a detached worktree, change and push nothing, and send nothing from a repository to an outside service. Demiurge is at `~/projects/demiurge`.
