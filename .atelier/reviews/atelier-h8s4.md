---
schema: atelier.review
schema_version: 2
id: atelier-h8s4
title: finding on atelier-orb2 @ea6ff0ee
target: atelier-orb2
identity: GPT-6.1 Sol independent planning reviewer
commit: ea6ff0ee0165cd82b315020fb7d5be1b3816f070
created_at: 2026-11-11T02:50:57.471423779Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-orb2.md:39 (also :41-42), severity medium: The carried-procedure requirements cannot all hold. Line 39 requires keeping a line when its command or record kind has no replacement, but lines 41-42 forbid every removed command/path and require every named command to exist. This is present in the live tree: garden.md:44-59 and groom.md:53-79 create decision-kind digests and finish them with decide; audit.md:25-26 requires the removed decision-record test. The new CLI has no issue types or decision-record completion command; decide answers an escalated question on a started item (redesign-product.md, Commands and Review; next/crates/atelier-cli/src/main.rs). Merely listing the gaps in the Report cannot make those Done when lines true. Before starting, settle the minimal command/record mapping for these carried instructions and reconcile the no-replacement rule with the ban/help checks; escalate any mapping that would require the mission-4 procedure rewrite instead of leaving it to the implementer.
