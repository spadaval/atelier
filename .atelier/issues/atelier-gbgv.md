---
created_at: '2026-06-20T04:17:09.579815005+00:00'
id: atelier-gbgv
issue_type: mission
labels:
- cli
- mission
- review
- workflow
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-41h9
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-20T05:36:24.759424352+00:00'
status: closed
title: Make workflow transitions the sole lifecycle authority
updated_at: '2026-06-20T05:36:24.759424352+00:00'
---

## Description

Make configured workflow transitions the sole authority for issue lifecycle closure, merge/integration behavior, and operator next-step guidance. This mission removes special close behavior, separates provider and local-room integration authority, and hardens provider review readiness without introducing a new issue ORM/session layer.

## Outcome

### Constraints

- Do not add compatibility aliases, staged deprecations, fallback command shims, or magic close behavior unless explicitly requested.
- Provider-backed workflows must use provider merge authority; local branch integration remains only as an explicit room/local-first workflow action.
- Use pragmatic reload-after-tracker-mutating-action behavior for now; do not introduce an issue ORM/session layer in this mission.

### Risks

- Removing issue close and changing terminal workflow actions can break existing tests, docs, and recovery text if stale command references remain.
- Provider review behavior depends on Forgejo config, role authors, token scope, and branch state; doctor and tests need focused coverage to avoid late failures.

## Evidence

- Manual check: Help output no longer lists issue close, and invoking issue close is rejected without a compatibility shim.
- Manual check: Provider-mode terminal transition planning uses tracker.commit, branch.push, review.merge, and base sync without local branch_integrate.
- Manual check: Room-mode workflow still supports explicit local branch integration.
- Manual check: atelier status current-work guidance is derived from transition options or routes to them without static invalid lifecycle prompts.
- Manual check: Forgejo review commands/actions share config loading, and doctor reports provider readiness failures with remediation.
- Manual check: Regression coverage proves tracker-mutating pre-actions preserve review fields across transition status writes.

## Notes

### Terminal Notes

- Close reason: Completed all linked transition-authority work, closed parent epic atelier-41h9, merged provider PR 13, and verified terminal mission checks are ready.

Migrated from `.atelier/missions/atelier-gbgv.md` as a declared mission objective issue.
