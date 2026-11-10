---
schema: atelier.review
schema_version: 2
id: atelier-rc3i
title: finding on atelier-1sbw @5f7f51bc
target: atelier-1sbw
identity: gpt-6.1-sol-r4
commit: 5f7f51bc93d0984818efab6ed8ce9924a1a46c87
created_at: 2026-11-10T23:35:49.822598227Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:511 R3 Reference labels are matched as literal substrings instead of using Markdown whitespace normalization, so splitting a strategy still loses reference definitions. In the invented fixture replace both [the mock][m] uses with [the mock][two  words] and the definition with [two words]: mocks/a.md#a-mock. CommonMark resolves both uses before conversion. The definition stays in design.md, but missing() does not recognize the equivalent label in Why and does not copy it into mission.md. That file retains the reference text without a definition, so it renders as text rather than a link. Conversion and --verify exit 0. Normalize label whitespace consistently when matching uses and definitions and add a split-document regression.
