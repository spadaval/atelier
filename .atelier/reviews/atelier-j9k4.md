---
schema: atelier.review
schema_version: 2
id: atelier-j9k4
title: 'crates/atelier-app/src/use_cases.rs:19 — error recovery steps still end with an inline ''Next: ...'' sentence (also prune.rs:708/711, bundle.rs:729/769/802). The Goal lists use_cases.rs:19 among the footer names, but the Done-when bullets govern footers, and an inline stderr recovery sentence is not a footer section, so I judge it acceptable. The gap is documentation: the new Next Footer rule says no view prints its own footer line and does not say errors keep an inline ''Next:'' recovery sentence, so a later reader cannot tell whether these are violations. Follow-up: one sentence in human-cli-output.md (Views, ''Errors ... give a useful recovery step'') naming the inline ''Next:'' form.'
target: atelier-onwh
identity: Independent reviewer (Claude subagent)
commit: 81e00ba8711cfead5a7dd7743deb6cf53d00af80
created_at: 2026-10-17T07:08:38.358205754Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: c5fa522e6015a1c9b6e7ebe6b466ceded5f7b34f
    recorded_at: 2026-10-17T07:12:54.236525181Z
    resolution_commit: ab0b9724d7ef9197a7138e2509fa8c216343fe92
    reason: 'Fixed now instead of deferred: the Views section names the inline Next: recovery sentence for errors.'
    migrated_from: atelier-d4cn
---

crates/atelier-app/src/use_cases.rs:19 — error recovery steps still end with an inline 'Next: ...' sentence (also prune.rs:708/711, bundle.rs:729/769/802). The Goal lists use_cases.rs:19 among the footer names, but the Done-when bullets govern footers, and an inline stderr recovery sentence is not a footer section, so I judge it acceptable. The gap is documentation: the new Next Footer rule says no view prints its own footer line and does not say errors keep an inline 'Next:' recovery sentence, so a later reader cannot tell whether these are violations. Follow-up: one sentence in human-cli-output.md (Views, 'Errors ... give a useful recovery step') naming the inline 'Next:' form.
