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
updated_at: "2026-11-07T05:01:01.441016682+00:00"
---

## Goal

Make the skill, the CLI and every repository describe the new lifecycle in the same words, with one meaning per word, and nothing left of what this mission removed. A fresh agent read about 1,000 lines across eight files and still could not state the order; "check" alone meant at least six things.

## Done when

- [ ] The agent-factory skill (spadaval/.agents, edited in a worktree there) describes the new lifecycle in its atelier reference and its review, implement, plan and validate procedures, and nothing this mission removed. Its total line count falls.
- [ ] check, land, close, review and done each mean one thing across CLI help, the skill and docs; CONTEXT.md defines each.
- [ ] Every Atelier repository on this host (atelier, astromech, cosplay-studio, demiurge, life-on-rails) runs the new policy, its open after-merge checks are loose ends, and its AGENTS.md describes the lifecycle in the same words (the every-repo criterion).
