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
updated_at: "2026-11-07T14:11:43.248964117+00:00"
---

## Goal

atelier, astromech, cosplay-studio, demiurge and life-on-rails share one installed binary but keep their own policies, records and AGENTS.md. Move the four other repositories onto the new lifecycle so the every-repo criterion holds, and make `branch_base` optional on read so its write can be dropped later (atelier-hfme).

## Done when

- [ ] Atelier reads `WorkflowBranch.branch_base` as optional (expand); tests show a record without it and a record with it both parse. Writing it stops later, in atelier-hfme, once astromech's pin is past this commit.
- [ ] `atelier migrate after-merge` names the item when removing its last Done when line would leave the section empty, and keeps that line as an ordinary Done when line instead (finding atelier-z3z1).
- [ ] docs/product/workflow-configuration.md says how to move an existing policy without stranding items (the order below).
- [ ] In each of astromech, cosplay-studio, demiurge and life-on-rails, through one issue in that repository (or the light path, which astromech's pin 12efdfd9 and the other three, which have no pinned CI, can read), reviewed independently and merged with `atelier merge`, in this order: every item in a status the new policy lacks (`backlog` validations under the on-base workflow, `blocked`, `review`) moves to `todo` or is superseded, with no play-check needed (finding atelier-b0uv; astromech's play-check validations stay `validation` items, since they assess its missions' criteria before close); its policy takes the starter's shape (`merge_commit`, no `close` for work, `loose_end` and `validation` on the on-base workflow); `atelier migrate after-merge --apply` runs where there are After merge lines (only astromech has any, 5); `atelier check` and `atelier doctor` pass; its AGENTS.md describes the lifecycle in the same words as Atelier's own.
- [ ] Nothing is pushed for demiurge or life-on-rails beyond what their owner already pushes (both carry hundreds of unpushed local commits); astromech and cosplay-studio push their base as they usually do.

## Scope

Atelier: `WorkflowBranch` read, `migrate after-merge`, workflow-configuration.md. The four repositories' policies, records and AGENTS.md, each through its own process. Not in scope, and said here because the earlier plan included them: `atelier migrate dispositions` is optional cleanup and is not run (it rewrites about 1,400 review records in repositories with work in flight, and the binary reads both forms); removing the `branch_base` write and `start --base` moves to atelier-hfme (both are invisible to agents, and removing them needs another pin move).

## Context

docs/strategies/simpler-lifecycle.md (criterion every-repo; Shape: One binary serves every repository), ADR 0028 decision 10. atelier-ml3r rewrites the skill first, so agents in those repositories follow the new order before their policies change. crates/atelier-core/src/lib.rs (`WorkflowBranch`, `deny_unknown_fields`).
