---
schema: atelier.review
schema_version: 2
id: atelier-iurp
title: finding on atelier-va99 @76b870a3
target: atelier-va99
identity: independent-planning-review-astra
commit: 76b870a36854bdd6d98362946ec82eec502305bf
created_at: 2026-11-11T07:04:44.478070412Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-va99.md:43 requires the service to run from this item’s merged commit, and line 44 requires its installed-service proof before Done when is complete. This task cannot satisfy those lines before its own merge. Ready Work requires task criteria to hold before merge; post-merge operational work belongs on the base. Keep branch-build author proof in va99, and explicitly assign installation/update to the merged commit to a post-merge base item or the validation setup before oc9k runs its acceptance checks, with the dependency recorded. Recommendation: FIX NOW; the current ordering cannot be executed without waiving a Done when line.
