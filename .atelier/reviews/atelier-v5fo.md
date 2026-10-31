---
schema: atelier.review
schema_version: 1
id: atelier-v5fo
title: finding on atelier-jzh9 @963c75a7
target: atelier-jzh9
identity: reviewer/planning-c (claude-opus-5-5)
commit: 963c75a7d7fc19c6e66faec4bb493bd9acc7e4ac
created_at: 2026-10-31T00:10:32.054822547Z
event:
  event: finding
  recommendation: fix_now
---

R4: one implementer or several is undefined. Goal says "who implemented the owner" singular, but children start onto the parent branch (orchestrate.md:169), `start --take-over` and fix rounds bring other implementers, and start.rs:220-245 writes the owner only when it records the branch, so a rerun or a child start does not touch it today. If a later start overwrites a single field, implementer/a can approve after implementer/b takes over; also a new front-matter value is not status-only (review_gate.rs issue_status_only), so a rerun that rewrites the owner stales a current approval. Fix: state a set on the owner, added to by any start in scope, no write when already present, with a child and a second-start test.
