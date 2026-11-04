---
schema: atelier.review
schema_version: 1
id: atelier-r3yo
title: finding on atelier-4pzj @8889e8b5
target: atelier-4pzj
identity: claude-reviewer-4pzj
commit: 8889e8b5c0798c42541dcb49ade0b8eb513e83c8
created_at: 2026-11-04T23:04:59.988891413Z
event:
  event: finding
  recommendation: fix_now
---

low: atelier-dvnv's decision (cheap subset: records committed, base commits, owner claims) does not obviously cover two failures its Done when still requires a test for: an owner branch with a done child blocked by an open issue (land invariants) and an owner branch behind its base. The Unknown about --quiet is also left unanswered. Fix: either name those checks in the subset or narrow Done when to the subset, and reduce the Unknown to the --quiet question.
