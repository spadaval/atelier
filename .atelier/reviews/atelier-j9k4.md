---
schema: atelier.review
schema_version: 1
id: atelier-j9k4
title: 'crates/atelier-app/src/use_cases.rs:19 — error recovery steps still end with an inline ''Next: ...'' sentence (also prune.rs:708/711, bundle.rs:729/769/802). The Goal lists use_cases.rs:19 among the footer names, but the Done-when bullets govern footers, and an inline stderr recovery sentence is not a footer section, so I judge it acceptable. The gap is documentation: the new Next Footer rule says no view prints its own footer line and does not say errors keep an inline ''Next:'' recovery sentence, so a later reader cannot tell whether these are violations. Follow-up: one sentence in human-cli-output.md (Views, ''Errors ... give a useful recovery step'') naming the inline ''Next:'' form.'
target: atelier-onwh
identity: Independent reviewer (Claude subagent)
commit: 81e00ba8711cfead5a7dd7743deb6cf53d00af80
created_at: 2026-10-17T07:08:38.358205754Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/use_cases.rs:19 — error recovery steps still end with an inline 'Next: ...' sentence (also prune.rs:708/711, bundle.rs:729/769/802). The Goal lists use_cases.rs:19 among the footer names, but the Done-when bullets govern footers, and an inline stderr recovery sentence is not a footer section, so I judge it acceptable. The gap is documentation: the new Next Footer rule says no view prints its own footer line and does not say errors keep an inline 'Next:' recovery sentence, so a later reader cannot tell whether these are violations. Follow-up: one sentence in human-cli-output.md (Views, 'Errors ... give a useful recovery step') naming the inline 'Next:' form.
