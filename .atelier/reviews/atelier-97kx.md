---
schema: atelier.review
schema_version: 2
id: atelier-97kx
title: 'The recovery advice is wrong for the records it names. The review_gate.rs error (line 417) says to cherry-pick any commits made after the rebase and to record only review events again, and reviews-and-merge.md lists ''new proof'' as something to cherry-pick. But evidence also pins the HEAD it was recorded at, which after the rebase is a rewritten commit. evidence_freshness.rs proof_matches/evidence_is_fresh need record.data.commit to be an ancestor of HEAD, so a cherry-picked proof never counts. Reproduced with the built binary: a starter policy plus evidence.attached on close; review, rebase, commit a fix, record proof and commit it, then follow the advice (reset to the reviewed tip, cherry-pick both commits, git merge main). merge --check then reports ''FAIL evidence.attached: ... found 0''; the cherry-picked proof pins 3fc2e32, which is not in the branch history. Cherry-picking review records is worse, because ''cherry-pick any commits'' sweeps them in. An approval recorded after the rebase and cherry-picked back makes merge refuse again with the same ''pins a commit outside the work branch'' error, so the advice loops until the record file is deleted. The claim that review events pin rewritten commits is true, but the fix is incomplete. Error and docs should say: cherry-pick only the code and plan commits made after the rebase; leave out (or git rm) review and evidence records made since the rebase; then record proof and reviews again on the restored branch.'
target: atelier-v9ve
identity: Independent reviewer (Claude subagent)
commit: 501e6b6a81c45883ce218b3a3cbe73ce47f29e9c
created_at: 2026-10-16T20:00:43.604796020Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Independent reviewer (Claude subagent)
    commit: 3f5e3cd9b7b091bb45f2789893844439aa38af45
    recorded_at: 2026-10-16T20:08:38.733730342Z
    resolution_commit: b3076c97ff51c496ae1c48d3dd637818743703c9
    reason: 'Resolved: the review_gate.rs error and reviews-and-merge.md now say to cherry-pick only code or planning commits made after the rebase, leave out review and evidence records, and record proof and review again. This matches the code: an off-branch review record makes merge bail again (review_gate.rs evaluate_scope), and off-branch evidence never counts (proof_matches/evidence_is_fresh fail on a non-ancestor pin and are unwrap_or(false)).'
    migrated_from: atelier-z26w
---

The recovery advice is wrong for the records it names. The review_gate.rs error (line 417) says to cherry-pick any commits made after the rebase and to record only review events again, and reviews-and-merge.md lists 'new proof' as something to cherry-pick. But evidence also pins the HEAD it was recorded at, which after the rebase is a rewritten commit. evidence_freshness.rs proof_matches/evidence_is_fresh need record.data.commit to be an ancestor of HEAD, so a cherry-picked proof never counts. Reproduced with the built binary: a starter policy plus evidence.attached on close; review, rebase, commit a fix, record proof and commit it, then follow the advice (reset to the reviewed tip, cherry-pick both commits, git merge main). merge --check then reports 'FAIL evidence.attached: ... found 0'; the cherry-picked proof pins 3fc2e32, which is not in the branch history. Cherry-picking review records is worse, because 'cherry-pick any commits' sweeps them in. An approval recorded after the rebase and cherry-picked back makes merge refuse again with the same 'pins a commit outside the work branch' error, so the advice loops until the record file is deleted. The claim that review events pin rewritten commits is true, but the fix is incomplete. Error and docs should say: cherry-pick only the code and plan commits made after the rebase; leave out (or git rm) review and evidence records made since the rebase; then record proof and reviews again on the restored branch.
