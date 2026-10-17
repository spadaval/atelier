---
schema: atelier.review
schema_version: 1
id: atelier-378y
title: 'crates/atelier-app/src/record_ref.rs:90 — not_found''s ''Did you mean a, b, c (and N more)?'' names no command, while the no-match branch names kind.list_command(). Low impact: near matches are a suggestion list, and the list command shows every record. Pre-existing, outside ytxg''s Goal; follow-up could append the list command as the empty case does.'
target: atelier-ytxg
identity: Independent reviewer (Claude subagent)
commit: eef5a7a830429a69d180f520e30ae7fb13698d64
created_at: 2026-10-17T09:13:24.086413342Z
event:
  event: finding
  recommendation: defer
---

crates/atelier-app/src/record_ref.rs:90 — not_found's 'Did you mean a, b, c (and N more)?' names no command, while the no-match branch names kind.list_command(). Low impact: near matches are a suggestion list, and the list command shows every record. Pre-existing, outside ytxg's Goal; follow-up could append the list command as the empty case does.
