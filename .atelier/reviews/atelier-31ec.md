---
schema: atelier.review
schema_version: 2
id: atelier-31ec
title: 'status.rs:527-543 / human_output.rs:441: the Unassigned Work panel shows no in-flight marks while Ready Work does; a 6th ready item in flight on another branch appears only in Unassigned, unmarked, so an agent choosing work between missions can start owned work.'
target: atelier-ihns
identity: Independent reviewer 2 (Claude subagent)
commit: f8b0ab0b03064f8f8c2c6973f7d7cad6d43c9fb4
created_at: 2026-10-17T03:30:18.105089576Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: 83500a5e537a945fd8a1f0c98a0fd538b4df0d5e
    recorded_at: 2026-10-17T03:32:48.364324472Z
    resolution_commit: 56a619dc277f37868f4808a19f6680e1ce118c09
    reason: Unassigned rows get the same in-flight mark line via a shared mark_line helper; test covers an item past the ready list.
    migrated_from: atelier-dgbr
---

status.rs:527-543 / human_output.rs:441: the Unassigned Work panel shows no in-flight marks while Ready Work does; a 6th ready item in flight on another branch appears only in Unassigned, unmarked, so an agent choosing work between missions can start owned work.
