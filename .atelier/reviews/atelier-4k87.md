---
schema: atelier.review
schema_version: 2
id: atelier-4k87
title: finding on atelier-bxs7 @859f41a2
target: atelier-bxs7
identity: plan-reviewer-g6rp
commit: 859f41a2454c4278bc33d18868729b139e7e90f7
created_at: 2026-11-08T05:58:07.475475500Z
event:
  event: finding
  recommendation: fix_now
---

[medium] After 859f41a2, the live skill is a detached worktree (~/projects/atelier-skill) that someone must move by hand after every merge ('checkout --detach master'; the strategy says 'pulled'). atelier merge only updates a worktree that has the base branch checked out (merge.rs base_checkout), so a detached one stays where it was. Nothing in the plan does that step: no command, hook or AGENTS.md line. atelier-45gr reports a linked skill as 'linked to a checkout' with no warning, so the live skill can lag master silently, which is the drift this mission removes. Fix: make 45gr's linked case compare the checkout's HEAD with the local master and warn when it is behind, printing the checkout command, or add the step to AGENTS.md's merge sequence. Also make the strategy and bxs7 use the same verb.
