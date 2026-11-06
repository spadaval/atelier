---
created_at: "2026-11-06T21:26:18.664159142+00:00"
id: "atelier-n0qh"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-bqbz"
    type: "related"
  - kind: "issue"
    id: "atelier-n4mj"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Proof has a time: before merge by default, after merge only with a reason"
updated_at: "2026-11-06T21:26:18.664159142+00:00"
---

## Goal

Done when criteria are checked before merge unless marked for after merge, with a reason (needs the deployed or merged system; a device or place no agent can reach; the human's judgment; time or real use) and a nearest stand-in run before merge. Merged work with open after-merge criteria shows as awaiting validation, validation is recorded on the base without a reviewer identity, a failure files a follow-up bug, and a mission does not close with open validation unless carried.

## Done when

Provisional: the proof-timing scenario test passes; moving a criterion to after merge once the item has started is a recorded plan change shown to the reviewer; agent-facing text in Atelier and Agent Factory says post-merge validation is only what cannot be checked before the merge; an ADR records the model.

## Unknowns

Whether after-merge state is a status, a field, or a criterion-level marker; decided with the status and type audit.
