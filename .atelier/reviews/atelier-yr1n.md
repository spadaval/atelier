---
schema: atelier.review
schema_version: 1
id: atelier-yr1n
title: finding on atelier-l7nz @1d23aab7
target: atelier-l7nz
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: 1d23aab77b622c215fbb69573237a52dfa28b73d
created_at: 2026-10-23T16:31:26.833380625Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/evidence_freshness.rs:250 judged_at_close decides 'judged as land/doctor do' by branch identity (owner != current branch name), but land and merge --check decide by whether the done claim is retained in the base (claim_nodes in session.rs: historical only if a parent of HEAD or the recorded base already has the claim; otherwise judged fresh at HEAD). Scenario (scratch /home/supadava/.cache/atelier-regroom-review/s3, policy evidence.attached min_count 1 on work close): owner O2 with child C2, C2 closed with a pass, then later code on the branch, O2 closed and approved but NOT merged. A second started branch P4 (from main) merges task/<O2> in. On P4, 'evidence list C2' prints 'Proof: ✓ pass 3jmu held at close 02ef5ea1', yet 'land' refuses 'atelier-l3s3 evidence.attached: expected at least 1 fresh passing evidence record; found 0' and 'merge P4 --check' refuses the same. Same false green on a detached HEAD at O2's tip (list says held at close; on O2's branch the identical state is stale and refused). The false-green direction is the harmful one and breaks the increment rule that marks agree with the gates. Suggested fix: replace the branch-name test with the gates' own discriminator, treat the claim as judged at close only when the claim commit is already in the base ref (policy base branch, or the claim_parents the gates use), else judge at HEAD; add a test with a sibling branch that merged an unmerged done claim after later code.
