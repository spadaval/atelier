---
acceptance: []
blocks:
- "atelier-000f"
created_at: "2026-06-08T17:33:27+00:00"
depends_on: []
evidence_required: []
id: "atelier-000t"
issue_type: "task"
labels:
- "decision"
- "locks"
- "spec"
- "worktree"
links: []
parent: null
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Decide how much inherited Chainlink lock sync survives"
updated_at: "2026-06-10T14:51:19.805241515+00:00"
---


Resolve the SPEC.md open question about Chainlink lock/sync behavior. Decide what becomes claim/worktree association, what remains as lock coordination, and what is removed or deferred.

## Decision

TODO: choose which inherited lock/sync concepts survive into Atelier.

## Rationale

Worktree helpers, agent claims, branch association, and Mission Control all need coherent coordination semantics. Inherited lock sync may be useful, redundant, or misleading after canonical repo-state export exists.

## Alternatives Considered

- Preserve Chainlink lock sync largely as-is.
- Replace it with claim/worktree association.
- Keep local locks only.
- Defer lock redesign until after export/rebuild.
