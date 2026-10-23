---
created_at: "2026-10-23T00:05:23.107691004+00:00"
id: "atelier-ukq9"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:05:13.840068638+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s11.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s11.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S11"
  result: "pass"
criteria_mission: "atelier-k8h8"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-k8h8"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S11 claim: merge refuses a done claim land will refuse, runs checks in the merged checkout's own target dir, cleans up merged-tree checkouts after an interrupted merge; land's per-review history walks are batched and find review records created inside a merge commit. Method: s11.sh, scratch repos. Observed: (a) a close commit that also changes code after approval: merge --check fails review.current and claims.reviewed ('land on main would refuse it with land_base_done_claim'), land refuses, merge integrates nothing. (b) a merge_commit check records its cwd: it ran in <repo>/.git/atelier-merge-XXXX/tree, the merged checkout; Atelier passes the caller's CARGO_TARGET_DIR through unchanged, so 'own target dir' rests on this repo's policy, whose two nextest checks set CARGO_TARGET_DIR=target (.atelier/workflow.yaml:175-177), as atelier-isyf scoped it. (c) kill -9 during a slow check left .git/atelier-merge-XXXX and a registered worktree; the next atelier merge removed both and merged. (e) a no_action disposition first added by a conflict-resolving merge commit: merge --check 'pass findings.disposed', land passes. (d) GIT_TRACE count of git log/rev-list/diff-tree calls in land: 1 vs 30 review records at one pin -> 2 and 2; 10 vs 80 findings+dispositions -> 3 and 3 (one-time graph load). Limits: approvals at distinct pins still cost one 'rev-list --parents pin..HEAD' plus merge-base calls each (1 approval -> 4 walks, 10 -> 13); atelier-7b6e's decision note accepted these approval-currency calls as outside its history-query layer and proposed answering ancestry from the loaded graph, but I found no open follow-up issue for it. An earlier uncommitted S11 receipt from a broken version of this script (it counted approvals) was deleted before commit."
updated_at: "2026-10-23T00:05:23.112845149+00:00"
---

## Summary

S11 claim: merge refuses a done claim land will refuse, runs checks in the merged checkout's own target dir, cleans up merged-tree checkouts after an interrupted merge; land's per-review history walks are batched and find review records created inside a merge commit. Method: s11.sh, scratch repos. Observed: (a) a close commit that also changes code after approval: merge --check fails review.current and claims.reviewed ('land on main would refuse it with land_base_done_claim'), land refuses, merge integrates nothing. (b) a merge_commit check records its cwd: it ran in <repo>/.git/atelier-merge-XXXX/tree, the merged checkout; Atelier passes the caller's CARGO_TARGET_DIR through unchanged, so 'own target dir' rests on this repo's policy, whose two nextest checks set CARGO_TARGET_DIR=target (.atelier/workflow.yaml:175-177), as atelier-isyf scoped it. (c) kill -9 during a slow check left .git/atelier-merge-XXXX and a registered worktree; the next atelier merge removed both and merged. (e) a no_action disposition first added by a conflict-resolving merge commit: merge --check 'pass findings.disposed', land passes. (d) GIT_TRACE count of git log/rev-list/diff-tree calls in land: 1 vs 30 review records at one pin -> 2 and 2; 10 vs 80 findings+dispositions -> 3 and 3 (one-time graph load). Limits: approvals at distinct pins still cost one 'rev-list --parents pin..HEAD' plus merge-base calls each (1 approval -> 4 walks, 10 -> 13); atelier-7b6e's decision note accepted these approval-currency calls as outside its history-query layer and proposed answering ancestry from the loaded graph, but I found no open follow-up issue for it. An earlier uncommitted S11 receipt from a broken version of this script (it counted approvals) was deleted before commit.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s11.sh
```

Exit status: 0

## Stdout

Bytes: 1957
Truncated: no

```text
=== (a) merge and land agree on a close that changes code after approval
merge --check rc=1: FAIL review.current: no current independent approval or explicit trivial exemption; authors: dev, dev <dev@x>, dev@x
FAIL claims.reviewed: commit 4bcff2c4 marks s11a-1996 done outside every current approval, so `land` on main would refuse it with land_base_done_claim
land rc=1: land: refused — 1 check (land_invariants_unmet)
ok: merge and land both refuse the uncovered done claim
ok: merge integrates nothing
=== (b,c) merged-tree checks run in the merged checkout's own target dir; leftovers swept
check ran in: /home/supadava/.cache/atelier-tmp/validate-k8h8/s11b/.git/atelier-merge-qxyZEL/tree (CARGO_TARGET_DIR seen by check: /home/supadava/.cache/atelier-tmp/validate-k8h8/foreign-target)
ok: check ran inside the merged-tree checkout
after kill -9, leftover: /home/supadava/.cache/atelier-tmp/validate-k8h8/s11b/.git/atelier-merge-qxyZEL; worktrees: 2
next merge rc=0: 
Merged task/s11b-btoj into main with merge commit ef4b5a7b; deleted local work branch. No other commits or tracker records were created.
after next merge, leftover: none; worktrees: 1
ok: interrupted merge's checkout cleaned up by the next merge
ok: foreign CARGO_TARGET_DIR never created (the check builds nothing here)
=== (e) a review record created inside a conflict-resolving merge commit is found
merge commit adds: A	.atelier/reviews/s11e-wfx4.md M	f.txt 
pass findings.disposed: every finding has an accepted disposition
ok: disposition first added by the merge commit is found
land: land: passed
=== (d) land's record history walks do not grow with review records
review records at one pin: 1 -> 2 walks, 30 -> 2 walks
findings+dispositions (after the one-time graph load): 10 -> 3 walks, 80 -> 3 walks
ok: record history walks do not grow with review records
info: approvals at distinct pins: 1 -> 4 walks, 10 -> 13 walks (approval-currency checks, one range walk per pin)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

