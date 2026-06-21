---
created_at: '2026-06-19T22:42:56.490398779+00:00'
id: atelier-v6et
issue_type: mission
labels:
- mission
- workflow-policy
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-33a4
    type: advances
  - kind: issue
    id: atelier-4xue
    type: advances
  - kind: issue
    id: atelier-cin6
    type: advances
  - kind: issue
    id: atelier-ji9c
    type: advances
  - kind: issue
    id: atelier-kmmv
    type: advances
  - kind: issue
    id: atelier-yhui
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-20T02:35:22.759835850+00:00'
status: closed
title: Workflow policy cleanup and custom issue types
updated_at: '2026-06-20T02:35:22.759835850+00:00'
---

## Description

Clean up Atelier workflow policy so repository-defined workflow state is explicit and transition-time behavior is declared. Scope includes custom issue types, status/category semantics, namespaced validators, replacing effects with actions, actual review completion validation, workflow-driven branching actions, and migration of `.atelier/workflow.yaml` plus docs/tests. Avoid generic capabilities and avoid a separate branch-policy action system.

## Outcome

### Constraints

- Do not add compatibility aliases, fallback support for removed workflow syntax, or a staged deprecation path unless a human explicitly asks for it.
- Express branch lifecycle behavior through workflow transition actions, not a separate branch-policy action system.
- Keep validators read-only; transition-time mutations belong in declared built-in actions.

### Risks

- Workflow schema, tracker records, docs, starter policy, and branch/review behavior must migrate together; partial migration can strand agents on stale workflow guidance.
- The config/workflow ownership boundary can leak project-local runtime paths or provider policy back into tracked config unless it is documented before implementation.

## Evidence

- Manual check: `atelier workflow check` passes against the migrated repository workflow policy.
- Manual check: `atelier lint` passes after the mission records, workflow config, docs, and starter policy are updated.
- Manual check: Focused CLI integration tests prove custom issue types, namespaced validators, transition actions, review completion validation, git branch actions, and migrated status/category output work together.
- Manual check: The terminal validation issue `atelier-4xue` records evidence that maps the mission requirements to changed files, command output, tests, and remaining risks.

## Notes

### Terminal Notes

- Close reason: All linked workflow policy cleanup issues are done; terminal validation evidence atelier-68v5 and PR 12 prove the final state

Migrated from `.atelier/missions/atelier-v6et.md` as a declared mission objective issue.
