---
schema: atelier.review
schema_version: 1
id: atelier-k06k
title: crates/atelier-cli/src/workflow_help.rs:27 — The help is rebuilt by re-parsing argv against a tree whose --status/--category use PossibleValuesParser. If the user passes an invalid status before --help (for example 'issue list --status bogus --help'), the re-parse returns InvalidValue, and the code falls back to static help with no workflow values. Exit stays 0, and help still appears. The fallback is benign and intentional; recording it only so the behaviour is known.
target: atelier-7w1g
identity: Independent reviewer (Claude subagent)
commit: b88141491d3879d251fd8f2effa493ff119c1c0b
created_at: 2026-10-17T07:31:06.263890946Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/src/workflow_help.rs:27 — The help is rebuilt by re-parsing argv against a tree whose --status/--category use PossibleValuesParser. If the user passes an invalid status before --help (for example 'issue list --status bogus --help'), the re-parse returns InvalidValue, and the code falls back to static help with no workflow values. Exit stays 0, and help still appears. The fallback is benign and intentional; recording it only so the behaviour is known.
