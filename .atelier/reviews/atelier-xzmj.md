---
schema: atelier.review
schema_version: 1
id: atelier-xzmj
title: 'crates/atelier-app/src/workflow_validation.rs:193-195: the new fn decision_recorded was inserted between work_terminal''s doc comment and work_terminal, so decision_recorded now carries work_terminal''s doc (''Passes when every child (recursively) and, for missions, all work linked by advances...'') followed by its own, and work_terminal (line 220) has no doc comment. Suggested fix: move the two work_terminal doc lines back above fn work_terminal.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:09.177556878Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-app/src/workflow_validation.rs:193-195: the new fn decision_recorded was inserted between work_terminal's doc comment and work_terminal, so decision_recorded now carries work_terminal's doc ('Passes when every child (recursively) and, for missions, all work linked by advances...') followed by its own, and work_terminal (line 220) has no doc comment. Suggested fix: move the two work_terminal doc lines back above fn work_terminal.
