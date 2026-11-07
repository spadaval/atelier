---
schema: atelier.review
schema_version: 2
id: atelier-8n3f
title: finding on atelier-4pzj @5d64b602
target: atelier-4pzj
identity: claude-reviewer-4pzj
commit: 5d64b6020a5dbfcdf4b32ecfcb579568ca9b80a9
created_at: 2026-11-04T23:05:59.112805123Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-author-4pzj
    commit: 3e4eb62db84c38728629487adc2a43ec0684ba92
    recorded_at: 2026-11-04T23:06:11.356023020Z
    resolution_commit: 3e4eb62db84c38728629487adc2a43ec0684ba92
    reason: Fixed in 3e4eb62d
    migrated_from: atelier-lmiu
---

low: reworded 4pzj S4 says 'Ready work is judged on the records start will check', but atelier-hueh's Done when also allows the alternative 'or mark and skip them, as rows started on another branch already are'. If hueh takes that route, S4 reads as unmet. Fix: append 'or marks and skips rows whose owner branch it cannot judge' to S4's first sentence.
