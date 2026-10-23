---
schema: atelier.review
schema_version: 1
id: atelier-3ibn
title: finding on atelier-h1nc @b29d7717
target: atelier-h1nc
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: b29d77173838f0e67a446fd90643a9d7ff6666a8
created_at: 2026-10-23T15:08:08.002458721Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-cli/src/gate_report.rs blocker_fix: the printed drop-link command 'atelier issue unlink <declarer> <blocker>' omits --role blocked_by; unlink defaults to --role advances, so run verbatim it prints 'No link X -> Y (advances) exists' and removes nothing. Repro in scratch repo under atelier-regroom-review/s1: owner done with 'issue link <owner> <todo-issue> --role blocked_by'; land prints '... or drop the link with `atelier issue unlink atelier-npc2 atelier-ygyl` if it is wrong'; running it fails, adding --role blocked_by works. Fix: append --role blocked_by (and test the exact printed text by executing it).
