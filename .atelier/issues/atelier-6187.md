---
created_at: "2026-06-14T06:08:32.387587058+00:00"
id: "atelier-6187"
issue_type: "task"
labels:
- "agent-factory"
- "docs"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1tv8"
  - kind: "issue"
    id: "atelier-5625"
  - kind: "issue"
    id: "atelier-8vyo"
  - kind: "issue"
    id: "atelier-jqcb"
  - kind: "issue"
    id: "atelier-tje5"
  - kind: "issue"
    id: "atelier-zkw6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T06:49:00.044984789+00:00"
status: "done"
title: "Reconcile Agent Factory and Atelier guidance boundary"
updated_at: "2026-06-14T06:49:00.044984789+00:00"
---

## Description

Implement ADR 0006 by simplifying the Agent Factory guidance split for this repository. Agent Factory should retain portable orchestration discipline, AGENTFACTORY.md should remain a thin binding, and repo-specific tactical workflow guidance should route to executable Atelier-owned surfaces.

In scope: AGENTFACTORY.md, Agent Factory skill or binding references that are repository-specific, Atelier prime/status or product docs when they need role-scoped routing language, and documentation that still presents Agent Factory as the primary tactical command cookbook.

Out of scope: implementing every possible role-scoped CLI surface unless needed to remove duplicated guidance; preserving compatibility aliases or old command shims.

## Outcome

- Agent-facing guidance has one clear ownership boundary: Agent Factory coordinates agents, while Atelier explains this repository's live operating state and tactical commands.
- AGENTFACTORY.md and mapped docs cite ADR 0006 and route recurring tracker/workflow guidance to Atelier-owned surfaces instead of duplicating it.
- Any added Atelier guidance surface remains concise, executable, and consistent with docs/product/cli-surface.md.

## Evidence

- Diff inspection or review artifact names each changed guidance file and proves ADR 0006 is referenced where the ownership boundary is enforced.
- Command/help parity transcript or focused review artifact proves the relevant Atelier guidance surface still points to implemented commands.
- `atelier lint` and `atelier export --check` pass after tracker updates.

## Notes

Created from the 2026-06-14 decision to make Atelier the primary repo-owned operational entry point while keeping Agent Factory as portable orchestration discipline.
Sibling Agent Factory guidance tasks should follow this boundary: keep portable
coordination rules in Agent Factory, and route repository-specific tactical
commands, recovery steps, and closeout diagnostics to Atelier-owned surfaces.
