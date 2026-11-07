---
schema: atelier.review
schema_version: 2
id: atelier-6vfg
title: 'crates/atelier-app/src/git_freshness.rs:712-747 (ancestry_path, called from uncached_edges at :672) does not terminate on a parent cycle. The DFS marks a commit only after its post-order visit, so a commit re-reached from its own ancestors while still expanding is pushed again without limit. Git still honors .git/info/grafts under --no-replace-objects (checked on git 2.53), and a graft file can make a cycle. Scenario: commits c1..c4, grafts line ''<c3> <c4>''. Then ''git rev-list --parents c1..c4'' prints ''c4 c3'' / ''c3 c4'' and ''git rev-list --ancestry-path c1..c4'' returns quickly with empty output (the old code path). A scratch copy of ancestry_path on that output kept allocating until it aborted at 3 GiB. So land or any edge read across such a range now hangs and exhausts memory where it used to answer. That breaks the ''identical answers'' and ''bounded'' claims for grafts. Fix: either propagate to a fixpoint like Git''s limit_to_ancestry (mark commits whose parent is pin or marked, repeat until nothing changes), which matches Git even on cycles, or track in-progress commits and skip them. Add a cyclic-graft case to the parity test. Also add an octopus merge and a shallow clone; I checked both by hand and they match, but the test does not cover them. sha256 coverage is not needed because the function only handles strings.'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 76a364da360c29b9afe0cdf075c9934ba73b4e5c
created_at: 2026-10-16T21:59:55.781789530Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 3841028c54e10d40fe6d109c806768be33cf0b2c
    recorded_at: 2026-10-16T22:08:59.546753632Z
    resolution_commit: bc1ae1c335dff65e1782a9cea9c7736d5d499f3f
    reason: 'Resolved: ancestry_path (git_freshness.rs:714) now marks commits whose parent is the pin or marked, repeated until a pass marks nothing. This is the same rule as Git''s limit_to_ancestry, and it is bounded because every pass but the last marks a new commit (at most N+1 passes). I replayed it with a scratch copy against git rev-list --ancestry-path on 4-commit graft cycles: ''<c3> <c4>'' gives an empty set in 1 pass, ''<c3> <c2> <c4>'' (marking runs through the cycle) gives 3 of 3 in 2 passes, and ''<c2> <c1> <c4>'' matches for pins c1 and c2. It also matches on this repository (pins 50 to 2,000 commits back) and on a 200k-commit linear history (2 passes, 91 ms). The new parity test covers octopus merges and a graft cycle. Its cycle cases never mark into the cycle, but they would have hung the old code.'
    migrated_from: atelier-9bjn
---

crates/atelier-app/src/git_freshness.rs:712-747 (ancestry_path, called from uncached_edges at :672) does not terminate on a parent cycle. The DFS marks a commit only after its post-order visit, so a commit re-reached from its own ancestors while still expanding is pushed again without limit. Git still honors .git/info/grafts under --no-replace-objects (checked on git 2.53), and a graft file can make a cycle. Scenario: commits c1..c4, grafts line '<c3> <c4>'. Then 'git rev-list --parents c1..c4' prints 'c4 c3' / 'c3 c4' and 'git rev-list --ancestry-path c1..c4' returns quickly with empty output (the old code path). A scratch copy of ancestry_path on that output kept allocating until it aborted at 3 GiB. So land or any edge read across such a range now hangs and exhausts memory where it used to answer. That breaks the 'identical answers' and 'bounded' claims for grafts. Fix: either propagate to a fixpoint like Git's limit_to_ancestry (mark commits whose parent is pin or marked, repeat until nothing changes), which matches Git even on cycles, or track in-progress commits and skip them. Add a cyclic-graft case to the parity test. Also add an octopus merge and a shallow clone; I checked both by hand and they match, but the test does not cover them. sha256 coverage is not needed because the function only handles strings.
