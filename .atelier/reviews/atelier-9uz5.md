---
schema: atelier.review
schema_version: 1
id: atelier-9uz5
title: 'crates/atelier-cli/src/main.rs:277-279,356-358: --body help does not match behaviour. (a) create help says ''plain text becomes the Goal'', but for epics and missions it becomes the Outcome. Neither create nor update help mentions that ready epics need Proof and missions need Success criteria (or a strategy), which the old create help (''Outcome/Proof for epics'') at least hinted at. (b) update --body rejects plain text (''Content before first recognized issue body section'') while create --body accepts it, and the update help does not say headings are required. Done-when for this record is ''help text matching behaviour''. Fix: state the plain-text rule per type, name Proof/Success criteria, and say update --body needs ## section headings (or accept plain text the same way create does).'
target: atelier-kaor
identity: Independent reviewer (Claude subagent)
commit: 3f41d55973b5f9469c0ed8066ded2673c0be2259
created_at: 2026-10-15T23:46:43.936357922Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/main.rs:277-279,356-358: --body help does not match behaviour. (a) create help says 'plain text becomes the Goal', but for epics and missions it becomes the Outcome. Neither create nor update help mentions that ready epics need Proof and missions need Success criteria (or a strategy), which the old create help ('Outcome/Proof for epics') at least hinted at. (b) update --body rejects plain text ('Content before first recognized issue body section') while create --body accepts it, and the update help does not say headings are required. Done-when for this record is 'help text matching behaviour'. Fix: state the plain-text rule per type, name Proof/Success criteria, and say update --body needs ## section headings (or accept plain text the same way create does).
