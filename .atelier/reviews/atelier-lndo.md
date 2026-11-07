---
schema: atelier.review
schema_version: 2
id: atelier-lndo
title: finding on atelier-vo0z @3c486736
target: atelier-vo0z
identity: claude-reviewer-subagent
commit: 3c4867369b9a6f0180d6b95f7b0b573075d5766b
created_at: 2026-11-04T21:50:04.605262428Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author
    commit: a523e50c56f1812587008cdf9bf0960bcd2fcd16
    recorded_at: 2026-11-04T21:50:34.532266804Z
    resolution_commit: a523e50c56f1812587008cdf9bf0960bcd2fcd16
    reason: README no longer states a glibc floor; musl covers older glibc
    migrated_from: atelier-qncy
---

minor: README.md:8 still says Linux install needs 'glibc 2.35 or newer'. With the musl archives the installer falls back to a static build on older glibc (checked: on simulated glibc 2.34 it picks the musl archive), so the README now turns away the RHEL 9 users this fix is for. Suggested fix: say any glibc works, with a static musl build used below 2.35.
