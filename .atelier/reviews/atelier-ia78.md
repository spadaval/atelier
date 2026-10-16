---
schema: atelier.review
schema_version: 1
id: atelier-ia78
title: 'Re-review at 4daf4c40 (master is an ancestor). Fix 3df28a5f: change_type now carries Background into the new body shape; in a scratch repo, a task with Background retyped to mission and back kept Background byte-identical in the file, and a record without Background gained none. The atelier-core unit test asserts Background survives task -> mission -> task and passes. issue create --help lists Background for work items and missions. Earlier checks still hold: any-position parse with stable rendered order, empty/duplicate/## rejection, bundle create, update --body, issue show, and check passing. Fresh evidence atelier-c43v (nextest workspace pass) is at commit 3df28a5f; the only later commit is records.'
target: atelier-by8u
identity: Independent reviewer (Claude subagent)
commit: 4daf4c40ed8b5ef1d1850c32e73fdbc6039c7f2f
created_at: 2026-10-16T00:56:25.073842028Z
event:
  event: approval
---

Re-review at 4daf4c40 (master is an ancestor). Fix 3df28a5f: change_type now carries Background into the new body shape; in a scratch repo, a task with Background retyped to mission and back kept Background byte-identical in the file, and a record without Background gained none. The atelier-core unit test asserts Background survives task -> mission -> task and passes. issue create --help lists Background for work items and missions. Earlier checks still hold: any-position parse with stable rendered order, empty/duplicate/## rejection, bundle create, update --body, issue show, and check passing. Fresh evidence atelier-c43v (nextest workspace pass) is at commit 3df28a5f; the only later commit is records.
