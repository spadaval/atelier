---
schema: atelier.review
schema_version: 1
id: atelier-nzzz
title: 'crates/atelier-app/src/review_gate.rs:664-677 prefetches claim_history_query for every applicable claim with proof or findings whenever evaluate_scope is global. That includes EvaluationKind::CurrentClaims (session.rs:430, historical=false). In that mode claim_endpoint is never reached: newest_failure runs with fresh=true (review_gate.rs:889), so proof_matches takes the evidence_is_fresh branch, and original_approval_valid (:772) runs only when historical. So every land walks the full path-limited history once per newly claimed issue with proof, and nothing reads the result. It runs in parallel and the answers are still correct, but it is wasted work in exactly the code this issue speeds up. Fix: pass an empty claim set unless historical. On missed claims: in Historical mode the filter is exact. claim_endpoint is reached only through proof_matches for ''validates'' evidence (same link filter) and through original_approval_valid for finding targets that are in applicable. So no claim is missed, and the prefetched query is byte-identical to the one claim_endpoint issues (same pinned HEAD, same args, same order).'
target: atelier-v5tt
identity: Independent reviewer (Claude subagent)
commit: 76a364da360c29b9afe0cdf075c9934ba73b4e5c
created_at: 2026-10-16T21:59:55.832083370Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/review_gate.rs:664-677 prefetches claim_history_query for every applicable claim with proof or findings whenever evaluate_scope is global. That includes EvaluationKind::CurrentClaims (session.rs:430, historical=false). In that mode claim_endpoint is never reached: newest_failure runs with fresh=true (review_gate.rs:889), so proof_matches takes the evidence_is_fresh branch, and original_approval_valid (:772) runs only when historical. So every land walks the full path-limited history once per newly claimed issue with proof, and nothing reads the result. It runs in parallel and the answers are still correct, but it is wasted work in exactly the code this issue speeds up. Fix: pass an empty claim set unless historical. On missed claims: in Historical mode the filter is exact. claim_endpoint is reached only through proof_matches for 'validates' evidence (same link filter) and through original_approval_valid for finding targets that are in applicable. So no claim is missed, and the prefetched query is byte-identical to the one claim_endpoint issues (same pinned HEAD, same args, same order).
