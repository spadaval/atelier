---
schema: atelier.review
schema_version: 1
id: atelier-esix
title: 'crates/atelier-cli/src/commands/workflow.rs:245-249: the parked-parent check now runs only when the child''s own status is todo-category. A blocked child is never checked, and the work workflow''s start goes from [todo, blocked] to in_progress, so start is easy to get around. Repro in /tmp/r28c: parent in backlog, child created and moved straight to ''block'' (never started). ''atelier start <child>'' succeeds: it creates task/<parent>, records the branch and moves the child to in_progress, all while the parent is still in backlog. The rule says start refuses children of a parked parent until the parent is planned. Fix: in the transition check, skip only when the child''s source status is active-category (so in-flight request_review/revise keep working) and still refuse todo or blocked into active. Classification can keep a blocked child as Blocked so it stays visible. Add a test: a blocked child under a backlog parent cannot start.'
target: atelier-28mz
identity: Independent reviewer (Claude subagent)
commit: 7041b06b4cffd9c0ddd71ac61ff4856279f973d1
created_at: 2026-10-16T01:30:34.103135139Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/workflow.rs:245-249: the parked-parent check now runs only when the child's own status is todo-category. A blocked child is never checked, and the work workflow's start goes from [todo, blocked] to in_progress, so start is easy to get around. Repro in /tmp/r28c: parent in backlog, child created and moved straight to 'block' (never started). 'atelier start <child>' succeeds: it creates task/<parent>, records the branch and moves the child to in_progress, all while the parent is still in backlog. The rule says start refuses children of a parked parent until the parent is planned. Fix: in the transition check, skip only when the child's source status is active-category (so in-flight request_review/revise keep working) and still refuse todo or blocked into active. Classification can keep a blocked child as Blocked so it stays visible. Add a test: a blocked child under a backlog parent cannot start.
