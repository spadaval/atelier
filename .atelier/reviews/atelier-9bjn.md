---
schema: atelier.review
schema_version: 1
id: atelier-9bjn
title: 'Resolved: ancestry_path (git_freshness.rs:714) now marks commits whose parent is the pin or marked, repeated until a pass marks nothing. This is the same rule as Git''s limit_to_ancestry, and it is bounded because every pass but the last marks a new commit (at most N+1 passes). I replayed it with a scratch copy against git rev-list --ancestry-path on 4-commit graft cycles: ''<c3> <c4>'' gives an empty set in 1 pass, ''<c3> <c2> <c4>'' (marking runs through the cycle) gives 3 of 3 in 2 passes, and ''<c2> <c1> <c4>'' matches for pins c1 and c2. It also matches on this repository (pins 50 to 2,000 commits back) and on a 200k-commit linear history (2 passes, 91 ms). The new parity test covers octopus merges and a graft cycle. Its cycle cases never mark into the cycle, but they would have hung the old code.'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 3841028c54e10d40fe6d109c806768be33cf0b2c
created_at: 2026-10-16T22:08:59.546753632Z
event:
  event: disposition
  finding: atelier-6vfg
  decision: fix_now
  supersedes: null
  resolution_commit: bc1ae1c335dff65e1782a9cea9c7736d5d499f3f
  follow_up: null
---

Resolved: ancestry_path (git_freshness.rs:714) now marks commits whose parent is the pin or marked, repeated until a pass marks nothing. This is the same rule as Git's limit_to_ancestry, and it is bounded because every pass but the last marks a new commit (at most N+1 passes). I replayed it with a scratch copy against git rev-list --ancestry-path on 4-commit graft cycles: '<c3> <c4>' gives an empty set in 1 pass, '<c3> <c2> <c4>' (marking runs through the cycle) gives 3 of 3 in 2 passes, and '<c2> <c1> <c4>' matches for pins c1 and c2. It also matches on this repository (pins 50 to 2,000 commits back) and on a 200k-commit linear history (2 passes, 91 ms). The new parity test covers octopus merges and a graft cycle. Its cycle cases never mark into the cycle, but they would have hung the old code.
