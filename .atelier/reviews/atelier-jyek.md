---
schema: atelier.review
schema_version: 2
id: atelier-jyek
title: finding on atelier-hwxm @b67bf398
target: atelier-hwxm
identity: lighter-workflow reviewer for atelier-hwxm (claude-opus-5-5)
commit: b67bf398c4b20c3cebad1b92cbdcfbd604f51822
created_at: 2026-11-07T01:34:33.934051583Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (implementer)
    commit: b963395a1c2e4d80d1ffcc0a50f853aa576cf0fe
    recorded_at: 2026-11-07T01:36:09.005864408Z
    resolution_commit: b963395a1c2e4d80d1ffcc0a50f853aa576cf0fe
    reason: Below the tip the merge-and-migrate refusal now adds that the rebase rewrites the commits after it and they need review again, since records pinned to them no longer count; the doc sentence is corrected; the test checks the warning is absent at the tip and present below it.
---

low: crates/atelier-app/src/review_gate.rs:1116 and docs/product/reviews-and-merge.md:97 The new j5oq refusal prints the same leave-the-merge-out rebase (git rebase --autostash --onto M~1 M) as the generic refusal, but drops the generic one's caveat for a merge below HEAD ('which rewrites the commits after it: record fresh proof and review for them', line 1121). Reproduced in fx4: merge main, commit later work, doctor: the message is identical to the HEAD case; following it works (append-only then passes) but rewrites 'later work' to a new SHA, so any review or evidence pinned to the commits after the merge is orphaned, against Atelier's own rule 'Do not rebase once review records exist'. The doc says 'there is no need to reset the branch', yet the named sequence starts by leaving the merge out, which is a reset (HEAD case) or a rebase (otherwise). Fix: append the existing effect clause when the merge is not HEAD (or say record proof and review again for the replayed commits), and correct the doc sentence to say the merge is left out and, if it is not the tip, the commits after it are replayed and need fresh records.
