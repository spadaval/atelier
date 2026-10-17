---
schema: atelier.review
schema_version: 1
id: atelier-ompn
title: 'crates/atelier-app/src/review_gate.rs:893: claimed_before_required reads .atelier/workflow.yaml from the branch''s own claim commit, but land and merge otherwise judge claims by the trusted base policy (base_policy::from_captured), so a branch can exempt itself from report parts the base requires. Reproduced in a scratch repo whose base requires all four parts: on the owner''s branch, commit a weakened workflow.yaml together with a child record edited to done with an Outcome-only Close Reason, then restore workflow.yaml in the next commit; git diff main...HEAD shows no workflow.yaml change and atelier land passes (control without the policy dance fails with required_field.report.*). The same path serves merge and the owner''s own claim. Fix: take the then-policy from base history, not the branch, e.g. resolve the policy at git merge-base <introducing commit> <base_commit> via base_policy::from_captured (that still exempts the atelier-tqgi case of a child closed before the base changed), and add a session test that weakens and restores the policy around a claim.'
target: atelier-sfsq
identity: Independent reviewer 2 (Claude subagent)
commit: ffef9991c39f636085b51fd89b6ba2829730f58b
created_at: 2026-10-17T18:12:34.978619775Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/review_gate.rs:893: claimed_before_required reads .atelier/workflow.yaml from the branch's own claim commit, but land and merge otherwise judge claims by the trusted base policy (base_policy::from_captured), so a branch can exempt itself from report parts the base requires. Reproduced in a scratch repo whose base requires all four parts: on the owner's branch, commit a weakened workflow.yaml together with a child record edited to done with an Outcome-only Close Reason, then restore workflow.yaml in the next commit; git diff main...HEAD shows no workflow.yaml change and atelier land passes (control without the policy dance fails with required_field.report.*). The same path serves merge and the owner's own claim. Fix: take the then-policy from base history, not the branch, e.g. resolve the policy at git merge-base <introducing commit> <base_commit> via base_policy::from_captured (that still exempts the atelier-tqgi case of a child closed before the base changed), and add a session test that weakens and restores the policy around a claim.
