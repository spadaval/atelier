---
created_at: "2026-11-07T05:01:01.441016682+00:00"
id: "atelier-ml3r"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-4nsl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-8bh4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ilba"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l457"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Say the lifecycle once, plainly, in the skill and Atelier's own docs"
updated_at: "2026-11-07T11:19:13.793485566+00:00"
---

## Goal

Make the agent-factory skill, the CLI help and Atelier's own docs describe the new lifecycle in the same words, with one meaning per word and nothing left of what this mission removed. A fresh agent read about 1,000 lines across eight files and still could not state the order; "check" alone meant at least six things. The rollout to the other repositories is atelier-nn67 and follows this item.

## Done when

- [ ] The agent-factory skill (spadaval/.agents, edited in a worktree there, reviewed and merged into its `main`) describes the new lifecycle in its atelier reference and its orchestrate, implement, review, plan and validate procedures: start, implement, tick every Done when line and write the Report, review request, fix and re-review, approve, merge (which marks work done); approvals that only code makes stale; loose ends; the light path; records correctable until they reach the base; a `fix_now` finding resolved by the approval after its fix.
- [ ] Nothing the mission removed is left in the skill: `start --base`, `<issue_type>/<id>` branch names, close before or after review, After merge markers and stand-ins, `carried to`, `fix_now` dispositions with resolution commits, receipts for suite runs, the reviewer-versus-Git-author rule, recorded bases. The skill's total line count falls.
- [ ] check, land, close, review and done each mean one thing across CLI help, the skill and docs, and CONTEXT.md defines each: `atelier check` validates records and merge checks are the policy's merged-tree commands (nothing else is called a check); `land` is the session handoff command; close applies to missions only; review is independent review; done means merged for work and done on the base for decisions, loose ends and validations.
- [ ] Atelier's own AGENTS.md and docs/product describe the lifecycle in the same words as the skill.
- [ ] The legible criterion is ready to be judged: an agent reading only the skill, an AGENTS.md and the CLI help finds one order and no contradiction (checked by the mission's validation, not by this item's author).

## Scope

The agent-factory skill in spadaval/.agents (via `git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/atelier-ml3r -b atelier-ml3r main`; never switch the live checkout's branch), Atelier's CLI help text, CONTEXT.md, AGENTS.md, docs/product. Not other repositories (atelier-nn67).

## Context

docs/adr/0028-simpler-lifecycle.md (decision 8), docs/strategies/simpler-lifecycle.md. Everything else in the mission has merged before this starts, so the skill describes what is built.
