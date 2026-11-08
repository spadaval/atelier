---
schema: atelier.review
schema_version: 2
id: atelier-pak1
title: finding on atelier-8bj8 @5cf579c4
target: atelier-8bj8
identity: reviewer-8bj8
commit: 5cf579c407d1090c215bb64d63ba9bf9b57608f3
created_at: 2026-11-08T06:14:00.821139860Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/skill_link.rs:132 When the lock lists agent-factory, link treats a real directory at either path as a skills copy and deletes it with remove_dir_all, but the lock only says the skill is installed, not for which agents. Reproduced in a scratch HOME: lock listing agent-factory, ~/.agents/skills/agent-factory a dir, ~/.claude/skills/agent-factory a hand-made dir holding MY_EDITS.md; setup --link printed 'was a copy skills add installed, now removed' for both and MY_EDITS.md is gone. Needs a user's own Claude-side copy plus a skills install for other agents only, so it does not arise on this host (both paths are symlinks, no lock). Suggested fix: rename a replaced copy aside (e.g. <path>.bak) instead of deleting it, and print where it went.
