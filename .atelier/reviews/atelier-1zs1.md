---
schema: atelier.review
schema_version: 1
id: atelier-1zs1
title: 'crates/atelier-cli/src/commands/mission_review.rs:119-120 and :156: proof age is worded ''<n> code commits before close'' and the deviation ''after it before close'' even when the mission is open and the count runs to HEAD (the documented behaviour). ''atelier debrief atelier-r6bt'' (status todo, no close) prints ''S1 ✓ proven ... | 3 code commits before close'' and five ''aged proof ... before close'' lines. A reader is told about a close that has not happened, and the bound is wrong. Fix: say ''before HEAD'' (or ''since'') when timeline.closed is None.'
target: atelier-rx22
identity: Independent reviewer (Claude subagent)
commit: 7ad42cef3afe76518fe905a2633a939ceaa108f4
created_at: 2026-10-17T19:32:16.989148812Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/mission_review.rs:119-120 and :156: proof age is worded '<n> code commits before close' and the deviation 'after it before close' even when the mission is open and the count runs to HEAD (the documented behaviour). 'atelier debrief atelier-r6bt' (status todo, no close) prints 'S1 ✓ proven ... | 3 code commits before close' and five 'aged proof ... before close' lines. A reader is told about a close that has not happened, and the bound is wrong. Fix: say 'before HEAD' (or 'since') when timeline.closed is None.
