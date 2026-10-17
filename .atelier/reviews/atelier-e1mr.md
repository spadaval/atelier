---
schema: atelier.review
schema_version: 1
id: atelier-e1mr
title: '[low] references/atelier.md merge bullet: the text still says ''atelier merge <id> fast-forwards the policy''s base branch'' and then scopes merge_commit to ''a branch behind the base that merges cleanly''. merge.rs never fast-forwards under merge_commit: it always runs commit-tree with the base head and branch head as parents, so even an up-to-date branch lands as a merge commit after merge_checks on that tree. An agent reading the reference would expect a fast-forward there. Fix: say merge fast-forwards under the default fast_forward_only, and under merge_commit always lands a merge commit, which is why a branch behind the base needs no catch-up merge.'
target: atelier-zqee
identity: Independent reviewer (Claude subagent)
commit: dcb2107de131a531bb41298e5edd137d643dcbc5
created_at: 2026-10-17T07:19:15.177964286Z
event:
  event: finding
  recommendation: fix_now
---

[low] references/atelier.md merge bullet: the text still says 'atelier merge <id> fast-forwards the policy's base branch' and then scopes merge_commit to 'a branch behind the base that merges cleanly'. merge.rs never fast-forwards under merge_commit: it always runs commit-tree with the base head and branch head as parents, so even an up-to-date branch lands as a merge commit after merge_checks on that tree. An agent reading the reference would expect a fast-forward there. Fix: say merge fast-forwards under the default fast_forward_only, and under merge_commit always lands a merge commit, which is why a branch behind the base needs no catch-up merge.
