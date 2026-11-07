---
schema: atelier.review
schema_version: 2
id: atelier-kctm
title: finding on atelier-nn67 @ae99fb06
target: atelier-nn67
identity: independent reviewer subagent (claude-opus-5-5), atelier-nn67
commit: ae99fb066f77ea27886e584db6d77b84bad698e3
created_at: 2026-11-07T14:50:15.025883763Z
event:
  event: finding
  recommendation: fix_now
---

low: crates/atelier-cli/src/commands/session.rs:684 on an unowned branch that changes .atelier/workflow.yaml or config.toml, land now prints 'it changes no Atelier tracker files since main, so there is nothing to land' (reproduced on a scratch repo with this branch's binary; the new light_path test asserts this message on exactly that branch). Since tracker_changes_since now counts only records, 'tracker files' at 684-685 (and 'tracker files/changes' at 126, 162, 634, 639, 642) contradicts what the agent just committed. Fix: say 'tracker records', as sessions-and-hooks.md now does.
