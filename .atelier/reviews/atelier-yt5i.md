---
schema: atelier.review
schema_version: 2
id: atelier-yt5i
title: git.rs:504-510,558-566 hard-code ✓ for merged/done on other-branch lines and ↳ marks; a blocked owner on another branch gets no ⚠ while the status Owner row prints [⚠ blocked]; a second copy of the symbol rule outside status_cue.
target: atelier-deyy
identity: Independent reviewer (Claude subagent)
commit: 0ff857361716902c8e3d52b24d7e21409ce09eaf
created_at: 2026-10-17T03:52:31.237920898Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-1w8c)
    commit: e9dff855977c8d8f636f41794cfd43176183a2b9
    recorded_at: 2026-10-17T03:55:41.769390469Z
    resolution_commit: 47928f72b37b99a6f33d9ac1deffba9eec71b42b
    reason: Other-branch lines and in-flight marks use status_cue with the branch policy's category; blocked-owner test added.
    migrated_from: atelier-ddmh
---

git.rs:504-510,558-566 hard-code ✓ for merged/done on other-branch lines and ↳ marks; a blocked owner on another branch gets no ⚠ while the status Owner row prints [⚠ blocked]; a second copy of the symbol rule outside status_cue.
