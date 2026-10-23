---
schema: atelier.review
schema_version: 1
id: atelier-g2pl
title: finding on atelier-l7nz @e6210fb6
target: atelier-l7nz
identity: Independent reviewer (Claude Sonnet subagent)
commit: e6210fb6abda8f05565720c55d5ce1125e2f69c2
created_at: 2026-10-23T16:20:23.328829754Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/src/commands/evidence.rs print_confirmation (Pass arm, gate_report::verdict("evidence record", &[])): a pass always ends 'evidence record: passed' and '✓', but a manual pass recorded on a checkout with uncommitted code is stale at once (capture refuses dirty trees for commands, the --summary path does not). Repro: 'echo x >> file; atelier evidence record --target <id> --kind test --summary s --result pass' prints '✓ pass ... evidence record: passed'; 'evidence list <id>' then shows '⚠ stale (uncommitted code changes)'. Pre-existing acceptance of dirty manual receipts, but the new verdict claims a gate pass it does not give. Suggest computing the verdict from item_proof for the target (fresh_pass == this id) or warning.
