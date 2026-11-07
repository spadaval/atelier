---
schema: atelier.review
schema_version: 2
id: atelier-zrt0
title: finding on atelier-ot1c @c4b7c6f5
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: c4b7c6f59fd4657696d74f90954c132a8bb4ac6d
created_at: 2026-11-04T22:40:57.184103186Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author-ot1c
    commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    recorded_at: 2026-11-04T22:42:41.211392680Z
    resolution_commit: 9cb03c04eb92235c7d5e6d47e0f4704b1bdb5637
    reason: Fixed in 9cb03c04
    migrated_from: atelier-ax29
---

low: scripts/merge-ci-passed.sh:14 Outside atelier merge's synthetic merge checkout HEAD^2 does not exist and the script dies with git's 'ambiguous argument HEAD^2'; the review brief lists it among merge checks for reviewers to run themselves. Suggest a guard: if HEAD has no second parent, print that the check only runs inside atelier merge's merge checkout (or fall back to HEAD vs origin/master) and exit 2.
