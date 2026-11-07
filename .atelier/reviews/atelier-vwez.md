---
schema: atelier.review
schema_version: 2
id: atelier-vwez
title: 'Wording: the land/merge refusal (crates/atelier-cli/src/commands/git.rs:111-120), Design item 1 and the commit message say gates read prune retention from config.toml. No gate does: canonical_retention_days is read only by ''atelier prune'' (main.rs:896-902, prune.rs:869). Only custom_context_types reaches a gate, through validate_link_types/validate_policy. Scenario: a user whose retention edit makes land fail with land_config_uncommitted is told the gate depends on retention, which is false. The refusal itself is still correct, because any config difference is refused. Drop ''and prune retention'' from the message, the comment and the Design text in a follow-up.'
target: atelier-v44v
identity: Independent reviewer (Claude subagent)
commit: 40a0de63cd3897f24acbfcf963d1311137a6cf33
created_at: 2026-10-16T19:37:54.991166667Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: Claude Opus 5.5 (orchestrator)
    commit: 4eb09141feaeedc59b41680efa31bff49c78a26b
    recorded_at: 2026-10-16T19:38:32.566747055Z
    follow_up: atelier-qohr
    reason: Deferred to atelier-qohr as recommended.
    migrated_from: atelier-cgvd
---

Wording: the land/merge refusal (crates/atelier-cli/src/commands/git.rs:111-120), Design item 1 and the commit message say gates read prune retention from config.toml. No gate does: canonical_retention_days is read only by 'atelier prune' (main.rs:896-902, prune.rs:869). Only custom_context_types reaches a gate, through validate_link_types/validate_policy. Scenario: a user whose retention edit makes land fail with land_config_uncommitted is told the gate depends on retention, which is false. The refusal itself is still correct, because any config difference is refused. Drop 'and prune retention' from the message, the comment and the Design text in a follow-up.
