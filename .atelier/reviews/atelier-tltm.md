---
schema: atelier.review
schema_version: 1
id: atelier-tltm
title: docs/product/work-view-ordering.md:43-46 (edited here) — the row state rank and tie breakers are stated for work mission and work parent, but ScopedWork::new (work.rs:508-534) keeps issue_descendant_ids order and never calls work_order::ordered_work_indices, so work mission atelier-o3lm --all lists by ID with the active atelier-i2qq in the middle of backlog rows, and atelier-k8h8 --all puts done atelier-kops between ready rows. Pre-existing and outside this item's scope (ordering), but the doc this item revised now states a rank the views it names do not apply; file a follow-up to order Scoped Work by the shared order.
target: atelier-i2qq
identity: Independent reviewer (Claude subagent)
commit: c4b06e91043b5f0b07fce4a2cb9a0ba9e6cd2945
created_at: 2026-10-17T06:18:10.799196485Z
event:
  event: finding
  recommendation: defer
---

docs/product/work-view-ordering.md:43-46 (edited here) — the row state rank and tie breakers are stated for work mission and work parent, but ScopedWork::new (work.rs:508-534) keeps issue_descendant_ids order and never calls work_order::ordered_work_indices, so work mission atelier-o3lm --all lists by ID with the active atelier-i2qq in the middle of backlog rows, and atelier-k8h8 --all puts done atelier-kops between ready rows. Pre-existing and outside this item's scope (ordering), but the doc this item revised now states a rank the views it names do not apply; file a follow-up to order Scoped Work by the shared order.
