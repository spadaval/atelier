---
schema: atelier.review
schema_version: 1
id: atelier-zpfx
title: 'crates/atelier-app/src/git_freshness.rs:70-71 — the doc comment says atelier ''keeps MSG_NOSIGNAL for the other Unix targets'', which can be read as Apple no longer passing it; Requests::write (line ~103) still passes MSG_NOSIGNAL on every Unix target including Apple, so both protections apply there. No behavioural consequence; wording only. No action: the comment is not wrong about what other targets rely on, and the code is the source of truth.'
target: atelier-2hh8
identity: Independent reviewer (Claude subagent)
commit: b5dc2f5743e1caf2984cb461489485c8b22deba9
created_at: 2026-10-17T09:16:45.810992956Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-app/src/git_freshness.rs:70-71 — the doc comment says atelier 'keeps MSG_NOSIGNAL for the other Unix targets', which can be read as Apple no longer passing it; Requests::write (line ~103) still passes MSG_NOSIGNAL on every Unix target including Apple, so both protections apply there. No behavioural consequence; wording only. No action: the comment is not wrong about what other targets rely on, and the code is the source of truth.
