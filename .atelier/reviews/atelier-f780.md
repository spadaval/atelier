---
schema: atelier.review
schema_version: 1
id: atelier-f780
title: 'crates/atelier-app/src/workflow_policy.rs:63-64 — the mission-has-parent hint always prints `atelier issue link {parent_id} {mission} --role advances`, even when the parent is itself a mission. Reproduced in a fixture: `atelier issue update M2 --parent M` (both missions) suggests `atelier issue link M M2 --role advances`, and running that is refused with workflow_issue_link_type_invalid ("M (mission) cannot advance a mission"). On master the hint used generic placeholders and was never wrong, so this is a regression: the recovery step now names a concrete command that fails. Fix: look up the parent type in the index and keep `<issue-id>` in the source slot when the parent is a mission (it is already loaded further down for the non-mission case). Add a mission-under-mission case to test_hints_name_real_ids_instead_of_placeholders.'
target: atelier-6xxb
identity: Independent reviewer (Claude subagent)
commit: b6ffbc65aaed76df1da64ae0f4046e7d307e1a4e
created_at: 2026-10-17T07:38:01.242609278Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/workflow_policy.rs:63-64 — the mission-has-parent hint always prints `atelier issue link {parent_id} {mission} --role advances`, even when the parent is itself a mission. Reproduced in a fixture: `atelier issue update M2 --parent M` (both missions) suggests `atelier issue link M M2 --role advances`, and running that is refused with workflow_issue_link_type_invalid ("M (mission) cannot advance a mission"). On master the hint used generic placeholders and was never wrong, so this is a regression: the recovery step now names a concrete command that fails. Fix: look up the parent type in the index and keep `<issue-id>` in the source slot when the parent is a mission (it is already loaded further down for the non-mission case). Add a mission-under-mission case to test_hints_name_real_ids_instead_of_placeholders.
