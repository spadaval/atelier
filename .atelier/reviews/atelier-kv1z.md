---
schema: atelier.review
schema_version: 1
id: atelier-kv1z
title: 'crates/atelier-cli/src/commands/start.rs:52-66 vs crates/atelier-app/src/work_classification.rs:206 — the item''s symptom still reproduces through a different mechanism. A scoped view''s ''Start the top ready item'' can name an item that start refuses when the item''s owner branch already exists and its records are behind the checkout (repro in the work-view-ordering finding). work ready lists the same row, so its dependency_readiness_batch gate does not catch it. This predates the change and the change does not make it worse. File a follow-up: either judge readiness against the owner branch''s records, or mark and skip such rows like in-flight ones.'
target: atelier-6wyv
identity: Independent reviewer (Claude subagent)
commit: 1380e97a1504d0cb1859aaaee518e7d616ea02ea
created_at: 2026-10-17T08:53:38.659974072Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/start.rs:52-66 vs crates/atelier-app/src/work_classification.rs:206 — the item's symptom still reproduces through a different mechanism. A scoped view's 'Start the top ready item' can name an item that start refuses when the item's owner branch already exists and its records are behind the checkout (repro in the work-view-ordering finding). work ready lists the same row, so its dependency_readiness_batch gate does not catch it. This predates the change and the change does not make it worse. File a follow-up: either judge readiness against the owner branch's records, or mark and skip such rows like in-flight ones.
