---
schema: atelier.review
schema_version: 1
id: atelier-ufqs
title: This repository's own .claude/settings.json runs 'bd prime' (beads) on SessionStart and PreCompact, unchanged since 'bd init' (b824aac4); no atelier hook is installed, and CLAUDE.md carries a bd-doctor marker. Decision 1's hook change therefore does not reach this repo's own Claude sessions, and the dogfooding that sdb0 or ADR 0022's 'raises its bar' consequence implies cannot happen here until setup claude is run. Pre-existing, so not caused by this change; file a follow-up to replace the beads hook.
target: atelier-bw0b
identity: Independent reviewer (Claude subagent)
commit: 484e6d6fd514738417d0297e2bc5dd376b181e86
created_at: 2026-10-16T23:21:54.455836028Z
event:
  event: finding
  recommendation: defer
---

This repository's own .claude/settings.json runs 'bd prime' (beads) on SessionStart and PreCompact, unchanged since 'bd init' (b824aac4); no atelier hook is installed, and CLAUDE.md carries a bd-doctor marker. Decision 1's hook change therefore does not reach this repo's own Claude sessions, and the dogfooding that sdb0 or ADR 0022's 'raises its bar' consequence implies cannot happen here until setup claude is run. Pre-existing, so not caused by this change; file a follow-up to replace the beads hook.
