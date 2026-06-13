---
created_at: "2026-06-13T03:04:29.899244013+00:00"
id: "atelier-09sx"
issue_type: "bug"
labels:
- "bug"
- "docs"
- "known-gap"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Workflow docs overstate repo-defined workflow implementation"
updated_at: "2026-06-13T03:12:37.812133932+00:00"
---

## Description

The product docs describe a repo-defined workflow system with configured record workflows, states, transitions, validators, hooks, guidance, config reload behavior, and `.atelier/config.toml` policy selection. The current implementation is much narrower: issue and mission transitions use hardcoded state and validator logic; `atelier workflow validate` is an advanced diagnostic over hardcoded builtin validators; only `worktree_setup` hooks read root `atelier.workflow.yaml`. This mismatch can mislead agents into assuming repository workflow policy is enforced when it is not.

## Outcome

- Product and architecture docs clearly distinguish implemented workflow behavior from the target repo-defined workflow vision.
- Normal operator guidance does not imply configured transitions, guidance blocks, before/after transition hooks, or custom workflow state machines are enforced today.
- Any retained workflow vision is explicitly marked as future/deferred work with tracker ownership.
- Tests or residue scans protect against reintroducing misleading normal-workflow claims.

## Evidence

- File-change review shows workflow docs and CLI guidance distinguish current implementation from deferred workflow vision.
- Residue scan artifact classifies references to repo-defined workflows, workflow config, transition hooks, and `workflow validate` as implemented, diagnostic-only, historical, or future/deferred.
- Focused transcript proves normal command help/status surfaces do not promise repo-defined workflow enforcement.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.

## Notes

This is a free-floating known bug, not part of the current proof/evidence/CLI
simplification mission. Do not treat it as selected work unless a future
operator explicitly creates or activates a workflow-configuration cleanup
mission or issue graph.
