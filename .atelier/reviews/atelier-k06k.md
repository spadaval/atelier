---
schema: atelier.review
schema_version: 2
id: atelier-k06k
title: crates/atelier-cli/src/workflow_help.rs:27 — The help is rebuilt by re-parsing argv against a tree whose --status/--category use PossibleValuesParser. If the user passes an invalid status before --help (for example 'issue list --status bogus --help'), the re-parse returns InvalidValue, and the code falls back to static help with no workflow values. Exit stays 0, and help still appears. The fallback is benign and intentional; recording it only so the behaviour is known.
target: atelier-7w1g
identity: Independent reviewer (Claude subagent)
commit: b88141491d3879d251fd8f2effa493ff119c1c0b
created_at: 2026-10-17T07:31:06.263890946Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 7bca3f99217647ceeef03961aecd9287e88c21cd
    recorded_at: 2026-10-17T07:41:23.223590486Z
    reason: 'Accepted: help with an invalid preceding value falls back to plain help and exits 0, as intended.'
    migrated_from: atelier-y8c3
---

crates/atelier-cli/src/workflow_help.rs:27 — The help is rebuilt by re-parsing argv against a tree whose --status/--category use PossibleValuesParser. If the user passes an invalid status before --help (for example 'issue list --status bogus --help'), the re-parse returns InvalidValue, and the code falls back to static help with no workflow values. Exit stays 0, and help still appears. The fallback is benign and intentional; recording it only so the behaviour is known.
