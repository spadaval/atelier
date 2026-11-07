---
schema: atelier.review
schema_version: 2
id: atelier-x2u5
title: finding on atelier-bqbz @709ebff0
target: atelier-bqbz
identity: lighter-workflow reviewer for atelier-bqbz (claude-opus-5-5)
commit: 709ebff0842519d604ec8d2c3fcc9926aee49548
created_at: 2026-11-07T03:14:14.397758036Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/atelier-bqbz.md:54 The no-migrate decision says a repository that wants the smaller set copies the starter's statuses and transitions, but it does not say that records in blocked or review must move first. I reproduced this with this branch's binary: a record in status review under the new starter makes 'atelier check' report workflow_issue_status_invalid, and every work view fails with the same error. astromech has such a record on a live branch today (astromech-l5c6). Nothing breaks now, because no repository's policy changes. Fix: add one sentence to docs/product/workflow-configuration.md:63-68 (move blocked/review records to in_progress or todo before dropping the statuses), or ship the migrate step when a repository first adopts the smaller set.
