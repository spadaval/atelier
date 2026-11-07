---
created_at: "2026-11-07T05:01:01.441016682+00:00"
id: "atelier-ml3r"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-3jdm"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-4nsl"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-8bh4"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ilba"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-jnrh"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-l457"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-md0v"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-mmpv"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Say the lifecycle once, plainly, everywhere"
updated_at: "2026-11-07T09:41:38.991522124+00:00"
---

## Goal

Make the skill, the CLI and every repository describe the new lifecycle in the same words, with one meaning per word, and nothing left of what this mission removed. A fresh agent read about 1,000 lines across eight files and still could not state the order; "check" alone meant at least six things.

## Done when

- [ ] The agent-factory skill (spadaval/.agents, edited in a worktree there) describes the new lifecycle in its atelier reference and its review, implement, plan and validate procedures, and nothing this mission removed (for example `start --base`, `<issue_type>/<id>` branches, close before review, after-merge markers). Its total line count falls.
- [ ] check, land, close, review and done each mean one thing across CLI help, the skill and docs; CONTEXT.md defines each.
- [ ] Every Atelier repository on this host (atelier, astromech, cosplay-studio, demiurge, life-on-rails) runs the new policy, its open after-merge checks are loose ends, and its AGENTS.md describes the lifecycle in the same words (the every-repo criterion); astromech's CI pin (`ATELIER_REV` in `.github/workflows/atelier.yml`) moves to the new binary, after which new ownership records stop writing `branch_base` and `start` drops the ignored `--base` flag. Until then, `start`'s detached-HEAD refusal suggests `git switch -c <id> <base> && atelier start <id>`, which works in a linked worktree, and docs/product/work-model.md no longer says old guidance passing `--base` "does not fail" (finding atelier-ywpf). The skill and each AGENTS.md describe the light path (atelier-4nsl), and astromech starts using it only after its pin moves.
- [ ] Each repository's `validation` items are settled before its policy moves `validation` to the on-base workflow: done, superseded, or converted to loose ends, so no item is left in a status that workflow lacks (finding atelier-b0uv); docs/product/workflow-configuration.md says how to move a policy without that trap.
- [ ] `atelier migrate after-merge` names the item when removing its last Done when line would leave the section empty, and keeps that line as an ordinary Done when line instead (finding atelier-z3z1).
