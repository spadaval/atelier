---
created_at: "2026-06-08T17:33:27+00:00"
id: "atelier-000t"
issue_type: "task"
labels:
- "locks"
- "spec"
- "worktree"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-000f"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide how much inherited Chainlink lock sync survives"
updated_at: "2026-06-10T14:51:19.805241515+00:00"
---

## Description

Resolve the SPEC.md open question about Chainlink lock/sync behavior. Specify what becomes claim/worktree association, what remains as lock coordination, and what is removed or deferred.

## Outcome

Outcome was not specified in the legacy issue record.

## Evidence

Evidence was not specified in the legacy issue record.

## Notes

### Resolution

Retire almost all inherited Chainlink lock/sync functionality from Atelier's
public product surface. Normal tracked work uses Git branch/worktree state,
issue claim/work association, local runtime association, and canonical export
freshness checks. Legacy `locks` and `sync` command groups are not core Atelier
commands.

Internal helper code may remain temporarily only where a core workflow still
depends on it during migration. It should not define the user-facing workflow,
durable coordination model, or Agent Factory guidance.

### Rationale

Worktree helpers, agent claims, branch association, and Mission Control all need coherent coordination semantics. Inherited lock sync may be useful, redundant, or misleading after canonical repo-state export exists.
The settled product direction is that Git moves committed Markdown state, while
Atelier records work association and validates freshness. Maintaining a parallel
lock/sync mental model would obscure the simpler repo-state workflow.

### Alternatives Considered

- Preserve Chainlink lock sync largely as-is.
- Replace it with claim/worktree association.
- Keep local locks only.
- Defer lock redesign until after export/rebuild.
