---
schema: atelier.review
schema_version: 1
id: atelier-zrt0
title: finding on atelier-ot1c @c4b7c6f5
target: atelier-ot1c
identity: claude-reviewer-ot1c
commit: c4b7c6f59fd4657696d74f90954c132a8bb4ac6d
created_at: 2026-11-04T22:40:57.184103186Z
event:
  event: finding
  recommendation: defer
---

low: scripts/merge-ci-passed.sh:14 Outside atelier merge's synthetic merge checkout HEAD^2 does not exist and the script dies with git's 'ambiguous argument HEAD^2'; the review brief lists it among merge checks for reviewers to run themselves. Suggest a guard: if HEAD has no second parent, print that the check only runs inside atelier merge's merge checkout (or fall back to HEAD vs origin/master) and exit 2.
