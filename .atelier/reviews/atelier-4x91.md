---
schema: atelier.review
schema_version: 2
id: atelier-4x91
title: approval on atelier-45gr @3322fce8
target: atelier-45gr
identity: reviewer-45gr-fix
commit: 3322fce84a4188baadbd8d8533765983ed2c22b1
created_at: 2026-11-08T07:27:14.980028701Z
event:
  event: approval
---

Test-only fix: expected checkout path is canonicalized because freshness::linked_checkout prints git rev-parse --show-toplevel (resolved path, correct for users). Reproduced via symlinked TMPDIR on Linux: parent fails 2 of 11, 3322fce8 passes 11 of 11. cargo fmt clean.
