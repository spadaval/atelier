---
schema: atelier.review
schema_version: 1
id: atelier-0dm4
title: 'base_done_claim treats a parent record that fails to parse the same as a missing record, so an unchanged done record counts as a new done claim (git.rs:315-322: status_at turns parse errors into None through .ok(), and is_none_or then counts that as ''differs from every parent''). Scenario: run ''atelier migrate epics --apply'' on the base on an already-done epic. The parent blob has issue_type epic, which the current parser rejects, so land reports ''land_base_done_claim ... marks atelier-ep1c done outside review'' and gives the start/supersede advice for an item whose status did not change. That error hides the real refusal (land_base_unreviewed for .atelier/workflow.yaml). Reproduced on the built binary. Any later schema migration that the current parser cannot read back would flag every done record it touches in the same way. Only a missing blob should count as a new record. When the parent cannot be parsed, compare the raw front-matter status, or skip the record.'
target: atelier-fj1t
identity: Independent reviewer (Claude subagent)
commit: 2ceeb45048383ed43904476d464e056de03d8e8c
created_at: 2026-10-16T18:40:05.606761928Z
event:
  event: finding
  recommendation: defer
---

base_done_claim treats a parent record that fails to parse the same as a missing record, so an unchanged done record counts as a new done claim (git.rs:315-322: status_at turns parse errors into None through .ok(), and is_none_or then counts that as 'differs from every parent'). Scenario: run 'atelier migrate epics --apply' on the base on an already-done epic. The parent blob has issue_type epic, which the current parser rejects, so land reports 'land_base_done_claim ... marks atelier-ep1c done outside review' and gives the start/supersede advice for an item whose status did not change. That error hides the real refusal (land_base_unreviewed for .atelier/workflow.yaml). Reproduced on the built binary. Any later schema migration that the current parser cannot read back would flag every done record it touches in the same way. Only a missing blob should count as a new record. When the parent cannot be parsed, compare the raw front-matter status, or skip the record.
