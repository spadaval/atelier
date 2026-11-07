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
updated_at: "2026-11-07T11:29:57.096645251+00:00"
---

## Goal

Make the agent-factory skill, the CLI help and Atelier's own docs describe the new lifecycle in the same words, with one meaning per word and nothing left of what this mission removed. A fresh agent read about 1,000 lines across eight files and still could not state the order; "check" alone meant at least six things. The rollout to the other repositories is atelier-nn67 and follows this item.

## Done when

- [ ] Before the skill change merges, astromech's CI pin (`ATELIER_REV`) is at or past atelier-4nsl's merge, a commit that reads every record form the installed binary writes (including the light path's version-3 review records) and judges them by this mission's rules, moved through astromech's own process (astromech-5sp5 moved it to 55cf6072 on 2026-11-07 as a stopgap), since the skill, which describes the light path, is live in every repository the moment it merges.
- [ ] The agent-factory skill (spadaval/.agents) describes the new lifecycle in its atelier reference and its orchestrate, implement, review, plan and validate procedures: start, implement, tick every Done when line and write the Report, review request, fix and re-review, approve, merge (which marks work done); approvals that only code makes stale; loose ends; the light path; records correctable until they reach the base; a `fix_now` finding resolved by the approval after its fix. The change is made in a worktree (`git -C ~/projects/agents-skills worktree add ~/projects/agents-skills-wt/atelier-ml3r -b atelier-ml3r main`; never switch the live checkout), the reviewer reviews that branch at a named commit together with this item's Atelier diff, and exactly that commit merges into its `main` (no-ff) when this item merges.
- [ ] Nothing the mission removed is left in the skill or its evals (`evals/agent-factory`): `start --base`, `<issue_type>/<id>` branch names, close before or after review, After merge markers and stand-ins, `carried to`, `fix_now` dispositions with resolution commits, receipts for suite runs, the reviewer-versus-Git-author rule, recorded bases. The skill's total line count falls.
- [ ] One meaning per word, in prose; no flag or policy key is renamed (that would need new shims). CONTEXT.md defines: check = `atelier check` (record validation), merge checks (the policy's merged-tree commands) and `merge --check` (merge gates without merging), and prose uses "line" for a Done when line; land = `atelier land`, the handoff and pushed-commit check; close = what a mission does (the `close_reason` key and `--reason` keep their names and write the Report); review = independent review; done = the status work gets when it merges, decisions, loose ends and validations get on the base, and a closed mission has, with superseded as the other terminal status. CLI help and docs use the words that way.
- [ ] `start`'s detached-HEAD refusal suggests `git switch -c <id> <base> && atelier start <id>`, which works in a linked worktree, and docs/product/work-model.md no longer says old guidance passing `--base` "does not fail" (finding atelier-ywpf).
- [ ] ADR 0028 (decisions 2, 3 and 5, Consequences), docs/product/workflow-configuration.md and docs/product/work-model.md name atelier-nn67, not this item, for astromech's pin, the `branch_base` drop, astromech's validation items and the other repositories' policies.
- [ ] Atelier's own AGENTS.md and docs/product describe the lifecycle in the same words as the skill.
- [ ] A fresh subagent given only the rewritten skill, Atelier's AGENTS.md and the CLI help states the lifecycle order, what makes an approval stale and when to file a loose end, and reports no contradiction; its report is attached as a note. (The legible criterion itself is judged by the mission's validation after atelier-nn67, on astromech's AGENTS.md.)

## Scope

The agent-factory skill and its evals in spadaval/.agents, Atelier's CLI help text and `start`'s detached-HEAD hint, CONTEXT.md, AGENTS.md, docs/product, ADR 0028. astromech only for its CI pin, if it needs moving again. Not the other repositories' policies or AGENTS.md (atelier-nn67).

## Context

docs/adr/0028-simpler-lifecycle.md (decision 8), docs/strategies/simpler-lifecycle.md. Everything else in the mission except atelier-nn67 has merged before this starts, so the skill describes what is built.
