---
schema: atelier.review
schema_version: 2
id: atelier-11uw
title: 'Medium: crates/atelier-cli/src/commands/mission_review.rs:1430 and :1525 The HTML renders each per-criterion receipt result as either pass (✓) or ✗/warn, so blocked and not_applicable results look identical to fail, while the Markdown prints the actual word (''fast blocked''). Reproduced: a receipt with --criterion fast=blocked shows ''fast blocked'' in Markdown and ''✗ fast'' in both the HTML timeline and Validation section; nothing else on the page says it was blocked rather than failed. Violates ''Markdown and HTML agree on facts''. Fix: show the result word (or a distinct mark/class per EvidenceResult, e.g. a neutral tone for blocked/n-a) and cover a non-pass, non-fail result in a test.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:07.487087347Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    recorded_at: 2026-10-18T07:07:54.883471895Z
    resolution_commit: 8151a566626693a44fc6dd9c06568de0a78f6328
    reason: Fixed in 8151a566
    migrated_from: atelier-22w1
---

Medium: crates/atelier-cli/src/commands/mission_review.rs:1430 and :1525 The HTML renders each per-criterion receipt result as either pass (✓) or ✗/warn, so blocked and not_applicable results look identical to fail, while the Markdown prints the actual word ('fast blocked'). Reproduced: a receipt with --criterion fast=blocked shows 'fast blocked' in Markdown and '✗ fast' in both the HTML timeline and Validation section; nothing else on the page says it was blocked rather than failed. Violates 'Markdown and HTML agree on facts'. Fix: show the result word (or a distinct mark/class per EvidenceResult, e.g. a neutral tone for blocked/n-a) and cover a non-pass, non-fail result in a test.
