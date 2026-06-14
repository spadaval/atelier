---
created_at: "2026-06-13T20:37:03.573130806+00:00"
id: "atelier-x45p"
issue_type: "task"
labels:
- "artifact-update"
- "data-model"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-2ehd"
  - kind: "issue"
    id: "atelier-fyrm"
  - kind: "issue"
    id: "atelier-ihz0"
  - kind: "issue"
    id: "atelier-k3vs"
  - kind: "issue"
    id: "atelier-nqp4"
  - kind: "issue"
    id: "atelier-of3h"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T22:52:34.215999765+00:00"
status: "done"
title: "Define canonical record field ownership"
updated_at: "2026-06-13T22:52:34.215999765+00:00"
---

## Description

Create the durable data-model contract for canonical Markdown records. Resolve which fields belong in front matter versus body sections for issues, missions, plans, evidence, milestones, activity, relationships, priority, status, runtime paths, and provenance.
- Contract names required, optional, derived, compatibility-only, and forbidden fields for each canonical record kind.
- The contract explains priority/status vocabulary, relationship bucket direction, evidence metadata placement, and runtime/cache path ownership.
- Implementation tasks can remove weird or duplicated fields without guessing product semantics.
- Documentation file change or ADR file change records the field table and migration rules.
- Manual check artifact classifies representative existing records against the contract with pass/fail/defer notes.
- `atelier lint` and `atelier export --check` command transcripts pass after the artifact update.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
