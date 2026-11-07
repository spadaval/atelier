---
schema: atelier.review
schema_version: 2
id: atelier-kby7
title: finding on atelier-md0v @c4586155
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: c4586155e0908d88b52e5855c91c02fc66092091
created_at: 2026-11-07T09:35:44.710810654Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 94bcb1a28dcceeb48924425c021d538c68c78587
    recorded_at: 2026-11-07T09:49:15.113913646Z
    resolution_commit: 1e6f9b6737141e10a54da8571cd76ce2338deed6
    reason: Fixed in 1e6f9b67.
---

low: crates/atelier-cli/src/commands/review_request.rs:194 the brief's Findings so far marks every fix_now finding without a disposition 'open until an approval after its fix' and sorts it first, even after an approval has resolved it, while review list (through review_gate::fix_approval) shows the same finding settled, 'approved since by <approval>'. Before this change a resolved fix_now finding showed its disposition. A later round's reviewer, for example after more code, is told a settled finding is open, and the brief and review list now judge settlement by two separate rules. Fix: use review_gate::fix_approval here as review list does, and show 'approved since by <approval>' when one resolves the finding.
