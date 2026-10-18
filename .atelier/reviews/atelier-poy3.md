---
schema: atelier.review
schema_version: 1
id: atelier-poy3
title: 'High: crates/atelier-app/src/mission_review.rs:540-566 and crates/atelier-cli/src/commands/mission_review.rs:1040-1050 Decisions recorded anywhere but a delivered work entry vanish from both forms. The old decision log covered mission_decisions (the mission itself plus every scoped item); now WorkEntry.decisions/malformed are the only outlet, so decision notes on the mission, on in-progress/todo/backlog items (Left, Found) and on dropped items, and malformed notes on any of them (an S4 derived deviation), are not shown, and one-way ones are missing from the sidebar index. Seen live: atelier debrief atelier-r6bt omits all four decisions on atelier-8bil (in Left To Do) while -q still reports decisions=30 against 26 visible. Reproduced in a scratch mission: a one-way note on an in-progress item, a note on an open item and a note on the mission all absent from Markdown and HTML, -q says decisions=3. This also makes the product doc''s ''Decisions appear only where they were made'' untrue for them. Fix: carry decisions and malformed notes on OpenEntry rows (Left/Found/Dropped) and show mission-level notes (e.g. in What We Set Out To Do or as timeline events at their note dates), include them in the one-way index, and add a test with a decision on an open item and on the mission.'
target: atelier-8bil
identity: Independent reviewer (Claude subagent)
commit: 6ee2a98cdea89ed3ac58ef59f139935c48c67fd3
created_at: 2026-10-18T07:00:06.946123503Z
event:
  event: finding
  recommendation: fix_now
---

High: crates/atelier-app/src/mission_review.rs:540-566 and crates/atelier-cli/src/commands/mission_review.rs:1040-1050 Decisions recorded anywhere but a delivered work entry vanish from both forms. The old decision log covered mission_decisions (the mission itself plus every scoped item); now WorkEntry.decisions/malformed are the only outlet, so decision notes on the mission, on in-progress/todo/backlog items (Left, Found) and on dropped items, and malformed notes on any of them (an S4 derived deviation), are not shown, and one-way ones are missing from the sidebar index. Seen live: atelier debrief atelier-r6bt omits all four decisions on atelier-8bil (in Left To Do) while -q still reports decisions=30 against 26 visible. Reproduced in a scratch mission: a one-way note on an in-progress item, a note on an open item and a note on the mission all absent from Markdown and HTML, -q says decisions=3. This also makes the product doc's 'Decisions appear only where they were made' untrue for them. Fix: carry decisions and malformed notes on OpenEntry rows (Left/Found/Dropped) and show mission-level notes (e.g. in What We Set Out To Do or as timeline events at their note dates), include them in the one-way index, and add a test with a decision on an open item and on the mission.
