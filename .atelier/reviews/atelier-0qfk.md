---
schema: atelier.review
schema_version: 1
id: atelier-0qfk
title: 'crates/atelier-cli/src/main.rs:909 — bare `work` now runs outside a repository (CanonicalTransactionMode::None plus no load_repository); verified manually (exit 0, same help, -q lists names), but no test pins it, so the carve-out could regress unnoticed. No action: the behavior is a convenience the Done when does not require.'
target: atelier-od8b
identity: Independent reviewer (Claude subagent)
commit: 903423d35d443e1238acad167b032ccc50a7228f
created_at: 2026-10-17T08:07:03.733460193Z
event:
  event: finding
  recommendation: no_action
---

crates/atelier-cli/src/main.rs:909 — bare `work` now runs outside a repository (CanonicalTransactionMode::None plus no load_repository); verified manually (exit 0, same help, -q lists names), but no test pins it, so the carve-out could regress unnoticed. No action: the behavior is a convenience the Done when does not require.
