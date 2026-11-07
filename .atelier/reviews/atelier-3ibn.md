---
schema: atelier.review
schema_version: 2
id: atelier-3ibn
title: finding on atelier-h1nc @b29d7717
target: atelier-h1nc
identity: Independent reviewer 2 (Claude Sonnet subagent)
commit: b29d77173838f0e67a446fd90643a9d7ff6666a8
created_at: 2026-10-23T15:08:08.002458721Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 421e196c3d875d7570f8bc6dada71ab38ee5ae1b
    recorded_at: 2026-10-23T15:13:38.097470662Z
    resolution_commit: 052b990459d71d425ebcdf3db6d1a65f9c870666
    reason: Printed unlink ran verbatim did nothing; S1 needs a runnable fix.
    migrated_from: atelier-jshi
---

medium: crates/atelier-cli/src/gate_report.rs blocker_fix: the printed drop-link command 'atelier issue unlink <declarer> <blocker>' omits --role blocked_by; unlink defaults to --role advances, so run verbatim it prints 'No link X -> Y (advances) exists' and removes nothing. Repro in scratch repo under atelier-regroom-review/s1: owner done with 'issue link <owner> <todo-issue> --role blocked_by'; land prints '... or drop the link with `atelier issue unlink atelier-npc2 atelier-ygyl` if it is wrong'; running it fails, adding --role blocked_by works. Fix: append --role blocked_by (and test the exact printed text by executing it).
