---
created_at: "2026-11-08T05:52:15.790452443+00:00"
id: "atelier-bxs7"
issue_type: "loose_end"
labels:
- "agent-factory"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-45gr"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-8bj8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-dkul"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Install the new binary and link this host's skill to this repository"
updated_at: "2026-11-08T05:52:15.790452443+00:00"
---

## Goal

Install the new binary on this host and point the live skill at this repository, so agents here use the moved skill and spadaval/.agents can drop its copy.

## Done when

- [ ] If `atelier-4k28`'s smoke script has merged, it passes on the new binary before install. Then `cargo install --path crates/atelier-cli --locked`, and `atelier setup --link ~/projects/atelier-skill`, a worktree of this repository that follows `master` (`git worktree add --detach ~/projects/atelier-skill master`, since `master` is checked out elsewhere; moved forward with `git -C ~/projects/atelier-skill checkout --detach master` after each merge) replaces both symlinks; `readlink -f ~/.claude/skills/agent-factory ~/.agents/skills/agent-factory` names that checkout's `skills/agent-factory`, and `atelier doctor` reports the skill as linked to a checkout.
