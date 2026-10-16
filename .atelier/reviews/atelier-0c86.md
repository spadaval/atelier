---
schema: atelier.review
schema_version: 1
id: atelier-0c86
title: 'Consistency: ebdc2ad5 stopped trusting the local base for unowned branches because it ''can hold unpushed commits that no land checked'', yet an owned work branch still trusts branch_base, which is the local refs/heads/<base> tip whenever the branch contains it. A branch started from a local base that has an unlanded direct commit (for example, deleting passing proof) gets a passing land on the branch. The base''s own land against its upstream still catches it before a push, so this is not a hole in what reaches the upstream. Consider using the merge base with <base>@{upstream} when it is older, or document why trusting the local base is acceptable for owned branches.'
target: atelier-37uq
identity: Independent reviewer (Claude subagent)
commit: 6115f1e9021c5c9d59228e425f3a5b1c5f767091
created_at: 2026-10-16T23:40:59.919280803Z
event:
  event: finding
  recommendation: defer
---

Consistency: ebdc2ad5 stopped trusting the local base for unowned branches because it 'can hold unpushed commits that no land checked', yet an owned work branch still trusts branch_base, which is the local refs/heads/<base> tip whenever the branch contains it. A branch started from a local base that has an unlanded direct commit (for example, deleting passing proof) gets a passing land on the branch. The base's own land against its upstream still catches it before a push, so this is not a hole in what reaches the upstream. Consider using the merge base with <base>@{upstream} when it is older, or document why trusting the local base is acceptable for owned branches.
