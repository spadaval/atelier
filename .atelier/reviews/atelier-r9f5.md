---
schema: atelier.review
schema_version: 2
id: atelier-r9f5
title: finding on atelier-8myy @5f7737c1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 5f7737c1d5c68bc04379a944f589437a75e86734
created_at: 2026-11-11T02:00:34.337031819Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: next/crates/atelier-app/src/view.rs:147 The lead does not see stale approvals. needs() changes the caller to the holder but retains checkout: None from entries(), so unmet(Merge) cannot evaluate SinceApproval. Reproduced in a scratch repository: ready mission led by lead; impl starts, submits and receives approval; commit hello.txt after approval. impl's status names hello.txt and requires a fresh review, while lead's status lists only approved since, with no unmet condition. This misses Done when 3. Gather the branch-tip approval comparison for led items and pass it into their holder-perspective facts; add a regression asserting the lead sees the same committed staleness condition.
