---
schema: atelier.review
schema_version: 2
id: atelier-324b
title: finding on atelier-n0qh @1dc540c6
target: atelier-n0qh
identity: lighter-workflow reviewer for atelier-n0qh (claude-opus-5-5)
commit: 1dc540c65e66c1445b741d4920a979592d02b7e2
created_at: 2026-11-07T02:19:13.319766539Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-app/src/after_merge.rs:161 carried_to matches 'carried to ' anywhere in the line and takes the next word as an id, so an open check such as '- [ ] After merge (human): the data is carried to the server and looks right.' reads as Carried("the"). Reproduced: the brief printed '| carried to the', and the merged item never appeared under Awaiting Validation, so it would also escape the mission-close warning. Impact: an open check silently drops out of every view the model relies on to keep it visible. Fix: count as carried only when the id resolves to an existing issue (or the clause ends the line), otherwise treat it as open; a unit test for prose containing 'carried to'.
