---
schema: atelier.review
schema_version: 2
id: atelier-bywd
title: finding on atelier-xtvh @7d3a0a0a
target: atelier-xtvh
identity: gpt-6.1-sol
commit: 7d3a0a0a47072197837aa59399efb11fd9fb94bc
created_at: 2026-11-10T19:14:30.679920067Z
event:
  event: finding
  recommendation: fix_now
---

High: next/crates/atelier-cli/src/merge.rs:59 R1: Reopening the checkout after the unlocked CI wait silently changes which item this invocation merges. before_lock discards its original item/branch identity, and read at lines 60-66 accepts whichever approved item is now checked out. Reproduced offline at the pinned head with the supplied FAKE_GH and a local bare origin: prepare demo-ab12 and demo-cd34, both approved for implementer t1; start merge on demo-ab12 with its CI in progress; after two polls switch that checkout to demo-cd34, whose CI has passed; let demo-ab12 CI pass. The original command exits 0, prints Merged demo-cd34 into main, pushes main, and main has Atelier-Merge: demo-cd34. It never merges demo-ab12. This is an unintended merge and publication, unlike the existing refusal when a checkout changes during merge checks. Carry the original item/branch identity across the wait and refuse if the checkout now names another branch; continue rereading tips, config, approval and checks for that same item. Add this branch-switch-during-wait regression test. Recommendation: fix_now. Verdict: merge after the blockers are fixed, since overlapping CI with review is valuable and no substantial scope reduction meets the contract. Sizes: complexity medium (bounded subprocess polling and two-phase merge validation), value high (the required latency reduction), risk high (the current wait can silently publish another item). Contract: fail on R2; the required scenarios pass, including the fix for atelier-0lkh. Fitness: fail on R1. Every added moving part otherwise traces to the requirements.
