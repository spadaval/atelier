---
schema: atelier.review
schema_version: 2
id: atelier-4ril
title: finding on atelier-1sbw @5f7f51bc
target: atelier-1sbw
identity: gpt-6.1-sol-r4
commit: 5f7f51bc93d0984818efab6ed8ce9924a1a46c87
created_at: 2026-11-10T23:35:49.377939223Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:370 R2 URI-encoded relative destinations are compared directly with Git file names, so valid links are silently left broken after the strategy moves. In the invented fixture track docs/strategies/mocks/a v2.md and use [mocks](mocks/a%20v2.md) in Shape. The old link resolves after URL decoding; the lookup at lines 370-376 finds no literal a%20v2.md and leaves it unchanged in the new design.md, where it points to a missing file. Conversion and --verify both exit 0. Decode the destination for filesystem matching while preserving valid URL encoding in the rewritten link, and resolve an encoded-space regression independently of LINK.
