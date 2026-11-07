---
schema: atelier.review
schema_version: 2
id: atelier-hf3z
title: finding on atelier-0uaj @93e82219
target: atelier-0uaj
identity: Independent reviewer 5 (Claude Sonnet subagent)
commit: 93e822196a3baede9b848a94d926edbdd224e6be
created_at: 2026-10-23T17:02:21.624222405Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Orchestrator (Claude Opus 5.5, rc8j)
    commit: 60bb7fed56cb0444f54ae01147e866fbff3e3b2e
    recorded_at: 2026-10-23T17:02:44.750653699Z
    follow_up: atelier-zegg
    reason: Rare path; the gate result is correct; the warning rides with the follow-up.
    migrated_from: atelier-eygx
---

low: review_gate.rs append_only: when the offending commit is a merge, the printed 'git rebase --autostash --onto <M>~1 <M>' drops the whole merge including the merged side branch's work, silently (checked: side file gone after the fix). Gate then passes. Suggest the line name this, or that one-line recovery be a revert of just the record path.
