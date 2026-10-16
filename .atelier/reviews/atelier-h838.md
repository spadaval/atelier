---
schema: atelier.review
schema_version: 1
id: atelier-h838
title: 'crates/atelier-app/src/work_classification.rs:178-205: a todo decision with an open native blocker falls into the ''Some(todo)'' arm with can_enter_active=false (no start) and becomes Backlog, and open_decision requires no blockers, so it appears in neither `work human` nor `work blocked` (reproduced: both list ''(none)''/omit it; status shows ''Needs a human: 0''). A blocked design question is invisible to the human. Also open_decision overrides the terminal_ancestor Backlog rule, so a decision under a superseded parent still shows as needs-human, contrary to the documented classify_issue contract. Suggested fix: for decisions, classify open blockers as Blocked and keep the terminal-ancestor check ahead of the decision override; add tests.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:08.871898374Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/work_classification.rs:178-205: a todo decision with an open native blocker falls into the 'Some(todo)' arm with can_enter_active=false (no start) and becomes Backlog, and open_decision requires no blockers, so it appears in neither `work human` nor `work blocked` (reproduced: both list '(none)'/omit it; status shows 'Needs a human: 0'). A blocked design question is invisible to the human. Also open_decision overrides the terminal_ancestor Backlog rule, so a decision under a superseded parent still shows as needs-human, contrary to the documented classify_issue contract. Suggested fix: for decisions, classify open blockers as Blocked and keep the terminal-ancestor check ahead of the decision override; add tests.
