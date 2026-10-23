---
created_at: "2026-10-23T00:03:13.093846627+00:00"
id: "atelier-fzum"
evidence_type: "validation"
commit: "f3ee2581393146618dcb92e6bd5d971bdfb6110b"
result: "pass"
captured_at: "2026-10-23T00:03:11.452711392+00:00"
command: "bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s3.sh"
exit_status: "0"
path: "/home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s3.sh"
agent_identity: "claude-validator"
target:
  kind: "issue"
  id: "atelier-k8h8"
  role: "validates"
criteria:
- slug: "S3"
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
title: "S3 claim: a merge strategy merges a clean branch into the current base after merge_checks pass on the merged tree, keeps a clean merge's approval, and refuses a lockfile git merges cleanly but cargo metadata --locked rejects; merge_commit is the default when a test check exists (decision atelier-6mgg). Method: s3.sh builds a scratch Cargo workspace (crates a, m, z; path deps only, offline) whose policy has a shell.check 'cargo metadata --locked --offline' and NO merge_strategy line. Observed: a clean approved branch 1 commit behind the base merged with a two-parent merge commit, review.current passed, merge.merged_tree reported the check ran on the merged tree. A second approved branch (z depends on path m; passes cargo metadata --locked alone) against a base that removed crate m (also passes alone): git merge-tree reports a clean merge, atelier merge refused with 'FAIL merge.merged_tree ... not every shell.check passed', base unchanged, merged-tree worktree removed. A policy without shell.check (scenario t0) fast-forwarded, so the default switches only with a check. Limits: my broken merge fails cargo metadata on the manifest's missing path dependency with a stale Cargo.lock, not a version-only lock staleness; the repo's own test (merge_gates.rs ~2253) uses a stand-in script for cargo metadata --locked, so a pure lock-version case is not exercised by either."
updated_at: "2026-10-23T00:03:13.094899910+00:00"
---

## Summary

S3 claim: a merge strategy merges a clean branch into the current base after merge_checks pass on the merged tree, keeps a clean merge's approval, and refuses a lockfile git merges cleanly but cargo metadata --locked rejects; merge_commit is the default when a test check exists (decision atelier-6mgg). Method: s3.sh builds a scratch Cargo workspace (crates a, m, z; path deps only, offline) whose policy has a shell.check 'cargo metadata --locked --offline' and NO merge_strategy line. Observed: a clean approved branch 1 commit behind the base merged with a two-parent merge commit, review.current passed, merge.merged_tree reported the check ran on the merged tree. A second approved branch (z depends on path m; passes cargo metadata --locked alone) against a base that removed crate m (also passes alone): git merge-tree reports a clean merge, atelier merge refused with 'FAIL merge.merged_tree ... not every shell.check passed', base unchanged, merged-tree worktree removed. A policy without shell.check (scenario t0) fast-forwarded, so the default switches only with a check. Limits: my broken merge fails cargo metadata on the manifest's missing path dependency with a stale Cargo.lock, not a version-only lock staleness; the repo's own test (merge_gates.rs ~2253) uses a stand-in script for cargo metadata --locked, so a pure lock-version case is not exercised by either.

## Command

```console
bash /home/supadava/.cache/atelier-tmp/validate-k8h8/scripts/s3.sh
```

Exit status: 0

## Stdout

Bytes: 1887
Truncated: no

```text
lock branch alone: cargo metadata --locked passes
base alone: cargo metadata --locked passes
--- merge clean branch s3-ohvo (behind base by 1)

pass work.terminal: owner and all scoped work are closed; all child and advancing work is done
pass merge.merged_tree: merge_checks ran on merged tree 92900634d729658a53a59d32f9b0ca43178b2fef of task/s3-ohvo into main at 33333d3589588c3f76e257c93bfca8fea97da55f and passed: cargo metadata --locked --offline --format-version 1 > /dev/null; evidence at the branch head does not prove it
pass claims.reviewed: every done claim merging adds is covered by a current accepted approval
Git hooks are disabled during gated integration and cleanup; put required read-only checks in the base policy's merge_checks.
Merged task/s3-ohvo into main with merge commit 3e121313; deleted local work branch. No other commits or tracker records were created.
rc=0
ok: main tip is a two-parent merge commit
ok: approval kept across the clean merge
ok: merge check ran
--- lock branch: git merges cleanly?
ok: git merge-tree reports a clean merge
  failed to load manifest for dependency `m`

Caused by:
  failed to read `/home/supadava/.cache/atelier-tmp/validate-k8h8/s3/.git/atelier-merge-CtyUxt/tree/crates/m/Cargo.toml`

Caused by:
  No such file or directory (os error 2)

pass work.terminal: owner and all scoped work are closed; all child and advancing work is done
FAIL merge.merged_tree: merged tree a0276b68d93fbe6400f25dd95cb6bd1e149d73fe of task/s3-1urs into main at 3e1213134769bc8fd697f5fdc2f6ebddb03913ce is unproven: not every shell.check passed on it; shell.check names the failure
pass claims.reviewed: every done claim merging adds is covered by a current accepted approval
Error: merge blocked: shell.check, merge.merged_tree
rc=1
ok: merge refused
ok: base unchanged
/home/supadava/.cache/atelier-tmp/validate-k8h8/s3 01a5a56 [task/s3-1urs]
```

## Stderr

Bytes: 0
Truncated: no

```text
```

