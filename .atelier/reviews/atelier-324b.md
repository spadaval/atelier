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
  disposition:
    decision: fix_now
    identity: atelier-n0qh implementer (claude-opus-5-5)
    commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    recorded_at: 2026-11-07T02:29:07.231024018Z
    resolution_commit: 44243f7d20d2a36af8deecc8ca198fbcd9979403
    reason: carried to counts only when the id is an existing issue; unit test carried_to_counts_only_when_it_names_an_existing_issue, and the scenario's 'Carried to the phone.' step still warns
---

low: crates/atelier-app/src/after_merge.rs:161 carried_to matches 'carried to ' anywhere in the line and takes the next word as an id, so an open check such as '- [ ] After merge (human): the data is carried to the server and looks right.' reads as Carried("the"). Reproduced: the brief printed '| carried to the', and the merged item never appeared under Awaiting Validation, so it would also escape the mission-close warning. Impact: an open check silently drops out of every view the model relies on to keep it visible. Fix: count as carried only when the id resolves to an existing issue (or the clause ends the line), otherwise treat it as open; a unit test for prose containing 'carried to'.
