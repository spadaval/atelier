---
schema: atelier.review
schema_version: 3
id: atelier-u5q8
title: approval on branch release-0.4.0 @a0187f90
branch: release-0.4.0
identity: gpt-6.1-sol
commit: a0187f9054a3539eb25652ef5ca8194e6f9e5c6c
created_at: 2026-11-09T21:01:40.609021567Z
event:
  event: approval
---

Verdict: merge. The changelog now accurately describes when start --base refuses from detached HEAD. Sizes: complexity low (one changelog sentence), value low (corrects a specific upgrade edge case), risk low (documentation only). Contract: pass. Fitness: pass. Re-reviewed git diff 3de58711 a0187f90 and finding atelier-7pjf. The diff contains the prior finding record plus the wording fix; a0187f90 itself changes only CHANGELOG.md. Source inspection confirms main.rs accepts --base, prints the deprecation note and ignores it; start.rs selects an existing recorded, owned or bare-ID branch and refuses detached HEAD only when the selected work branch does not exist. git::work_ref recognizes existing local branches, including the unrecorded bare-ID case behind atelier-7pjf. The replacement wording removes the incorrect recorded-branch prerequisite and introduces no further problem. Findings: none; atelier-7pjf is fixed and no fix_now remains in this re-review. Proof: source read at the pinned commit; git diff --check 3de58711 a0187f90 passed. Open questions: none. Residual risk and tests not run: no rebuild, CLI scenarios or brief proof rerun; this review covers only the changelog correction, as requested.
