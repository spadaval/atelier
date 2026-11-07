---
schema: atelier.review
schema_version: 2
id: atelier-9l63
title: 'review_gate.rs:160-187 work_edges trusts the base argument, but session.rs land passes claim_base = HEAD^ when there is no owner (on the base branch or an unowned branch). Every merge before HEAD has its sides as ancestors of HEAD^, so the ''side is on the base'' test is vacuous there. Example on main: approval pinned at P, then a clean merge M of an unreviewed branch X, then a commit at HEAD that changes the claim. evaluate_claims now excludes M and X''s commits and treats the approval as current, where before it was stale. On the base, check_base_commits still rejects X''s unpushed code commits, so this is not exploitable today. With no upstream, that check returns ''Base commits unchecked'', but then nothing gates direct base commits anyway. Suggested fix: pass the base ref tip, or require each side to be an ancestor of the policy base ref, rather than HEAD^.'
target: atelier-n7g1
identity: Independent reviewer (Claude subagent)
commit: 1f9c88ded59b5f0bcd19473fb6e9e9c7753f45ca
created_at: 2026-10-16T18:31:51.198881977Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 1b916f400b7c37ed7bb110695d534e4236a85ba7
    recorded_at: 2026-10-16T18:33:32.950528217Z
    follow_up: atelier-7hh4
    reason: Deferred to atelier-7hh4 as recommended; not exploitable through atelier merge.
    migrated_from: atelier-wiyb
---

review_gate.rs:160-187 work_edges trusts the base argument, but session.rs land passes claim_base = HEAD^ when there is no owner (on the base branch or an unowned branch). Every merge before HEAD has its sides as ancestors of HEAD^, so the 'side is on the base' test is vacuous there. Example on main: approval pinned at P, then a clean merge M of an unreviewed branch X, then a commit at HEAD that changes the claim. evaluate_claims now excludes M and X's commits and treats the approval as current, where before it was stale. On the base, check_base_commits still rejects X's unpushed code commits, so this is not exploitable today. With no upstream, that check returns 'Base commits unchecked', but then nothing gates direct base commits anyway. Suggested fix: pass the base ref tip, or require each side to be an ancestor of the policy base ref, rather than HEAD^.
