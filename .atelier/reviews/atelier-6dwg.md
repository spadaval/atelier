---
schema: atelier.review
schema_version: 1
id: atelier-6dwg
title: 'List merge in merged (crates/atelier-app/src/review_gate.rs:306-319) checks only that resolved has as many entries as expected and contains each expected entry, not that every resolved entry is expected. When a side already holds a duplicate entry, a resolution can trade the duplicate for an entry nobody wrote. Reproduced: an out-of-scope record X has hand-written labels [dup, dup] on the base (atelier check accepts it); the branch appends ''branch'', the base appends ''base'', and labels conflict. Resolving to [dup, branch, base, smuggled] passes atelier check and merge --check passes review.current. The same holds for relationship lists such as blocks. Impact is limited: the CLI dedups labels and links so only hand-edited records hold duplicates, nothing either side added can be dropped, and in-scope records are also held to status-only. Fix: also require resolved.iter().all(|entry| expected.contains(entry)), or compare the two as sets.'
target: atelier-1isk
identity: Independent reviewer (Claude subagent)
commit: decb65a8e086ca314995b9534104ce64d67261eb
created_at: 2026-10-16T20:21:23.471554110Z
event:
  event: finding
  recommendation: defer
---

List merge in merged (crates/atelier-app/src/review_gate.rs:306-319) checks only that resolved has as many entries as expected and contains each expected entry, not that every resolved entry is expected. When a side already holds a duplicate entry, a resolution can trade the duplicate for an entry nobody wrote. Reproduced: an out-of-scope record X has hand-written labels [dup, dup] on the base (atelier check accepts it); the branch appends 'branch', the base appends 'base', and labels conflict. Resolving to [dup, branch, base, smuggled] passes atelier check and merge --check passes review.current. The same holds for relationship lists such as blocks. Impact is limited: the CLI dedups labels and links so only hand-edited records hold duplicates, nothing either side added can be dropped, and in-scope records are also held to status-only. Fix: also require resolved.iter().all(|entry| expected.contains(entry)), or compare the two as sets.
