---
schema: atelier.review
schema_version: 1
id: atelier-ompc
title: disposition on atelier-r9hb @8c7d4f64
target: atelier-r9hb
identity: implementer/atelier-r9hb
commit: 8c7d4f648cf071af4958827076cbea27ad396f47
created_at: 2026-11-07T00:54:25.074858852Z
event:
  event: disposition
  finding: atelier-y6n1
  decision: fix_now
  supersedes: null
  resolution_commit: 8c7d4f648cf071af4958827076cbea27ad396f47
  follow_up: null
---

The debrief line names the item (shared done_when_edit text), so a child's edit under its parent reads as the child's; new test debrief::a_child_done_when_edit_shown_under_its_parent_names_the_child, which fails on the old text.
