---
created_at: "2026-06-15T01:14:48.944207222+00:00"
id: "atelier-fb61"
issue_type: "epic"
labels:
- "product"
- "workflow"
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1zfd"
  - kind: "issue"
    id: "atelier-jrtk"
  - kind: "issue"
    id: "atelier-m2nh"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Simplify mission validation workflow"
updated_at: "2026-06-15T01:15:02.315449901+00:00"
---

## Description

Simplify mission validation so a mission proves coordination, intent coverage, and integration readiness without revalidating every child issue or epic after those records already carry their own proof. The mission should remain an accountability boundary, not a second validation bureaucracy.

## Outcome

- Product and validation docs define mission closeout as a thin synthesis over linked work, blockers, tracker health, and explicitly mission-scoped risks.
- Atelier closeout/status behavior stops requiring duplicate mission-level proof when linked epics and issues already carry passing evidence.
- Tests or command transcripts show the simplified workflow still catches open work, open blockers, stale tracker state, and missing issue/epic proof.

## Evidence

- Child issue proof from atelier-1zfd, atelier-jrtk, and atelier-m2nh maps to each epic outcome.
- `target/debug/atelier mission status atelier-rxpr` output demonstrates mission closeout no longer adds redundant validation gates after linked work is proved.
- `target/debug/atelier lint` passes with the simplified mission validation contract documented.
