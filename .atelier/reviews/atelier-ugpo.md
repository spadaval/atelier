---
schema: atelier.review
schema_version: 2
id: atelier-ugpo
title: finding on atelier-mtb3 @2580f29e
target: atelier-mtb3
identity: sol-plan-reviewer (gpt-6.1-sol)
commit: 2580f29e70a63df886f976fd663588960eb65b11
created_at: 2026-11-09T06:50:36.647284367Z
event:
  event: finding
  recommendation: fix_now
---

low: The title comparison has no stated oracle. Its first Done when and site-matches-cli require each page to show the title printed by atelier brief -q and atelier debrief -q, but both quiet commands print counts only. I ran them for atelier-yfqu: the outputs begin criteria=1 items=6 and criteria=1 proven=0 respectively, with no mission title; commands/mission_review.rs:65-103,2109-2136 confirms this. Name a separate base-commit source for the title (the mission record or full issue output), retain the quiet commands for counts, and correct the same wording in the mission and strategy criterion so the validator has an executable comparison.
