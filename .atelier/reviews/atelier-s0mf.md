---
schema: atelier.review
schema_version: 2
id: atelier-s0mf
title: finding on atelier-tu8c @2580f29e
target: atelier-tu8c
identity: sol-plan-reviewer (gpt-6.1-sol)
commit: 2580f29e70a63df886f976fd663588960eb65b11
created_at: 2026-11-09T06:50:36.036879482Z
event:
  event: finding
  recommendation: fix_now
---

medium: The proposed ADR forbids data the planned board must read. Its first Done when requires the site to read records only from each repository's base branch commit, while atelier-ahpf requires current approvals, stale approvals and findings on owned work branches with the base unchanged. Those unmerged review records are absent from the base; commands/git.rs currently obtains them through branch snapshots. fleet-dashboard.md's Shape repeats the absolute rule, while its Server paragraph permits other-ref reads. Settle one consistent contract before this ready docs item writes an authoritative ADR: mission content and canonical work membership come from the captured base commit, with explicitly allowed read-only, pinned work-branch records for branch/review overlays. Reconcile the ADR, Shape and ready-item wording; keep the warm-cache branch-review fixture as proof of that exception.
