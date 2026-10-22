---
schema: atelier.review
schema_version: 1
id: atelier-465o
title: 'low: crates/atelier-cli/src/commands/git.rs:483-484 the doc comment says readiness facts are ''appended'', but the title is the last, free-text column, so facts appended after it are hard to separate (titles may contain anything), and atelier-tciw''s examples (''astromech-cmhv  approved  behind 3  merges cleanly'') show a different, title-less layout. The row can take tciw''s facts without a rewrite if they go into the state column before the title, as ftqn''s own ''✓ approved'' example does (''done, not merged ✓ approved  Title''). Suggested: fix the comment to say facts join the state column before the title, and note the placement on atelier-tciw.'
target: atelier-9g48
identity: Independent reviewer (Claude subagent)
commit: 57aaf3090f1be8100a33932f28e2c6aea6376cf3
created_at: 2026-10-22T20:43:04.778910968Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/git.rs:483-484 the doc comment says readiness facts are 'appended', but the title is the last, free-text column, so facts appended after it are hard to separate (titles may contain anything), and atelier-tciw's examples ('astromech-cmhv  approved  behind 3  merges cleanly') show a different, title-less layout. The row can take tciw's facts without a rewrite if they go into the state column before the title, as ftqn's own '✓ approved' example does ('done, not merged ✓ approved  Title'). Suggested: fix the comment to say facts join the state column before the title, and note the placement on atelier-tciw.
