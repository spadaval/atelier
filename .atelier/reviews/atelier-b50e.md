---
schema: atelier.review
schema_version: 1
id: atelier-b50e
title: 'Resolved: from_checkout is now fresh && matches_commits_to_head (evidence_matches_edges over pin..HEAD, no code_clean) && !code_clean, replacing the vacuous retained-claim proof_matches(.., false). Dirty only -> true; later commit only -> false; both -> false. The new test a_current_claim_whose_pass_a_later_commit_staled_does_not_blame_the_checkout replays the d7lz reproduction (clean and unrelated-dirty) and fails when the old expression is restored (scratch copy: clean case prints ''recorded before the uncommitted changes in this checkout''); it passes on the fix.'
target: atelier-x3bj
identity: Independent reviewer (Claude subagent)
commit: 409a44549696c2d99627a1b2c48e94761455bf73
created_at: 2026-10-16T20:28:39.101630040Z
event:
  event: disposition
  finding: atelier-d7lz
  decision: fix_now
  supersedes: null
  resolution_commit: cd50ea4e1aa7054d4111e659643f6cbe0f563d81
  follow_up: null
---

Resolved: from_checkout is now fresh && matches_commits_to_head (evidence_matches_edges over pin..HEAD, no code_clean) && !code_clean, replacing the vacuous retained-claim proof_matches(.., false). Dirty only -> true; later commit only -> false; both -> false. The new test a_current_claim_whose_pass_a_later_commit_staled_does_not_blame_the_checkout replays the d7lz reproduction (clean and unrelated-dirty) and fails when the old expression is restored (scratch copy: clean case prints 'recorded before the uncommitted changes in this checkout'); it passes on the fix.
