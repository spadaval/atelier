---
created_at: "2026-06-23T20:16:36.006376392+00:00"
id: "atelier-e96p"
issue_type: "task"
labels:
- "cli"
- "docs"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-59vp"
  - kind: "issue"
    id: "atelier-ht4k"
  - kind: "issue"
    id: "atelier-krt8"
  - kind: "issue"
    id: "atelier-pguu"
  - kind: "issue"
    id: "atelier-t8ew"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Document fixed mission/epic domain contract"
updated_at: "2026-06-23T20:16:36.006376392+00:00"
---

## Description

Update the product and architecture docs so mission and epic are explicit built-in domain issue types. Capture the fixed rules: missions coordinate work through advances links, epics are the only structural child-work containers, ordinary issue types may be standalone or epic children, and lifecycle remains configurable through workflows.

Also document the workflow system as layered operator guidance instead of relying on man pages as the only explanation. `docs/product/work-model.md` should explain the conceptual model, `docs/product/workflow-configuration.md` should own the complete workflow/action/validator contract, `atelier man <role>` should stay role-specific, `issue transition --options` should explain one issue's live transition choices, and `atelier lint` should remain the correctness check for records and policy.

## Outcome

- Docs explain the domain shape without capabilities or roles, name the allowed hierarchy/link shapes, and state which behavior remains configurable through statuses, transitions, validators, and actions.
- `docs/product/workflow-configuration.md` documents the fixed built-in mission/epic rules, allowed hierarchy shapes, mission `advances` links, branch selection rules, transition execution order, built-in validators, built-in actions, custom context-only links, and error/recovery expectations.
- `docs/product/work-model.md` explains why mission, epic, issue, evidence, workflow validator, and transition action are separate concepts without requiring readers to parse every workflow YAML field.
- Bundle documentation matches the fixed domain model: missions are issue records with `issue_type: mission`, mission scope is authored as `advances`, and bundle hierarchy validation follows the same mission/epic/ordinary-issue rules as `issue create` and `issue update`.
- Product docs distinguish canonical reference docs from role-specific man pages and live command surfaces: docs explain the system, `atelier man <role>` routes the role, `issue transition --options` explains the current issue, and `atelier lint` checks correctness.

## Evidence

- File changes in `docs/product/work-model.md`, `docs/product/workflow-configuration.md`, and bundle documentation describe the fixed mission/epic domain model, allowed hierarchy shapes, `advances` links, workflow actions, workflow validators, and custom context-only links.
- The docs identify which guidance belongs in canonical docs, role-specific `atelier man <role>` output, live `atelier issue transition <id> --options` output, and `atelier lint`.
- `git diff --check -- docs` passes.
- `target/debug/atelier lint atelier-c0qc` passes.
