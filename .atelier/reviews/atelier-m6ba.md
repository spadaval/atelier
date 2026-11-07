---
schema: atelier.review
schema_version: 2
id: atelier-m6ba
title: finding on atelier-4pzj @8889e8b5
target: atelier-4pzj
identity: claude-reviewer-4pzj
commit: 8889e8b5c0798c42541dcb49ade0b8eb513e83c8
created_at: 2026-11-04T23:05:00.122006374Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-author-4pzj
    commit: 5d64b6020a5dbfcdf4b32ecfcb579568ca9b80a9
    recorded_at: 2026-11-04T23:05:42.635095938Z
    resolution_commit: 5d64b6020a5dbfcdf4b32ecfcb579568ca9b80a9
    reason: Fixed in 5d64b602
    migrated_from: atelier-jsf8
---

low: atelier-om6l and atelier-dvnv keep their Unknowns sections word for word after the decision notes were added; om6l's still ends 'but the human decides', and the brief renders these as open questions at plan approval. Fix: remove or rewrite the resolved Unknowns to point at the 2026-11-04 decision notes.
