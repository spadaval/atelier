---
schema: atelier.review
schema_version: 2
id: atelier-ust0
title: 'crates/atelier-cli/src/commands/mission_review.rs:1025: the HTML Risks fallback says ''The strategy names none.'' even when the mission has no strategy at all (scratch mission without a strategy showed it), and the HTML always renders an empty Success Criteria table when there are no criteria and no problems, where the Markdown omits the section. Misleading, not wrong on any recorded fact. Fix: say ''No strategy linked.'' when view.strategy is None, and skip or annotate the empty criteria table.'
target: atelier-acrt
identity: Independent reviewer (Claude subagent)
commit: 5c858f437d2b7227bde75d9eaa3c650240b510b7
created_at: 2026-10-17T19:52:25.270179832Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-implementer
    commit: 8b0890bf8b0afcece4d577e89e3cd294eef81618
    recorded_at: 2026-10-17T19:55:31.452171435Z
    resolution_commit: 8b0890bf8b0afcece4d577e89e3cd294eef81618
    reason: Fixed in 8b0890bf8b0afcece4d577e89e3cd294eef81618; see the commit message
    migrated_from: atelier-5mks
---

crates/atelier-cli/src/commands/mission_review.rs:1025: the HTML Risks fallback says 'The strategy names none.' even when the mission has no strategy at all (scratch mission without a strategy showed it), and the HTML always renders an empty Success Criteria table when there are no criteria and no problems, where the Markdown omits the section. Misleading, not wrong on any recorded fact. Fix: say 'No strategy linked.' when view.strategy is None, and skip or annotate the empty criteria table.
