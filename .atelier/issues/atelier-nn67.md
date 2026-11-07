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
updated_at: "2026-11-07T11:19:13.110128252+00:00"
---

## Goal

atelier, astromech, cosplay-studio, demiurge and life-on-rails share one installed binary but keep their own policies, records and AGENTS.md. Move the four other repositories onto the new lifecycle, so the every-repo criterion holds, and then remove the compatibility shims that exist only for them.

## Done when

- [ ] In each of astromech, cosplay-studio, demiurge and life-on-rails, through that repository's own review and merge: its `validation` items are settled first (done, superseded, or converted to loose ends), so none is left in a status the on-base workflow lacks (finding atelier-b0uv); its policy takes the starter's shape (`merge_commit`, no `close` for work, `loose_end` and `validation` on the on-base workflow); `atelier migrate dispositions --apply` and `atelier migrate after-merge --apply` have run; `atelier check` and `atelier doctor` pass; its AGENTS.md describes the lifecycle in the same words as Atelier's own.
- [ ] astromech's CI pin (`ATELIER_REV` in `.github/workflows/atelier.yml`) moves to a binary with the new lifecycle, and only then may astromech use the light path.
- [ ] After the pin moves, in Atelier: new ownership records stop writing `branch_base`, and `start` drops the ignored `--base` flag; `start`'s detached-HEAD refusal suggests `git switch -c <id> <base> && atelier start <id>`, which works in a linked worktree, and docs/product/work-model.md no longer says old guidance passing `--base` "does not fail" (finding atelier-ywpf).
- [ ] `atelier migrate after-merge` names the item when removing its last Done when line would leave the section empty, and keeps that line as an ordinary Done when line instead (finding atelier-z3z1).
- [ ] docs/product/workflow-configuration.md says how to move an existing policy without stranding items (the order above).

## Scope

The four repositories' policies, records and AGENTS.md (each through its own process; astromech has CI and its own reviewers' rules), astromech's CI pin, and the Atelier code named above. Do not change other repositories' code.

## Context

docs/strategies/simpler-lifecycle.md (criterion every-repo; Shape: One binary serves every repository), ADR 0028 decision 10. atelier-ml3r rewrites the skill first, so agents in those repositories follow the new order before their policies change.
