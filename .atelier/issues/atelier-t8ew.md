---
created_at: "2026-06-23T21:04:40.286980477+00:00"
id: "atelier-t8ew"
issue_type: "feature"
labels:
- "cli"
- "human-output"
- "mission"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3js3"
  - kind: "issue"
    id: "atelier-ycj9"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add read-only mission report namespace"
updated_at: "2026-06-23T21:05:33.035541958+00:00"
---

## Description

Add a narrow mission command group for mission-shaped read/report surfaces
without restoring the old mission lifecycle namespace. The target surface is
read-focused: mission status and mission discovery/listing answer mission health
questions, while creation, linking, mutation, lifecycle transitions, and closeout
remain on issue/workflow commands.

This replaces the surprising behavior where `atelier issue status <mission-id>`
silently renders a mission report. Root `atelier status` stays lightweight
checkout orientation and should only signpost mission choices or drill-downs; it
must not absorb the full mission health report.

Non-scope: do not add mission create, mission close, mission link, mission
start, mission update, or compatibility aliases for old lifecycle commands.

## Outcome

- `atelier mission status <mission-id>` is the primary mission health/report
  surface.
- Mission discovery has an obvious read-only surface, such as `atelier mission
  list`, unless `atelier issue table --kind mission` is intentionally retained
  and clearly signposted.
- `atelier status` remains concise checkout/work orientation and points to the
  mission report instead of rendering it inline.
- `atelier issue status <mission-id>` no longer surprises users with an
  unannounced mission report; it is removed, rejected with corrective guidance,
  or explicitly reframed according to the final command contract.
- Command help, role guides, product docs, and command-audit docs distinguish
  the read-only mission report namespace from issue/workflow mutation commands.

## Evidence

- Before/after transcripts cover `atelier mission status <mission-id>`,
  mission discovery/listing, root `atelier status` mission signposting, and the
  final behavior of `atelier issue status <mission-id>`.
- Help/docs examples show mission lifecycle and durable mutation still routed
  through `atelier issue create --issue-type mission`, `atelier issue link`, and
  `atelier issue transition`.
- Focused CLI tests or transcript evidence prove removed mission lifecycle
  commands are not restored as compatibility aliases.
