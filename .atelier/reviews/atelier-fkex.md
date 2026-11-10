---
schema: atelier.review
schema_version: 2
id: atelier-fkex
title: finding on atelier-fcb7 @9394bb27
target: atelier-fcb7
identity: gpt-6-astra-amend3-r1
commit: 9394bb27f77fc5da005bab5e28496dbf8e6a0a27
created_at: 2026-11-10T04:47:26.355722655Z
event:
  event: finding
  recommendation: fix_now
---

high atelier-syyj Scope: the copy list names the merged checkout, merge commit and push, then excludes everything else, but the existing integration safeguards are in merge.rs:1282-1370 outside those helpers and depend on mutation_lock and committed_snapshot. Copying only the named pieces can publish a tree checked against an obsolete base or overwrite concurrent checkout changes. Explicitly retain the integration contract: revalidate captured branch/base tips and checkout state after checks, serialize cooperative writers and refuse changed inputs; name the supporting primitives to extract and carry their race/failure contract tests. This is needed in the ready merge item, not a later step.
