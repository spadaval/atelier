---
schema: atelier.review
schema_version: 2
id: atelier-mp3l
title: finding on atelier-stnv @e9d6bd3d
target: atelier-stnv
identity: grok-plan-rereviewer (grok-4.7)
commit: e9d6bd3d2535398cf53af3fafc98e5fbb746edf5
created_at: 2026-11-08T06:00:27.290364459Z
event:
  event: finding
  recommendation: fix_now
---

high: "Treat declined like every superseded special case" contradicts "show declined as finished". The Done when line says everywhere the code special-cases superseded today, including land's base completion claim (crates/atelier-cli/src/commands/git.rs base_done_claims) and the blocker checks (crates/atelier-app/src/review_gate.rs), declined is treated the same, and the next sentence says debrief shows declined work as finished, labelled declined. Debrief counts a category-done item as finished only when its status is not superseded (crates/atelier-app/src/mission_review.rs done, line 344), lists a superseded item as dropped (lines 648 and 660), and leaves it out of the planned count (line 711). The debrief renderer draws superseded as retired, not finished (crates/atelier-cli/src/commands/mission_review.rs status_tone and status_class). Copying those checks puts a declined item in Dropped. decisions.rs decided() also special-cases status superseded, and that means a decision was replaced, not that work was declined. The land skip and the completion blocker skip, including the parent filter at review_gate.rs line 1321, do have to treat declined like superseded, or a base decline is refused and an open blocker holds it. Name those gate sites. Say debrief, doctor and work views show a declined item as finished under its own status, not as dropped.
