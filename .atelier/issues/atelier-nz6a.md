---
created_at: "2026-11-08T05:52:15.097702619+00:00"
id: "atelier-nz6a"
issue_type: "task"
labels:
- "agent-factory"
workflow_branch:
  branch_base: master
  work_branch: atelier-nz6a
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-g6rp"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Make the skill step aside outside Atelier repositories, and give its install lines first"
updated_at: "2026-11-08T22:46:55.629916577+00:00"
---

## Goal

Stop the skill loading where it does not apply, and tell a new user how to install both halves. Its description matches most multi-step software work, so it loads in repositories that do not use Atelier.

## Done when

- [ ] The front matter description names Atelier, for example "Use in repositories tracked with Atelier, or to set a repository up with Atelier", and keeps the triggers that matter inside one (missions, delegation, review, diagnosis).
- [ ] The first lines of SKILL.md, before any process guidance, say: run `atelier status`. If it succeeds, continue. If it fails with "Not an Atelier repository", stop using the skill, and offer `procedures/install.md` only if the user asked to set up Atelier. If `atelier` is not found, do the same and give the install lines: atelier's release installer (as in `README.md`) and `npx skills add spadaval/atelier --skill agent-factory -g`.
- [ ] The plan-file fallback for repositories without Atelier is removed from SKILL.md's Scale section and anywhere else that offers it, and `references/atelier.md` and `procedures/install.md` agree with the new opening.
- [ ] `README.md` gives the skill's install line next to the binary's.
- [ ] A new eval scenario, `evals/agent-factory/scenarios/non-atelier-repo`, with setup, prompt and grader: a multi-step task in a git repository with no `.atelier/`. Pass when the agent runs `atelier status`, does not run `atelier init` or create `.atelier/`, and does not follow the skill's process (no issues, no review records). Its README row is added.
- [ ] All scenarios, including the new one, run once each on the skill arm at this branch with Opus, read by hand, with results in `evals/agent-factory/results/`. Each existing scenario passes as it did in the last recorded results, or the Report explains the difference.
- [ ] The skill stays within `atelier-qj9d`'s line limits, or the Report gives the counts.

## Scope

`skills/agent-factory`, `evals/agent-factory` and `README.md`.

## Context

Strategy: docs/strategies/agent-factory-in-atelier.md. The skill stays global; repository-specific guidance stays in the repository's `.atelier/`, `AGENTS.md` and verify skill.
