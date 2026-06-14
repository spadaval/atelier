---
created_at: "2026-06-12T04:58:38.294509848+00:00"
id: "atelier-tcmr"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-40ou"
    type: "advances"
  - kind: "issue"
    id: "atelier-efpk"
    type: "advances"
  - kind: "issue"
    id: "atelier-gjaz"
    type: "advances"
  - kind: "issue"
    id: "atelier-nzy1"
    type: "advances"
  - kind: "issue"
    id: "atelier-v9id"
    type: "validates"
  - kind: "issue"
    id: "atelier-wpyb"
    type: "advances"
  - kind: "issue"
    id: "atelier-ymfl"
    type: "advances"
  - kind: "issue"
    id: "atelier-zue4"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "closed"
title: "Repair CLI workflow rework and validation gaps"
updated_at: "2026-06-13T01:44:05.915180835+00:00"
---

## Intent

Repair CLI workflow rework and validation gaps

## Constraints

- Create new repair issues instead of reopening misleading closed mission work unless the old issue was closed accidentally and has no replacement.
- Use sectioned issue Markdown with Description, Outcome, Evidence, and optional Notes for all new repair work.
- Every repair item must name observable behavior and evidence before it can close.

## Risks

- Reopening old closed issues can obscure audit history and make it harder to see what failed in the previous mission.
- Large rework can sprawl unless grouped under one mission with explicit blockers and validation.

## Validation

- Mission links all repair epics and tasks needed to make the CLI surface, issue section parser, validators, docs, Agent Factory skill, projection freshness, and closeout checks match product intent.
- Agent Factory guidance explains how to write good mission, epic, issue, validation, Outcome, Evidence, and Notes text without prescribing implementation scripts.
- Mission closeout requires a contract audit mapping every mission and epic Outcome line to linked work and attached evidence.
- Mission closeout requires an independent adversarial validation pass by a validation agent that did not implement the slices being validated.
- Mission closeout requires positive and negative command transcripts for each major repaired surface, including old-command absence or replacement behavior.
- Mission closeout requires focused tests, stale/ignored-test inventory, docs/help/Agent Factory guidance parity checks, export/lint/doctor checks, and attached evidence records for each major repair area.
