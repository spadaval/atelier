---
schema: atelier.review
schema_version: 1
id: atelier-dyke
title: 'crates/atelier-app/src/work_classification.rs:219 is_context: (a) index.get_subissues clones every child Issue just to read its status; it only runs for Ready-classified items and children are one level deep, so cost is O(total children) per classification pass and acceptable, though index.child_ids plus a status lookup would avoid the clones. (b) With no workflow policy, status_category is always None so a parent whose children are all done stays Context; this matches the existing no-policy degradation (nothing is ever classified Done without a policy), so no change needed unless no-policy mode is meant to be first-class.'
target: atelier-ymmb
identity: Independent reviewer (Claude subagent)
commit: b04dabfa30b96e84fe1b5ea9745987123c3a026f
created_at: 2026-10-16T01:03:49.668414628Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/work_classification.rs:219 is_context: (a) index.get_subissues clones every child Issue just to read its status; it only runs for Ready-classified items and children are one level deep, so cost is O(total children) per classification pass and acceptable, though index.child_ids plus a status lookup would avoid the clones. (b) With no workflow policy, status_category is always None so a parent whose children are all done stays Context; this matches the existing no-policy degradation (nothing is ever classified Done without a policy), so no change needed unless no-policy mode is meant to be first-class.
