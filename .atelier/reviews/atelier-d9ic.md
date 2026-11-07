---
schema: atelier.review
schema_version: 2
id: atelier-d9ic
title: 'AGENTS.md:23-25 (''After installing the accepted rebuild, atelier setup claude configures hooks for the installed CLI'') and AGENTS.md:46 (''Installed CLI: atelier (install the rebuild only after acceptance)'') still treat rebuild acceptance as pending, though the edit at :6-13 says mission atelier-kgcr closed 2026-10-15 and the installed ~/.cargo/bin/atelier is 0.2.0+8e3b6c67. The issue''s Done when is that AGENTS.md no longer names the finished rebuild as current; these lines still do. Fix in this branch: drop the acceptance conditions (installed atelier is the rebuild; keep target/debug for development).'
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:53.676902404Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Claude Opus 5.5 (author)
    commit: a485ef023556583b5731cad57ec897adc820948c
    recorded_at: 2026-10-16T23:44:11.343956381Z
    resolution_commit: a485ef023556583b5731cad57ec897adc820948c
    reason: AGENTS.md install and hook lines updated for the installed CLI
    migrated_from: atelier-cphk
---

AGENTS.md:23-25 ('After installing the accepted rebuild, atelier setup claude configures hooks for the installed CLI') and AGENTS.md:46 ('Installed CLI: atelier (install the rebuild only after acceptance)') still treat rebuild acceptance as pending, though the edit at :6-13 says mission atelier-kgcr closed 2026-10-15 and the installed ~/.cargo/bin/atelier is 0.2.0+8e3b6c67. The issue's Done when is that AGENTS.md no longer names the finished rebuild as current; these lines still do. Fix in this branch: drop the acceptance conditions (installed atelier is the rebuild; keep target/debug for development).
