---
schema: atelier.review
schema_version: 2
id: atelier-2gp6
title: finding on atelier-1sbw @f317ac78
target: atelier-1sbw
identity: gpt-6.1-sol
commit: f317ac78fdffcef88ab75a2773d9ee5f59688361
created_at: 2026-11-10T21:55:10.185552889Z
event:
  event: finding
  recommendation: fix_now
---

R1 .atelier/issues/atelier-1sbw.md:36: high. Mission conversion has no destination for the old mission record body when a strategy exists. It selects five sections from the strategy and moves the rest of that strategy to design.md, but neither rule preserves the mission record Background or Notes. At f317ac78, atelier-295u, atelier-fcb7 and atelier-g6rp all have both a strategy and Notes. For example g6rp records the human decision to keep file-pr separate and the unfinished push/closure boundary; those are not in its strategy. The converter can satisfy every written mapping rule and silently remove this context from the live tree. Define a deterministic destination in the permitted mission documents for record-only text, including Notes and decisions, and test a mission whose record adds information not present in its strategy. Git history is not a substitute for preserving current open-mission context.
