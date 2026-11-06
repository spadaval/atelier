---
schema: atelier.review
schema_version: 1
id: atelier-540u
title: finding on atelier-rkxm @09c94fb3
target: atelier-rkxm
identity: lighter-workflow reviewer for atelier-rkxm (claude-opus-5-5)
commit: 09c94fb3f6a888ada6cde0fa1f7e9e75d971e382
created_at: 2026-11-06T23:54:55.850298038Z
event:
  event: finding
  recommendation: fix_now
---

low: .atelier/issues/atelier-rkxm.md (type reader table, task row) says task is read by issue_cli.rs:28 'only', but production code also hard-codes it at crates/atelier-cli/src/commands/bundle.rs:1650 (default_issue_type for bundle import) and crates/atelier-app/src/record_history.rs:204 (epic migration target when reading historical records). A type cut that trusts 'only' would leave bundle import defaulting to a removed type. Fix: list both, as the feature row does for its epic-migration default. Nit in the same note: issue_status_is_terminal has 20 production call sites at 05f01d83, not 22.
