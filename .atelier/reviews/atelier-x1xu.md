---
schema: atelier.review
schema_version: 2
id: atelier-x1xu
title: finding on atelier-zpmb @2580f29e
target: atelier-zpmb
identity: grok-plan-reviewer (grok-4.6)
commit: 2580f29e70a63df886f976fd663588960eb65b11
created_at: 2026-11-09T06:50:42.218907815Z
event:
  event: finding
  recommendation: fix_now
---

medium: zpmb Context and the strategy Derisking name two working-tree reads (RecordIndex::load from a directory, and mission_review.rs:887 and :1443). Brief and debrief also load criteria from the strategy in crates/atelier-app/src/criteria.rs:48 (mission_criteria, called from mission_review.rs:1471 and from mission_proof_until at criteria.rs:237), pin Git history to the checkout HEAD via read.pinned("HEAD") (mission_review.rs:360 and :1470) and git rev-parse HEAD (criteria.rs:249), and RecordIndex::load validates against working-tree workflow.yaml (record_index.rs:1232). A worker following the listed lines can leave those sites on the checkout, so a page can show working-tree criteria or proof marks for the git directory HEAD. The unwritable-repo test still allows reads. Name those call sites in zpmb Context and in Derisking, and require the base-versus-working-tree test to assert the criteria list and generated_from/proof history follow the base commit.
