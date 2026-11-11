---
schema: atelier.review
schema_version: 2
id: atelier-a40w
title: finding on atelier-8myy @43c3a5d1
target: atelier-8myy
identity: gpt-6.1-sol
commit: 43c3a5d12f12a4491d62132199fcf516187c90c0
created_at: 2026-11-11T01:41:46.542903335Z
event:
  event: finding
  recommendation: fix_now
---

The second Done when line (.atelier/issues/atelier-8myy.md:35) requires plain list to say the item was dropped and why, and repeats that requirement for done items. This conflicts with redesign-product.md Commands/Anyone: plain list shows open work and --done shows finished items instead. atelier-app/src/work.rs enforces that distinction with e.finished() == filter.done, and view_output.rs list renders status summaries, not drop reasons. Once record precedence is repaired, a correct plain list should succeed and omit the terminal item; it cannot satisfy this checkbox without adding behaviour beyond validator finding 2. Rewrite the proof to require plain list to succeed without displaying the terminal item, list --done to show its terminal status, and status/show/refused acts to give the appropriate terminal explanation. Distinguish done from dropped rather than requiring a drop reason for both. Severity: medium; a worker following the current proof would change the list contract unnecessarily.
