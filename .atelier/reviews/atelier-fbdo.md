---
schema: atelier.review
schema_version: 3
id: atelier-fbdo
title: approval on branch release-0.4.0 @34191b31
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 34191b316cf92b5171e0a54f1ca1a0b1ba7335fc
created_at: 2026-11-09T21:06:06.726838516Z
event:
  event: approval
---

Verdict: merge. The added 0.4.0 entry accurately tells users about the repository board and item pages that the release merge will include. Sizes: complexity low (one changelog bullet), value medium (documents a newly shipped feature), risk low (documentation only). Contract: pass. Fitness: pass. Re-reviewed git diff a0187f90 34191b31: the only additions are prior approval atelier-u5q8 and this bullet; commit 34191b31 itself changes only CHANGELOG.md. After git fetch origin, inspected origin/master at 1858e8a68588114ad299501992708ea91a86911a: the atelier-ahpf Report, crates/atelier-app/src/board.rs, and the Board.tsx, Item.tsx and App.tsx views and routes. The board renders In Progress with branch review state, Ready Work, Needs a Human, and Done in the Last 14 Days; RECENT_DAYS is 14 and the source counts whole UTC dates including today and the preceding 14 days. The bullet matches that user-facing description without claiming an exact rolling-hour cutoff. Item pages are routed per repository and item and display the record body, links and reviews. No material misstatement found. git log --first-parent 34c682ad..origin/master contains only the board/item-page merge e846c07f and tracker-only planning update 1858e8a6. The extracted shared CLI helpers retain their behavior; no additional user-visible release change is missing from this increment. Proof: source and commit history inspected; git diff --check a0187f90 34191b31 passed. Findings: none; no fix_now recommended. Open questions: none. Residual risk and tests not run: no build, runtime or browser tests rerun for this documentation-only review; it verifies the wording against shipped source, not the feature implementation as a whole.
