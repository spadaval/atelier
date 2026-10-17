---
schema: atelier.review
schema_version: 1
id: atelier-proi
title: 'Test gap: the trusted point for an unowned branch off the base (merge base with refs/heads/<base>) has no test. Replacing that reference with "HEAD" makes land skip every retained claim there, and all 114 session and merge_gates tests still pass. The matching mutation for <base>@{upstream} is caught by 8 tests. Add a case where an unowned off-base branch edits an old claim (or adds a record about it) and land refuses.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6039144f2ace03f72f3112f5c4f01afa5cfce4f7
created_at: 2026-10-16T23:29:51.350384779Z
event:
  event: finding
  recommendation: fix_now
---

Test gap: the trusted point for an unowned branch off the base (merge base with refs/heads/<base>) has no test. Replacing that reference with "HEAD" makes land skip every retained claim there, and all 114 session and merge_gates tests still pass. The matching mutation for <base>@{upstream} is caught by 8 tests. Add a case where an unowned off-base branch edits an old claim (or adds a record about it) and land refuses.
