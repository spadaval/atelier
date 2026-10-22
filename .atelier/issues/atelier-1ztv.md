---
created_at: "2026-10-22T19:58:07.454492914+00:00"
id: "atelier-1ztv"
issue_type: "feature"
labels:
- "land"
- "merge"
- "review"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-as7i"
  - kind: "issue"
    id: "atelier-iaas"
  - kind: "issue"
    id: "atelier-n642"
  - kind: "issue"
    id: "atelier-ss25"
  - kind: "issue"
    id: "atelier-uw4x"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make land and merge agree on what review covers"
updated_at: "2026-10-22T19:58:16.616973437+00:00"
---

## Goal

`land` and `merge` apply one rule for what an approval covers. A record-only close commit keeps the approval current. Filing a follow-up on an approved branch keeps it too. Merge refuses, before integrating, any done claim that land would refuse. A planning commit that adds an issue to a mission re-checks no old done claims. A review record created inside a merge commit is found.

## Done when

- Each child's Done when holds, shown by its tests, with one independent review per child or per batch of children.
- `atelier doctor` passes on brewer-corp/astromech at 649dc8ec (atelier-as7i).
- Delivers atelier-k8h8 S7 (atelier-n642, atelier-iaas, atelier-ss25) and the gate-agreement part of S11 (atelier-uw4x, atelier-as7i).

## Unknowns

atelier-iaas offers two routes. Prefer the one that matches atelier-n642: records new on the branch that are not the item's own leave the approval current. Record the choice as a decision note.

## Background

The children all change the review gate: `crates/atelier-app/src/review_gate.rs` (introduced and scope queries, approval currency), `crates/atelier-cli/src/commands/git.rs` (`land_base_done_claim`) and `touched_claims` in `crates/atelier-cli/src/commands/session.rs`. They share one branch and run in order:
1. atelier-as7i first, because astromech's land and doctor are broken today.
2. atelier-n642 and atelier-uw4x as one rule. Once a close-only commit stays covered, land and merge accept it the same way, and merge refuses every other done claim land would refuse. State the rule once in `docs/product/`.
3. atelier-iaas.
4. atelier-ss25.
atelier-7b6e (batched history walks) follows atelier-as7i on its own branch.
