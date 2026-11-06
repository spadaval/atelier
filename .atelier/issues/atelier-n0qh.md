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
  - kind: "issue"
    id: "atelier-r9hb"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-rkxm"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Proof has a time: before merge by default, after merge only with a reason"
updated_at: "2026-11-06T22:08:52.659559285+00:00"
---

## Goal

Each Done when check is checked before merge unless marked for after merge, with a reason (needs the deployed or merged system; a device or place no agent can reach; the human's judgment; time or real use) and a nearest stand-in run before merge. Merged work with open after-merge checks shows as awaiting validation, validation is recorded on the base without a reviewer identity, a failure files a follow-up bug, and a mission does not close with open validation unless carried.

## Done when

Provisional: the proof-timing scenario test passes; atelier check refuses an after-merge check without a reason; the review request brief lists each after-merge check with its reason and stand-in, and Agent Factory's review text tells the reviewer to flag any that could have run before merge; moving a check to after merge once the item has started is shown to the reviewer through atelier-r9hb; agent-facing text in Atelier and Agent Factory says post-merge validation is only what cannot be checked before the merge; before-merge proof gains no new per-check evidence gate; an ADR records the model.

## Unknowns

Whether after-merge state is a status, a field, or a marker on the Done when check; decided from atelier-rkxm's counts. Recording validation on the base must not trip land's refusal of done claims made on the base (land_base_done_claim): preferably the item is done at merge and only its awaiting-validation marker clears on the base.
