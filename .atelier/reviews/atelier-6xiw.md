---
schema: atelier.review
schema_version: 2
id: atelier-6xiw
title: finding on atelier-md0v @c4586155
target: atelier-md0v
identity: independent reviewer subagent (claude-opus-5-5), atelier-md0v
commit: c4586155e0908d88b52e5855c91c02fc66092091
created_at: 2026-11-07T09:35:32.715107976Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: orchestrator (claude-opus-5-5), mission atelier-mmpv
    commit: 94bcb1a28dcceeb48924425c021d538c68c78587
    recorded_at: 2026-11-07T09:49:14.521721093Z
    resolution_commit: 1e6f9b6737141e10a54da8571cd76ce2338deed6
    reason: Fixed in 1e6f9b67.
---

medium: crates/atelier-app/src/review_gate.rs:728 fix_resolved takes a later pin plus a later created_at as proof that the approval saw a fix_now finding, but an approval pinned to a commit that does not hold the finding's record never saw it. Reproduced with this branch's binary in a scratch repo: reviewer R1 gets a brief at C1; the author commits a note (C2); reviewer R2 gets a brief at C2 that lists 'Findings so far: None'; R1 records a fix_now finding pinned to C1 (--commit-records); R2 then approves C2. merge passes with findings.disposed green, review list shows the finding 'approved since by' R2's approval, and land and doctor on the base pass, although nothing was fixed. That breaks the item's stated purpose ('so an approval that never saw the finding cannot resolve it') and the constraint that no fix_now finding counts as resolved without an approval that saw it; it arises whenever parallel reviewers get briefs at different heads. Fix: also require that the approval's pinned commit holds the finding's record (read.entry(approval.commit, .atelier/reviews/<finding>.md) is some), which keeps the normal flow (finding committed with --commit-records, fix, re-review) working; add this scenario as a test. A reviewer who omits --commit and so pins a HEAD that already holds another reviewer's finding commit stays a residual risk.
