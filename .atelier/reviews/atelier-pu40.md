---
schema: atelier.review
schema_version: 2
id: atelier-pu40
title: finding on atelier-5fsj @f35e6e42
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r1
commit: f35e6e4206d454b2355d188a33aa4e7689f5c005
created_at: 2026-11-10T14:56:54.837008491Z
event:
  event: finding
  recommendation: fix_now
---

high: scripts/scorecard/counted.py:51 R1. Calls before first edit only recognizes T3 file_change events, so shell writes do not stop the counter and shell-only implementers are dropped. A synthetic implementation thread whose first call is sed -i on src/lib.rs, followed by cargo test and a file_change on another source file, reports 2 calls instead of 0; without that later file_change it is excluded entirely. The baseline contains leading product-file sed -i commands before the first recognized edit in four atelier implementer threads, two with no file_change at all. The strategy requires calls before the first write outside .atelier, and the item says to escalate rather than change an uncomputable definition. Recognize the shell writes from the frozen inputs, add arithmetic tests for shell-first and shell-only implementations, and regenerate the baseline. If first writes cannot be recovered faithfully, escalate the proposed proxy before publishing it as this measure.
