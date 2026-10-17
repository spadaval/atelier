---
schema: atelier.review
schema_version: 1
id: atelier-67b6
title: crates/atelier-cli/src/commands/relate.rs:94-103 — custom roles are appended without deduplicating against built-ins, so a config with custom_context_types = [related, ...] lists 'related' twice (verified in a fixture). Cosmetic and config-dependent; every listed value is still accepted. Better addressed, if ever, by project_config rejecting redundant custom roles.
target: atelier-pyfr
identity: Independent reviewer (Claude subagent)
commit: 50adccc26e44b3a509a834acccb8e7f573d8cd69
created_at: 2026-10-17T09:52:32.025899066Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/src/commands/relate.rs:94-103 — custom roles are appended without deduplicating against built-ins, so a config with custom_context_types = [related, ...] lists 'related' twice (verified in a fixture). Cosmetic and config-dependent; every listed value is still accepted. Better addressed, if ever, by project_config rejecting redundant custom roles.
