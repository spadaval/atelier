---
created_at: "2026-06-13T17:29:11.073452262+00:00"
id: "atelier-oku1"
issue_type: "task"
labels:
- "architecture"
- "artifact"
- "assignee:root"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-fmri"
  - kind: "issue"
    id: "atelier-y041"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T17:46:59.126452885+00:00"
status: "done"
title: "Record workflow-state architecture decision"
updated_at: "2026-06-13T17:46:59.126452885+00:00"
---

## Description

Capture the high-leverage workflow-state decisions before implementation starts. The artifact must explain why issue status becomes configured workflow state, why .atelier/workflow.yaml is the fixed tracked policy file, why v1 is issue-only, and why the mission rejects compatibility aliases and old-command shims.

## Outcome

- An ADR records the workflow-state, status-category, policy-path, issue-only v1, and no-compatibility decisions with tradeoffs.
- CONTEXT.md defines Workflow, Workflow status, Status category, Transition, Validator, Guidance, Active work, and Abandon in domain language.
- Future workflow implementation issues can cite the ADR and glossary without relying on private chat context.

## Evidence

- Docs diff shows the ADR and CONTEXT.md terminology updates.
- Review notes or evidence classify the decisions and confirm they match the mission scope.
- atelier lint and atelier export --check pass after the artifact updates.
