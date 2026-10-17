---
schema: atelier.review
schema_version: 1
id: atelier-ust0
title: 'crates/atelier-cli/src/commands/mission_review.rs:1025: the HTML Risks fallback says ''The strategy names none.'' even when the mission has no strategy at all (scratch mission without a strategy showed it), and the HTML always renders an empty Success Criteria table when there are no criteria and no problems, where the Markdown omits the section. Misleading, not wrong on any recorded fact. Fix: say ''No strategy linked.'' when view.strategy is None, and skip or annotate the empty criteria table.'
target: atelier-acrt
identity: Independent reviewer (Claude subagent)
commit: 5c858f437d2b7227bde75d9eaa3c650240b510b7
created_at: 2026-10-17T19:52:25.270179832Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/mission_review.rs:1025: the HTML Risks fallback says 'The strategy names none.' even when the mission has no strategy at all (scratch mission without a strategy showed it), and the HTML always renders an empty Success Criteria table when there are no criteria and no problems, where the Markdown omits the section. Misleading, not wrong on any recorded fact. Fix: say 'No strategy linked.' when view.strategy is None, and skip or annotate the empty criteria table.
