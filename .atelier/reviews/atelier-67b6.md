---
schema: atelier.review
schema_version: 2
id: atelier-67b6
title: crates/atelier-cli/src/commands/relate.rs:94-103 — custom roles are appended without deduplicating against built-ins, so a config with custom_context_types = [related, ...] lists 'related' twice (verified in a fixture). Cosmetic and config-dependent; every listed value is still accepted. Better addressed, if ever, by project_config rejecting redundant custom roles.
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:32.025899066Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: Orchestrator (Claude, atelier-o3lm)
    commit: 5f00943ce60e1c6259d43098c548683140d5b06a
    recorded_at: 2026-10-17T09:55:36.552999659Z
    reason: 'Accepted: a custom role repeating a built-in one is listed twice; cosmetic.'
    migrated_from: atelier-rmt6
---

crates/atelier-cli/src/commands/relate.rs:94-103 — custom roles are appended without deduplicating against built-ins, so a config with custom_context_types = [related, ...] lists 'related' twice (verified in a fixture). Cosmetic and config-dependent; every listed value is still accepted. Better addressed, if ever, by project_config rejecting redundant custom roles.
