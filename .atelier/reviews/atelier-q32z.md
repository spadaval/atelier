---
schema: atelier.review
schema_version: 2
id: atelier-q32z
title: Historical acceptance lets invalid self-approval or stale independent approval become valid after an unrelated commit.
target: atelier-1ioo
identity: session-reviewer
commit: 3012fdcec368de842e977758a362dbc4a7cd5c5a
created_at: 2026-10-01T22:20:05.900407641Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: rebuild-orchestrator
    commit: 690d175f2f35d2b3de2eb6a19f4630be28f2a4ad
    recorded_at: 2026-10-01T22:20:05.968657353Z
    resolution_commit: 249ce8297f6225fb90cfe1f0c2d6bde4a566c690
    reason: Repaired and independently re-reviewed; final approval must cover the resolution.
    migrated_from: atelier-nmig
---

Historical acceptance lets invalid self-approval or stale independent approval become valid after an unrelated commit.
