---
created_at: "2026-06-16T16:18:22.229943934+00:00"
id: "atelier-jezn"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-m1r7"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T17:44:07.351994347+00:00"
status: "done"
title: "Remove export checks from normal guidance"
updated_at: "2026-06-16T17:44:07.351994347+00:00"
---

## Description

Clean up the guidance that teaches agents to run export checks as ordinary proof. Agent Factory and validation docs should point workers toward claim-specific proof, `lint`, `doctor`, and command-specific readiness checks.

## Outcome

- `AGENTFACTORY.md` no longer includes export checks in normal stale-state preflight, mission closeout, or generic handoff recipes.
- `/root/.agents/skills/agent-factory` tracker, plan, implement, validate, and related guidance no longer teaches export checks as ordinary workflow proof.
- `docs/architecture/quality/validation.md`, role guides, command audits, and issue-authoring examples use `lint`, `doctor`, focused tests, transcripts, and evidence records instead of export checks unless the issue specifically changes storage rendering.
- Existing open tracker issues created for unrelated work are not rewritten broadly, but new command-surface issue templates and examples stop adding export checks by default.
- Any remaining references to export checks are classified as historical, migration/debug, or storage-rendering-specific.

## Evidence

- Documentation file diff covers `AGENTFACTORY.md`, `/root/.agents/skills/agent-factory`, validation docs, role guides, and command-audit docs where they teach routine checks.
- Search command transcript classifies each remaining `export --check` reference as historical, migration/debug, storage-rendering-specific, or follow-up.
- Review artifact confirms generic issue Evidence boilerplate no longer includes export checks by default.
- `atelier lint atelier-jezn`, `atelier doctor`, docs whitespace check, and `git diff --check` pass.

## Notes

Do not remove export references from historical evidence logs or closed activity sidecars merely to make a search clean. Classification is enough there.
