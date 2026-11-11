---
schema: atelier.review
schema_version: 2
id: atelier-w4h2
title: finding on atelier-u1ko @d41d3bcd
target: atelier-u1ko
identity: GPT-6 Astra independent planning reviewer
commit: d41d3bcd1be9bacd36781ba370621136144f6a60
created_at: 2026-11-11T04:50:09.925215328Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/atelier-u1ko.md:76 The trial requires every clone's base to equal its pre-trial state even for trial 5 after the conversion was pushed, while line 75 correctly requires reverting and pushing. A revert preserves history and produces a new base SHA, so this acceptance condition cannot hold without the prohibited history rewrite. Define equality of restored tracked tree, records and hooks separately from commit identity, require the expected revert history and local/remote agreement on the published route, and exact ref restoration only for the unpublished route. Keep G's rollback proof consistent.
