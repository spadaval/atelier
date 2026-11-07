---
schema: atelier.review
schema_version: 2
id: atelier-1ia7
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:41-43: ''prime ... is removed once adopters have re-run atelier setup claude'' has no observable trigger (Atelier cannot see adopters'' settings) and no follow-up issue tracks the removal, so the alias can persist indefinitely. Fix: file a backlog removal issue, or tie removal to a release.'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:54.584666565Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: a485ef023556583b5731cad57ec897adc820948c
    recorded_at: 2026-10-16T23:44:13.267494929Z
    resolution_commit: a485ef023556583b5731cad57ec897adc820948c
    reason: One-release alias, removal tracked by atelier-jjvc (blocked by sdb0)
    migrated_from: atelier-g6fd
---

docs/adr/0022-status-orients-and-output-is-dense.md:41-43: 'prime ... is removed once adopters have re-run atelier setup claude' has no observable trigger (Atelier cannot see adopters' settings) and no follow-up issue tracks the removal, so the alias can persist indefinitely. Fix: file a backlog removal issue, or tie removal to a release.
