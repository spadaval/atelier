---
schema: atelier.review
schema_version: 2
id: atelier-49wf
title: finding on atelier-1sbw @bf9a2e65
target: atelier-1sbw
identity: gpt-6.1-sol-r3
commit: bf9a2e656ca3994af04fa9b0aa6059c043e0c084
created_at: 2026-11-10T23:26:07.411554296Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/cutover/convert_records.py:367 R1 Links from moved strategies to open records retain the deleted issues/ destinations. In the invented fixture, add [the item](../../.atelier/issues/demo-a1ta.md) to the strategy Why paragraph and commit the old tree. The target exists before conversion. Conversion writes [the item](../../issues/demo-a1ta.md) into .atelier/missions/demo-m1ss/mission.md, but deletes .atelier/issues/ and writes the target to .atelier/items/demo-a1ta/item.md instead. The link is broken even though conversion and --verify exit 0. Links only maps moved strategies; its tracked-file fallback treats issue files as staying in place. Map the destinations of open issue records as well, including missions, before rebasing relative links. Add regressions resolving links from moved strategies to both converted items and missions. This violates the requirement that moving a strategy break no visible link.
