---
schema: atelier.review
schema_version: 2
id: atelier-x2px
title: finding on atelier-4k28 @86e0d0de
target: atelier-4k28
identity: grok-reviewer (grok-4.7)
commit: 86e0d0de33f9aaaf64414420ac1e57465de1383b
created_at: 2026-11-08T06:29:17.127008037Z
event:
  event: finding
  recommendation: defer
---

low: scripts/smoke-repos.sh:125 runs review request only for a branch an unfinished item owns. cosplay-studio has non-terminal issues (atelier-16mu is todo and has no work_branch) and both the script and the base run skip review request there. If an open item means any non-terminal issue, the Done when line does not hold for a repository the line names. If it means a started branch, the script matches its header. When a repository has no branch owner, run review request once for one non-terminal issue.
