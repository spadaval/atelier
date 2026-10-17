---
schema: atelier.review
schema_version: 1
id: atelier-9839
title: 'Low, display only. crates/atelier-app/src/issue_read.rs:171-193 (objective Relationships line in issue show for missions/parents). (1) ''other'' still depends on storage: get_typed_relations is now deduplicated but list_record_links still yields one link per stored copy (and counts advances links), so one logical context link counts 3 if stored two-sided and 2 if one-sided; on real records atelier-o3lm goes 32->31 and atelier-r6bt 10->9 versus master for their two-sided ''related'' links. (2) ''open blockers'' now chains the objective''s own id, so a mission''s own blockers stored in the blocker''s blocks are newly counted (master counted only relates blocked_by). No gate reads these numbers. Suggest folding into atelier-x3ru / the human_output rework (count logical links once, decide whether own blockers belong here) rather than touching output now.'
target: atelier-ihss
identity: Independent reviewer (Claude subagent)
commit: af8645378791a3ff66fe5d8ab2a5db7d9cd010bd
created_at: 2026-10-17T06:27:54.435087854Z
event:
  event: finding
  recommendation: defer
---

Low, display only. crates/atelier-app/src/issue_read.rs:171-193 (objective Relationships line in issue show for missions/parents). (1) 'other' still depends on storage: get_typed_relations is now deduplicated but list_record_links still yields one link per stored copy (and counts advances links), so one logical context link counts 3 if stored two-sided and 2 if one-sided; on real records atelier-o3lm goes 32->31 and atelier-r6bt 10->9 versus master for their two-sided 'related' links. (2) 'open blockers' now chains the objective's own id, so a mission's own blockers stored in the blocker's blocks are newly counted (master counted only relates blocked_by). No gate reads these numbers. Suggest folding into atelier-x3ru / the human_output rework (count logical links once, decide whether own blockers belong here) rather than touching output now.
