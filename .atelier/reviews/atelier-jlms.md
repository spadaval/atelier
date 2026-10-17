---
schema: atelier.review
schema_version: 1
id: atelier-jlms
title: 'crates/atelier-cli/src/commands/work.rs:400 — the new wrong-kind message hard-codes the article: "{id} is a {issue_type}, not a mission". issue_types come from .atelier/workflow.yaml, so an adopter type starting with a vowel (incident, epic, improvement) prints ''atelier-x is a incident, not a mission'', the same ''a evidence'' grammar defect this item fixed in lint.rs. This repo''s own types (bug, decision, feature, spike, task, validation) are not affected. Fix: choose a/an from the first letter, or word it as ''is issue type incident''.'
target: atelier-qrwr
identity: Independent reviewer (Claude subagent)
commit: dcde3fc2e9f623ec96a1a5f679d8a24e0268a87f
created_at: 2026-10-17T07:16:43.765138219Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/work.rs:400 — the new wrong-kind message hard-codes the article: "{id} is a {issue_type}, not a mission". issue_types come from .atelier/workflow.yaml, so an adopter type starting with a vowel (incident, epic, improvement) prints 'atelier-x is a incident, not a mission', the same 'a evidence' grammar defect this item fixed in lint.rs. This repo's own types (bug, decision, feature, spike, task, validation) are not affected. Fix: choose a/an from the first letter, or word it as 'is issue type incident'.
