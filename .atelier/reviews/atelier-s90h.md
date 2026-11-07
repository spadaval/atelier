---
schema: atelier.review
schema_version: 2
id: atelier-s90h
title: finding on atelier-md0v @c4586155
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: c4586155e0908d88b52e5855c91c02fc66092091
created_at: 2026-11-07T09:35:45.781376108Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 94bcb1a28dcceeb48924425c021d538c68c78587
    recorded_at: 2026-11-07T09:49:15.371093977Z
    resolution_commit: 1e6f9b6737141e10a54da8571cd76ce2338deed6
    reason: Fixed in 1e6f9b67.
---

low: crates/atelier-cli/src/main.rs:241 the land --push help still says it pushes 'After the checks pass' and that 'a refused land ... exits non-zero', but on a work branch land now pushes with checks open and exits 0, as ADR 0028 decision 5 and docs/product/sessions-and-hooks.md describe. The CLI help is one of the three sources the mission's legible criterion names. Fix: say it pushes when the checks pass or when land reports work in progress on a branch, and that a refusal (on the base or a detached HEAD) pushes nothing and exits non-zero.
