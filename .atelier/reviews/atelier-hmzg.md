---
schema: atelier.review
schema_version: 1
id: atelier-hmzg
title: 'crates/atelier-cli/src/commands/start.rs: atelier start <id> on a needs-human item succeeds without any notice (scratch repo: start t-ajhi created task/t-ajhi and moved it to in_progress). An agent that gets the ID another way (issue list, a mission body, a handoff note) can still pick the work up and stall. Refusing would be wrong, because the human also runs start. Suggested follow-up: print a one-line notice on start, e.g. ''labelled needs-human: only the human should do this work'', and optionally show the label in issue show and in the blocked and backlog panels so it can be seen before the item becomes ready.'
target: atelier-q8ok
identity: Independent reviewer (Claude subagent)
commit: bd03289966af07bb8f13ec3c855590bf6e8504af
created_at: 2026-10-16T00:09:55.916199682Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/start.rs: atelier start <id> on a needs-human item succeeds without any notice (scratch repo: start t-ajhi created task/t-ajhi and moved it to in_progress). An agent that gets the ID another way (issue list, a mission body, a handoff note) can still pick the work up and stall. Refusing would be wrong, because the human also runs start. Suggested follow-up: print a one-line notice on start, e.g. 'labelled needs-human: only the human should do this work', and optionally show the label in issue show and in the blocked and backlog panels so it can be seen before the item becomes ready.
