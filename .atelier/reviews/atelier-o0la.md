---
schema: atelier.review
schema_version: 2
id: atelier-o0la
title: finding on atelier-5fsj @5ff588fb
target: atelier-5fsj
identity: gpt-6.1-sol-5fsj-r5
commit: 5ff588fb22b665eab363f80c3a085b948b8238ac
created_at: 2026-11-10T16:26:26.269335937Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/scorecard/commands.py:114 R12. The substitution boundary scanner counts parentheses inside quotes as shell delimiters. On the synthetic supported command out=$(printf ')'; atelier status), atelier_calls returns [] and thread_stats reports process=0 and atelier=0, although the substitution executes atelier status (expected one of each). The README explicitly recognises quoted punctuation and $(...) outside single quotes; this is within the closed contract. Scan the substitution boundary with quote and escape context, and add a regression asserting the process and confusion counts.
