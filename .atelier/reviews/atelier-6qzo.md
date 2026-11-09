---
schema: atelier.review
schema_version: 3
id: atelier-6qzo
title: finding on branch release-0.4.0 @6dd196be
branch: release-0.4.0
identity: gpt-6.1-sol
commit: 6dd196be861e871264a466b5892360a9f8218715
created_at: 2026-11-09T20:56:07.097918530Z
event:
  event: finding
  recommendation: fix_now
---

medium: CHANGELOG.md:11 The blanket promise that 0.3.x repositories work unmigrated is false, and Removed omits a breaking change. atelier-2bjz/atelier-w5hx removed issue note --one-way and support for existing [decision one-way] notes. The flag now exits 2; DecisionNote::parse rejects that tag and lint reports it, although v0.3.1 could write it (including atelier-fip8). Only this repository had its four notes rewritten. Add the removed flag and note format, explain converting existing tags to [decision] and using decision records for costly choices, and qualify the compatibility promise. This is a release-note fix, not a request to restore the feature.
