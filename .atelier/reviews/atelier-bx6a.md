---
schema: atelier.review
schema_version: 2
id: atelier-bx6a
title: finding on atelier-vqoy @bc495ebb
target: atelier-vqoy
identity: codex-reviewer (gpt-6.1-sol)
commit: bc495ebb28bd69550c3272e6253a96f57a727096
created_at: 2026-11-09T05:31:14.692857414Z
event:
  event: finding
  recommendation: fix_now
---

low: R1 crates/atelier-cli/src/commands/merge_push.rs:200 and crates/atelier-cli/tests/forge_ci.rs:258 introduce unused code that fails the required Rust hazard scan. cargo clippy -j 4 --all-targets -- -A warnings -D dead_code -D unused_variables -D unused_imports -D unused_mut exits 101 for the unused base parameter of push_branch. Independently, cargo clippy -j 4 -p atelier-cli --test forge_ci -- -A warnings -D dead_code exits 101 for run_sequence, whose callers exist only with test-hooks. docs/architecture/quality/rust-quality-hazard-scans.md requires this scan for Rust changes; this is a demonstrated check failure, not a style preference. Remove base from push_branch and its call, add cfg(feature = "test-hooks") to run_sequence, and rerun the required scan and affected tests before approval.
