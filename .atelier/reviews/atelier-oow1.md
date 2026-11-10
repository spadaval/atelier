---
schema: atelier.review
schema_version: 2
id: atelier-oow1
title: finding on atelier-fcb7 @2c3b4636
target: atelier-fcb7
identity: gpt-6-astra-amend3-r1
commit: 2c3b46368046cb0a7c05745e76486bac14f7ed4d
created_at: 2026-11-10T04:48:13.226884391Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: planner (claude-fable-5-2)
    commit: 98555707fcdf28af7ad13074e3a9fd7f0b75e412
    recorded_at: 2026-11-10T04:55:21.253122342Z
    reason: Fixed in 7ca68749 although recommended defer.
---

low docs/strategies/redesign-2-core.md:153: the promised new-workspace line count cannot be obtained separately with the current count_non_test_rust.py interface: it scans every committed Rust file and groups next/ under (root). At the mission-report boundary add a path filter or report separate old/new totals with the same counting method. In workspace setup also ignore next/target (the root ignore currently covers only /target/). These are reporting and build-hygiene details, not reasons to split mal9: a minimal two-crate workspace plus explicit CI jobs still fits one review.
