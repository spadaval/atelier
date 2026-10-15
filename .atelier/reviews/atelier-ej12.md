---
schema: atelier.review
schema_version: 1
id: atelier-ej12
title: 'Reviewed fdf8fd68 (one-pass graph validation). Before the first error, the new phases run the same checks in the same order as before, so fail-fast loading returns the same first error. The parent-commit binary (76cde2a9) and the new binary gave identical first errors for issue list and status on 6 invalid fixtures. Mutations (issue update and create) refuse invalid graphs and write nothing. No panic path: skipped edges never enter children, blocking or relations, so the self.issues[...] lookups only touch IDs that exist. The reviewer scenarios bwzf (cycle plus frobnicates) and qal1 (task child plus frobnicates) report both errors and exit 1. Cost is linear: 2000 and 8000 invalid issues report 4000 and 16000 unique findings in 0.34s and 1.36s, about the same as valid graphs. nextest with test-hooks: 592 of 592 pass. fmt and clippy are clean. 76cde2a9..HEAD changes only these three source files and .atelier records. Three low findings recorded (one defer, two no_action), none blocking.'
target: atelier-1ioo
identity: independent-reviewer-3-2026-10-15
commit: 8f13d015eda0d0d702e77bf318fb686565e62955
created_at: 2026-10-15T19:30:10.042531728Z
event:
  event: approval
---

Reviewed fdf8fd68 (one-pass graph validation). Before the first error, the new phases run the same checks in the same order as before, so fail-fast loading returns the same first error. The parent-commit binary (76cde2a9) and the new binary gave identical first errors for issue list and status on 6 invalid fixtures. Mutations (issue update and create) refuse invalid graphs and write nothing. No panic path: skipped edges never enter children, blocking or relations, so the self.issues[...] lookups only touch IDs that exist. The reviewer scenarios bwzf (cycle plus frobnicates) and qal1 (task child plus frobnicates) report both errors and exit 1. Cost is linear: 2000 and 8000 invalid issues report 4000 and 16000 unique findings in 0.34s and 1.36s, about the same as valid graphs. nextest with test-hooks: 592 of 592 pass. fmt and clippy are clean. 76cde2a9..HEAD changes only these three source files and .atelier records. Three low findings recorded (one defer, two no_action), none blocking.
