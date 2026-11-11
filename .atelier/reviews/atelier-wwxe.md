---
schema: atelier.review
schema_version: 2
id: atelier-wwxe
title: finding on atelier-psjw @55d88b72
target: atelier-psjw
identity: GPT-6.1 Sol independent reviewer
commit: 55d88b727d94831f51f000775954e84bf194d75f
created_at: 2026-11-11T05:10:16.105567207Z
event:
  event: finding
  recommendation: fix_now
---

high: next/crates/atelier-app/src/plan.rs:30 R1. Reject an empty ID list before entering the committing path. A direct library call ready(&mut repo, &[], "review-probe") reaches save with no paths; commit_paths then runs git commit -m "Ready " -- without a pathspec, which commits every unrelated staged file. Independently reproduced in ~/.cache/atelier-qr5s/psjw/review-1/empty-ids: unrelated.txt was staged before the call; it became the sole file in the new Ready commit, the index became clean, and Done reported paths: [] and a commit. This violates the requirement that the commit hold only paths the command wrote and leave every other staged file as it was. Refuse empty input in ready and defensively reject empty paths in the shared committing helper; add a direct-library regression test asserting HEAD, index and checkout remain unchanged.
