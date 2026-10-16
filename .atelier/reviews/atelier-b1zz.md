---
schema: atelier.review
schema_version: 1
id: atelier-b1zz
title: 'Resolved: review_gate.rs:668 prefetches claims only when historical. claim_endpoint has two callers: proof_matches with fresh=false and original_approval_valid. proof_matches gets fresh=!historical from newest_failure (review_gate.rs:889) and from workflow validation (review_gate.rs:950 to evidence_attached). original_approval_valid runs only under the historical guard (:768). So CurrentClaims and the non-global kinds never read a claim walk, and Historical keeps the exact filter the earlier review checked.'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 3841028c54e10d40fe6d109c806768be33cf0b2c
created_at: 2026-10-16T22:08:59.688593901Z
event:
  event: disposition
  finding: atelier-nzzz
  decision: fix_now
  supersedes: null
  resolution_commit: bc1ae1c335dff65e1782a9cea9c7736d5d499f3f
  follow_up: null
---

Resolved: review_gate.rs:668 prefetches claims only when historical. claim_endpoint has two callers: proof_matches with fresh=false and original_approval_valid. proof_matches gets fresh=!historical from newest_failure (review_gate.rs:889) and from workflow validation (review_gate.rs:950 to evidence_attached). original_approval_valid runs only under the historical guard (:768). So CurrentClaims and the non-global kinds never read a claim walk, and Historical keeps the exact filter the earlier review checked.
