---
acceptance: []
blocks:
- "atelier-0006"
- "atelier-000h"
- "atelier-001n"
- "atelier-001t"
- "atelier-0022"
created_at: "2026-06-08T17:33:27+00:00"
depends_on:
- "atelier-000u"
- "atelier-0023"
evidence_required: []
id: "atelier-0005"
issue_type: "task"
labels:
- "domain-model"
- "feature"
- "links"
- "spec"
links: []
parent: "atelier-000j"
priority: "P2"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Implement atelier link add/remove/list with validation"
updated_at: "2026-06-10T00:23:25.579173201+00:00"
---

Support explicit typed links through a preferred generic `atelier link add/remove/list` surface after the project-scoped random ID cutover. Links are the semantic graph that connects first-class records; dependencies remain true sequencing only.

## Scope

- Support directed links across record kinds, including missions, milestones, plans, evidence, epics, and issues.
- Define record references with explicit kind and the single project-scoped random record ID so links are not limited to issue-to-issue rows.
- Keep `blocks`, `blocked_by`, and `depends_on` normalized to dependency semantics where appropriate.
- Support mission-centered relationship types such as `advances`, `has_checkpoint`, `contributes_to`, `planned_by`, `validates`, `evidenced_by`, `implements`, `part_of`, `supersedes`, `derived_from`, `duplicates`, and `related`.
- Replace inherited `issue relate/unrelate/related` behavior with `atelier link`; do not maintain issue-only relation aliases as a parallel public implementation after cutover.

## Acceptance

Typed links are persisted, shown, exported, rebuilt, and linted; dependency aliases remain reserved for true sequencing; links can connect missions to epics/issues, missions to milestone checkpoints, plans to missions/milestones/issues, and evidence to validation targets; `atelier link` is the public surface; invalid or unknown relation behavior is documented and tested.
