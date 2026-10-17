---
schema: atelier.review
schema_version: 1
id: atelier-4675
title: 'crates/atelier-cli/src/commands/git.rs:500 — status''s Other Local Work Branches rows always add a ''| N active'' cue, including ''| 0 active'' (seen live: task/atelier-7w1g ... | not merged into master | 0 active). The new Counts rule says a list of counts leaves out its zeros. This predates the change and is a row cue, not a summary line, so it does not block. Follow-up: drop the cue at zero, or document why it stays.'
target: atelier-krf4
identity: Independent reviewer (Claude subagent)
commit: 23527f8026ade3ebd70c6740ddb995c23d25fe87
created_at: 2026-10-17T07:38:21.732008126Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-cli/src/commands/git.rs:500 — status's Other Local Work Branches rows always add a '| N active' cue, including '| 0 active' (seen live: task/atelier-7w1g ... | not merged into master | 0 active). The new Counts rule says a list of counts leaves out its zeros. This predates the change and is a row cue, not a summary line, so it does not block. Follow-up: drop the cue at zero, or document why it stays.
