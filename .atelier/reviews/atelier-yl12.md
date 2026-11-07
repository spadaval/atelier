---
schema: atelier.review
schema_version: 2
id: atelier-yl12
title: git.rs:516 entries.iter().flatten() silently skips a branch whose summary cannot be read (e.g. a committed broken workflow.yaml, or a full TMPDIR during Snapshot::at), so work ready lists that branch's owner as plain ready with no mark or notice, and contradicts sessions-and-hooks.md:128.
target: atelier-7n5u
identity: Independent reviewer (Claude subagent)
commit: 2414e9cb784631ee77c0d7d81c3fab5ff5552bff
created_at: 2026-10-17T03:13:07.986241522Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: de052831e12bf26494833849a691583b7428ab8d
    recorded_at: 2026-10-17T03:17:35.730174929Z
    resolution_commit: 21a4320007af41457ede1b2886d7e2455da24b12
    reason: Unreadable branches keep their owners and mark the row state unknown, or print a notice when no owner is found.
    migrated_from: atelier-2kxg
---

git.rs:516 entries.iter().flatten() silently skips a branch whose summary cannot be read (e.g. a committed broken workflow.yaml, or a full TMPDIR during Snapshot::at), so work ready lists that branch's owner as plain ready with no mark or notice, and contradicts sessions-and-hooks.md:128.
