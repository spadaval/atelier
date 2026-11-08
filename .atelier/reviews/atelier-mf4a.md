---
schema: atelier.review
schema_version: 2
id: atelier-mf4a
title: finding on atelier-4k28 @86e0d0de
target: atelier-4k28
identity: grok-reviewer (grok-4.7)
commit: 86e0d0de33f9aaaf64414420ac1e57465de1383b
created_at: 2026-11-08T06:29:11.647414695Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/smoke-repos.sh:104 clones with git clone --mirror, and a local path hardlinks .git/objects. On a scratch repository every object inode was shared during the clone (nlink 1 to 2), and the pack ctime stayed changed after the script removed its scratch directory (1794119288 to 1794119289). The snapshot at scripts/smoke-repos.sh:112 hashes refs, HEAD and status, so it still reports the source unchanged. That breaks the Done when rule that the script never writes to the repositories it clones, and git clone --local races with concurrent commits in those repositories. Clone with --no-hardlinks: the same pack's ctime did not move when that flag was used.
