---
schema: atelier.review
schema_version: 1
id: atelier-15pp
title: 'crates/atelier-app/src/record_history.rs:236-251 with crates/atelier-app/src/mission_review.rs:122-128: parent_joined is looked up only among directly linked items, so a grandchild (parent is itself a child) is always judged as if its parent were planned scope. A child added after the start is flagged together with any grandchild listed under it, while a linked item added after the start is flagged without its children. Both are genuinely new scope, so the report is not wrong, only uneven. No action unless nested scope becomes common; if it does, record each child''s own joining position in joined_at and look parents up there.'
target: atelier-rx22
identity: Independent reviewer 2 (Claude subagent)
commit: 518ba162bcf5a01d3faa37694526c38836d0474b
created_at: 2026-10-17T19:45:10.596684918Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/record_history.rs:236-251 with crates/atelier-app/src/mission_review.rs:122-128: parent_joined is looked up only among directly linked items, so a grandchild (parent is itself a child) is always judged as if its parent were planned scope. A child added after the start is flagged together with any grandchild listed under it, while a linked item added after the start is flagged without its children. Both are genuinely new scope, so the report is not wrong, only uneven. No action unless nested scope becomes common; if it does, record each child's own joining position in joined_at and look parents up there.
