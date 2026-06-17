---
created_at: "2026-06-17T20:03:01.896722151+00:00"
id: "atelier-7qsr"
issue_type: "epic"
labels:
- "architecture"
- "records"
- "removal"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-98mo"
  children:
  - kind: "issue"
    id: "atelier-a3e7"
  - kind: "issue"
    id: "atelier-aqqc"
  - kind: "issue"
    id: "atelier-uwb6"
  - kind: "issue"
    id: "atelier-v7d0"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Remove first-class plan and milestone records"
updated_at: "2026-06-17T20:03:31.616979507+00:00"
---

## Description

Remove first-class plan and milestone records from the supported v1 product
model. The audit found no committed plan records under `.atelier/plans/` and no
committed milestone records under `.atelier/milestones/`. The live behavior also
mixes durable plan records with bulk graph apply under `atelier plan`, while
milestones exist as latent checkpoint architecture without an active workflow.

Keep plans as ordinary Markdown artifacts referenced from issue, epic, mission,
or evidence bodies when durable intent needs to be preserved. Keep validation
and outcome data on accountable work records and evidence rather than moving it
to first-class milestone/checkpoint records.

## Outcome

- `.atelier/plans/` and `.atelier/milestones/` are no longer canonical
  first-class record directories.
- Plan CRUD/revision/link commands are removed or replaced by ordinary artifact
  references and issue/evidence workflows.
- Milestone/checkpoint record parsing, rendering, projection, relationship, and
  mission display support are removed.
- Product, architecture, storage, and command docs describe issues, epics,
  missions, evidence, and relationships as the supported v1 tracker model.
- Bulk apply no longer creates or links plans or milestones.

## Evidence

- Search transcript proves no public command, help text, docs, record-kind
  registration, projection schema, or tests still expose first-class plan or
  milestone records except historical notes explicitly marked obsolete.
- Focused tests cover removal/rejection of former plan and milestone inputs.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and
  `git diff --check` pass.
