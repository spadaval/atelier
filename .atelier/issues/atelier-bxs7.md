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
closed_at: "2026-11-08T22:46:43.104227849+00:00"
status: "done"
title: "Install the new binary and link this host's skill to this repository"
updated_at: "2026-11-08T22:46:43.104227849+00:00"
---

## Goal

Install the new binary on this host and point the live skill at this repository, so agents here use the moved skill and spadaval/.agents can drop its copy.

## Done when

- [x] If `atelier-4k28`'s smoke script has merged, it passes on the new binary before install. Then `cargo install --path crates/atelier-cli --locked`, and `atelier setup --link ~/projects/atelier-skill`, a worktree of this repository that follows `master` (`git worktree add --detach ~/projects/atelier-skill master`, since `master` is checked out elsewhere; moved forward with `git -C ~/projects/atelier-skill checkout --detach master` after each merge) replaces both symlinks; `readlink -f ~/.claude/skills/agent-factory ~/.agents/skills/agent-factory` names that checkout's `skills/agent-factory`, and `atelier doctor` reports the skill as linked to a checkout.

## Report

scripts/smoke-repos.sh: no regression across 5 repositories at 0db15896; cargo install gives atelier 0.3.1+0db15896; setup --link ~/projects/atelier-skill (detached at master 0db15896) replaced both symlinks, readlink -f names ~/projects/atelier-skill/skills/agent-factory for both, and doctor reports the skill linked to a checkout, not behind master.
