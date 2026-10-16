---
schema: atelier.review
schema_version: 1
id: atelier-q947
title: 'docs/adr/0022-status-orients-and-output-is-dense.md:48-67 does not address the same-day draft strategy docs/strategies/review-and-introspection.md (mission atelier-r6bt), which conflicts with it twice. (a) Strategy S6 (:59) says ''prime and issue show surface the current decisions'', and its Scope (:101) lists prime, while Decision 1 retires prime. (b) The strategy''s human-directed revision (8403ef43) says brief/debrief are ''Rich on a terminal, Markdown everywhere else'' (color, width-aware layout, links), while Decision 3 says ''Output is dense Markdown for every reader''. Failure: whoever picks up r6bt S6 or atelier-deyy (which writes Decision 3 into human-cli-output.md) gets two current, contradictory directions and has to pick one. Fix: one sentence in Decision 3 saying whether terminal styling on a TTY is compatible (the human''s call), and retarget S6/Scope to status (in the strategy, or add it to atelier-sdb0''s reference list).'
target: atelier-bw0b
identity: Independent reviewer 2 (Claude subagent)
commit: 0db4b1845dab4da518fe334854b54f50a9a95345
created_at: 2026-10-16T23:50:25.235999562Z
event:
  event: finding
  recommendation: fix_now
---

docs/adr/0022-status-orients-and-output-is-dense.md:48-67 does not address the same-day draft strategy docs/strategies/review-and-introspection.md (mission atelier-r6bt), which conflicts with it twice. (a) Strategy S6 (:59) says 'prime and issue show surface the current decisions', and its Scope (:101) lists prime, while Decision 1 retires prime. (b) The strategy's human-directed revision (8403ef43) says brief/debrief are 'Rich on a terminal, Markdown everywhere else' (color, width-aware layout, links), while Decision 3 says 'Output is dense Markdown for every reader'. Failure: whoever picks up r6bt S6 or atelier-deyy (which writes Decision 3 into human-cli-output.md) gets two current, contradictory directions and has to pick one. Fix: one sentence in Decision 3 saying whether terminal styling on a TTY is compatible (the human's call), and retarget S6/Scope to status (in the strategy, or add it to atelier-sdb0's reference list).
