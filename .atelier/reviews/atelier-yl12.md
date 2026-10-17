---
schema: atelier.review
schema_version: 1
id: atelier-yl12
title: git.rs:516 entries.iter().flatten() silently skips a branch whose summary cannot be read (e.g. a committed broken workflow.yaml, or a full TMPDIR during Snapshot::at), so work ready lists that branch's owner as plain ready with no mark or notice, and contradicts sessions-and-hooks.md:128.
target: atelier-7n5u
identity: Independent reviewer (Claude subagent)
commit: 2414e9cb784631ee77c0d7d81c3fab5ff5552bff
created_at: 2026-10-17T03:13:07.986241522Z
event:
  event: finding
  recommendation: fix_now
---

git.rs:516 entries.iter().flatten() silently skips a branch whose summary cannot be read (e.g. a committed broken workflow.yaml, or a full TMPDIR during Snapshot::at), so work ready lists that branch's owner as plain ready with no mark or notice, and contradicts sessions-and-hooks.md:128.
