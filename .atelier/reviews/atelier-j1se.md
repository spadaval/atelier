---
schema: atelier.review
schema_version: 3
id: atelier-j1se
title: finding on branch release-0.4.0 @6dd196be
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 6dd196be861e871264a466b5892360a9f8218715
created_at: 2026-11-09T20:56:32.226765288Z
event:
  event: finding
  recommendation: fix_now
---

low: CHANGELOG.md:27 The migration command shown does not turn any lines into loose ends: atelier migrate after-merge is a dry run unless --apply is given, as its current help states. Existing 0.3.x policies also need to declare loose_end first or it refuses with migrate_no_loose_end (documented by atelier-l457 and ADR 0028). Show atelier migrate after-merge --apply for the write, or explicitly distinguish preview from apply, and mention the policy prerequisite or link the existing Moving an existing policy instructions. Otherwise the upgrade guidance leaves users with no migrated records or an unexplained refusal.
