---
created_at: "2026-11-07T11:19:13.110128252+00:00"
id: "atelier-nn67"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-ml3r"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move every Atelier repository on this host to the new lifecycle"
updated_at: "2026-11-07T11:29:57.442362238+00:00"
---

## Goal

atelier, astromech, cosplay-studio, demiurge and life-on-rails share one installed binary but keep their own policies, records and AGENTS.md. Move the four other repositories onto the new lifecycle, so the every-repo criterion holds, and then remove the two compatibility shims that exist only for them: the `branch_base` field still written into ownership records, and `start`'s ignored `--base` flag. Old-form reads that historical records need stay.

## Done when

- [ ] Each repository's move is carried by one issue in that repository (or the light path, where its CI pin is past atelier-4nsl), through its own review and merge, in this order: first, every item in a status the new policy lacks (`backlog` validations under an on-base workflow, `blocked`, `review`) moves to `todo` or is superseded, and no play-check needs doing for this (finding atelier-b0uv; astromech's play-check validations stay `validation` items, not loose ends); then its policy takes the starter's shape (`merge_commit`, no `close` for work, `loose_end` and `validation` on the on-base workflow); then `atelier migrate dispositions --apply` and `atelier migrate after-merge --apply` run; `atelier check` and `atelier doctor` pass; its AGENTS.md describes the lifecycle in the same words as Atelier's own.
- [ ] `branch_base` is dropped by expand and contract: Atelier first reads `WorkflowBranch.branch_base` as optional; astromech's CI pin then moves to a commit with that change; only then do new ownership records stop writing it and `start` drop `--base`. Tests show a record without `branch_base` parses and a record with it still parses.
- [ ] `atelier migrate after-merge` names the item when removing its last Done when line would leave the section empty, and keeps that line as an ordinary Done when line instead (finding atelier-z3z1).
- [ ] docs/product/workflow-configuration.md says how to move an existing policy without stranding items (the order above).

## Scope

The four repositories' policies, records and AGENTS.md (each through its own process; astromech has CI and its own rules), astromech's CI pin, and the Atelier code named above. Do not change other repositories' code.

## Context

docs/strategies/simpler-lifecycle.md (criterion every-repo; Shape: One binary serves every repository), ADR 0028 decision 10. atelier-ml3r rewrites the skill first, so agents in those repositories follow the new order before their policies change. crates/atelier-core/src/lib.rs (`WorkflowBranch`, `deny_unknown_fields`).
