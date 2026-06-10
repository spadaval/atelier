---
acceptance: []
blocks: []
created_at: "2026-06-08T19:39:14+00:00"
depends_on: []
evidence_required: []
id: "atelier-000z"
issue_type: "task"
labels:
- "agent-factory"
- "epic"
- "migration"
- "mission"
- "tracker"
links: []
parent: null
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Mission: Replace Beads with Atelier for tracking and Agent Factory"
updated_at: "2026-06-08T22:40:41.118784838+00:00"
---


Replace Beads with Atelier in two places: this repository's own work tracking, and the Agent Factory skill's default tracker workflow. This mission is not complete when Atelier merely has feature parity in code; it is complete when Atelier is the live tracker for /root/atelier and Agent Factory can plan, orchestrate, validate, and hand off work through Atelier without Beads.

Scope includes defining the replacement MVP, importing current Beads state, proving command/JSON parity needed by agents, cutting this repo over, updating Agent Factory bindings/docs, and validating real agent-factory workflows against Atelier.

Out of scope for the first cutover: rich UI, direct live agent process supervision, and long-term run/session accounting.

## Acceptance Criteria

Atelier is the configured tracker in AGENTFACTORY.md for /root/atelier; current Beads issues/dependencies/statuses/labels/notes are migrated or explicitly waived; Agent Factory skill docs no longer assume Beads as the only tracker; a real planning/update/closeout workflow is executed through Atelier; Beads data is retained only as an archived fallback; bd doctor is no longer required for normal work in this repo.
