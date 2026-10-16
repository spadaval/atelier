---
schema: atelier.review
schema_version: 1
id: atelier-hep8
title: 'crates/atelier-cli/src/commands/start.rs:95 (no decision guard): `atelier start <decision>` on a standalone decision creates and switches to branch decision/<id> and records workflow_branch on the record, printing only ''todo (no start transition from this status)''. Reproduced in a scratch repo: clean main, file decision t-es5e, `atelier start t-es5e` -> ''git switch -c decision/t-es5e'', ''Recorded workflow_branch on t-es5e''. This violates the constraint that decisions must not be startable on a branch by accident; the record now claims a work branch it should never have. Suggested fix: refuse start for an issue whose workflow has no start transition reachable and whose type is a decision (or more generally when the type''s workflow defines no start), with an error pointing at `atelier issue transition <id> decide` on the base; add a CLI test.'
target: atelier-ox63
identity: Independent reviewer (Claude subagent)
commit: f4376ada15eaa6ee51fc9b97dd78e228648811b5
created_at: 2026-10-16T01:12:08.667841257Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/commands/start.rs:95 (no decision guard): `atelier start <decision>` on a standalone decision creates and switches to branch decision/<id> and records workflow_branch on the record, printing only 'todo (no start transition from this status)'. Reproduced in a scratch repo: clean main, file decision t-es5e, `atelier start t-es5e` -> 'git switch -c decision/t-es5e', 'Recorded workflow_branch on t-es5e'. This violates the constraint that decisions must not be startable on a branch by accident; the record now claims a work branch it should never have. Suggested fix: refuse start for an issue whose workflow has no start transition reachable and whose type is a decision (or more generally when the type's workflow defines no start), with an error pointing at `atelier issue transition <id> decide` on the base; add a CLI test.
