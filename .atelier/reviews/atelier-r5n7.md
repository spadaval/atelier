---
schema: atelier.review
schema_version: 1
id: atelier-r5n7
title: 'crates/atelier-cli/src/commands/issue.rs:942 — create_next_step offers ''Start this work: atelier start <id>'' for every work type whatever the initial status, so ''issue create --status backlog'' (how follow-ups such as atelier-6wyv/ytxg/0sia are filed) offers start for a status with no start transition. Reproduced in a fixture: following it creates task/<id>, writes workflow_branch on the record and leaves it backlog (''Status: backlog (no start transition from this status)''). This regresses the old create footer (transition options were right for backlog) and contradicts the rule this change adds to human-cli-output.md, ''A view offers only the steps the record''s state allows''. Fix: offer start only when the type''s workflow has a start transition from the created status, else the transition options (e.g. plan); add a backlog case to test_issue_create_footer_offers_start_for_work_and_transitions_for_missions.'
target: atelier-ksx6
identity: Independent reviewer (Claude subagent)
commit: b14cd5e157c4e2392e7ded21a46961d63da7e3a8
created_at: 2026-10-17T08:27:59.193347889Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/issue.rs:942 — create_next_step offers 'Start this work: atelier start <id>' for every work type whatever the initial status, so 'issue create --status backlog' (how follow-ups such as atelier-6wyv/ytxg/0sia are filed) offers start for a status with no start transition. Reproduced in a fixture: following it creates task/<id>, writes workflow_branch on the record and leaves it backlog ('Status: backlog (no start transition from this status)'). This regresses the old create footer (transition options were right for backlog) and contradicts the rule this change adds to human-cli-output.md, 'A view offers only the steps the record's state allows'. Fix: offer start only when the type's workflow has a start transition from the created status, else the transition options (e.g. plan); add a backlog case to test_issue_create_footer_offers_start_for_work_and_transitions_for_missions.
