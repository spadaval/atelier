---
schema: atelier.review
schema_version: 1
id: atelier-y49z
title: 'docs/product/retention-and-prune.md:22-26 together with prune.rs protect_referenced_candidates: kept_content records feed the fixpoint, so one decision or Background record keeps its whole connected terminal component forever. That includes parent epics, children, related records, and their evidence and reviews. Reasoned from the fixpoint, and confirmed that a plain record related from a kept decision shows ''referenced by retained issue'' and a review of a kept decision shows ''references retained issue''. This is correct for not leaving dangling links, but a Background on one child of a large closed epic now keeps the entire epic tree. The doc only says ''linked from, or linking to, a record that stays''. Fix: state the transitive consequence in the doc so Sumeet can confirm it is intended.'
target: atelier-r7cd
identity: Independent reviewer (Claude subagent)
commit: 85972cb43f9213363053eabf17e9a29dd7569afa
created_at: 2026-10-16T01:37:09.033795245Z
event:
  event: finding
  recommendation: defer
---

docs/product/retention-and-prune.md:22-26 together with prune.rs protect_referenced_candidates: kept_content records feed the fixpoint, so one decision or Background record keeps its whole connected terminal component forever. That includes parent epics, children, related records, and their evidence and reviews. Reasoned from the fixpoint, and confirmed that a plain record related from a kept decision shows 'referenced by retained issue' and a review of a kept decision shows 'references retained issue'. This is correct for not leaving dangling links, but a Background on one child of a large closed epic now keeps the entire epic tree. The doc only says 'linked from, or linking to, a record that stays'. Fix: state the transitive consequence in the doc so Sumeet can confirm it is intended.
