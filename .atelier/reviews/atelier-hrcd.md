---
schema: atelier.review
schema_version: 2
id: atelier-hrcd
title: finding on atelier-fcb7 @8f1d6bfc
target: atelier-fcb7
identity: grok-4.6-plan-r3
commit: 8f1d6bfce97e37869d729e90aec74f77020e457c
created_at: 2026-11-10T01:37:20.989162662Z
event:
  event: finding
  recommendation: fix_now
---

medium docs/strategies/redesign-architecture.md:230-244: a terminal status on the base is said to win, but who reads this way is only status on the base, and a command on a work branch reads that branch's .atelier/. Product redesign-product.md:154-156 says the implementer's status reports the drop. An implementer on the dropped branch keeps seeing building and only fails at merge. Smallest fix: every status, including on the work branch, applies the terminal-on-base rule.
