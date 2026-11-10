---
schema: atelier.review
schema_version: 2
id: atelier-xlbf
title: finding on atelier-fcb7 @8f1d6bfc
target: atelier-fcb7
identity: gpt-6-astra-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:32:22.026373579Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:360-363 (atelier-qy7l still open): the step-5 rollback reverts only repositories already pushed. If conversion or verify fails in the current repository before its push, that repository retains dirty or committed schema-3 records while rollback reinstalls the old binary. The before-step-3 row also assumes conversion reached a commit. Restore the current unpublished conversion as well as reverting published conversions, using each repository's recorded starting state; rehearse failures before commit and after commit but before push.
