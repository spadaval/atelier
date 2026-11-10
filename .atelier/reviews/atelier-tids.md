---
schema: atelier.review
schema_version: 2
id: atelier-tids
title: finding on atelier-qr5s @8f1d6bfc
target: atelier-qr5s
identity: grok-4.6-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:37:17.690828732Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:337-348 and atelier-wd1i: runbook step 1 stops while any item is in progress, and step 6 closes the cutover item with atelier close, which only accepts a draft or ready item with no branch. After u1ko merges, the next habitual act is atelier start atelier-wd1i, which makes step 1 fail and leaves close unreachable. Smallest fix: state that wd1i is never started, is run from the base, and is closed there with a result.
