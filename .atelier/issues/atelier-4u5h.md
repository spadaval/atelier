---
created_at: "2026-06-13T21:58:03.421531467+00:00"
id: "atelier-4u5h"
issue_type: "task"
labels:
- "architecture"
- "stabilization"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Extract issue workflow orientation from Agent Factory commands"
updated_at: "2026-06-13T21:58:03.421531467+00:00"
---

## Description

Core mission and ready-work behavior depends on helper functions housed in
`commands::agent_factory`. Mission audit/list code calls Agent Factory helpers
for workflow policy loading, issue done checks, issue status labels, readiness,
and blocker classification. That reverses the intended dependency direction:
product workflow semantics should not live inside one command group's
presentation module.
Audit evidence: `src/commands/mission.rs` calls
`crate::commands::agent_factory::load_issue_workflow_policy`,
`issue_is_done`, `issue_status_label`, `issue_status_category`,
`issue_start_readiness`, and `issue_blocks_work`; the helpers live in
`src/commands/agent_factory.rs`.

## Outcome

- Shared issue workflow/status orientation logic lives in a domain or workflow
  module that mission, status, ready-work, and Agent Factory command surfaces
  can all call.
- Agent Factory command code retains only Agent Factory-specific rendering and
  delegation guidance.
- Mission audit/list behavior and ready-work behavior stay unchanged except
  where the extracted ownership exposes a documented bug.

## Evidence

- Code diff or review artifact shows the new module boundary and names the
  remaining Agent Factory-only helpers.
- Focused mission status/audit/list and ready-work tests pass before and after
  extraction.
- `rg` residue search proves `commands::mission` no longer calls
  `commands::agent_factory` for issue workflow semantics.
- `cargo fmt -- --check`, relevant focused tests, `atelier lint`, and
  `atelier export --check` pass.

## Notes

Audit evidence: `src/commands/mission.rs` calls
`crate::commands::agent_factory::load_issue_workflow_policy`,
`issue_is_done`, `issue_status_label`, `issue_status_category`,
`issue_start_readiness`, and `issue_blocks_work`; the helpers live in
`src/commands/agent_factory.rs`.
