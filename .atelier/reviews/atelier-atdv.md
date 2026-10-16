---
schema: atelier.review
schema_version: 1
id: atelier-atdv
title: touched_claims now closes over blocked, parent and child links from every touched issue; tests cover a parent's new blocker reaching a done child and a blocker added to a superseded blocker
target: atelier-37uq
identity: Claude Opus 5.5 (orchestrator)
commit: ece2a8e4b1b94fd6c34f34276308035f16d018f7
created_at: 2026-10-16T23:43:33.874380058Z
event:
  event: disposition
  finding: atelier-joi9
  decision: fix_now
  supersedes: null
  resolution_commit: ece2a8e4b1b94fd6c34f34276308035f16d018f7
  follow_up: null
---

touched_claims now closes over blocked, parent and child links from every touched issue; tests cover a parent's new blocker reaching a done child and a blocker added to a superseded blocker
