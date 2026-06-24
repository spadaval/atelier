---
created_at: "2026-06-16T15:46:38.196848154+00:00"
id: "atelier-168m"
issue_type: "task"
labels:
- "branch"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0nv2"
  - kind: "issue"
    id: "atelier-89by"
  - kind: "issue"
    id: "atelier-8jaf"
  - kind: "issue"
    id: "atelier-bkw7"
  - kind: "issue"
    id: "atelier-mnwf"
  - kind: "issue"
    id: "atelier-x03l"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T16:28:17.298621301+00:00"
status: "done"
title: "Define automatic branch lifecycle and merge contract"
updated_at: "2026-06-16T16:28:17.298621301+00:00"
---

## Description

Define the product and configuration contract before implementation changes branch lifecycle behavior. The contract should replace explicit branch setup commands in the normal workflow with lifecycle-owned branch preparation and close-time integration.

## Outcome

- Target-state docs define branch owner derivation: nearest parent epic branch for child issues, issue branch for standalone issues, and epic branch for epics.
- Target-state docs define close behavior: child issue close commits tracker state on the epic branch; standalone issue and epic close commit tracker state and merge the owner branch to base.
- Target-state docs define the default merge strategy as squash merge, while naming configurable alternatives, base branch selection, and branch naming templates.
- Docs explain failure atomicity: close must not leave an item closed when the commit or merge that makes the close durable in the integration branch fails.
- Docs classify `atelier branch for-epic` and similar branch commands as internal, diagnostic, or advanced repair surfaces rather than normal operator workflow.
- Agent Factory guidance is updated or explicitly queued so worker prompts say `atelier start <id>` owns branch preparation.

## Evidence

- Documentation file diff covers `PRODUCT_INTENT.md`, `CONTEXT.md`, `docs/product/work-model.md`, `docs/product/cli-surface.md`, `docs/adr/0007-mission-workspaces-and-epic-review-branches.md`, and Agent Factory routing where relevant.
- Search transcript shows no target-state doc tells routine workers to run `atelier branch for-epic` before starting issue work.
- Review note or evidence record maps the contract to the parent epic Outcome bullets.
- `atelier lint atelier-168m`, `atelier export --check`, and `git diff --check` pass.

## Notes

This is the contract-first blocker. Implementation issues should not invent branch naming, merge strategy, or rollback semantics independently.
