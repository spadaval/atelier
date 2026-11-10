---
schema: atelier.review
schema_version: 2
id: atelier-utim
title: finding on atelier-fcb7 @6f837e9f
target: atelier-fcb7
identity: gpt-6-astra-plan-r1
commit: 6f837e9f52a011db821640751ec20b664759a850
created_at: 2026-11-10T00:59:59.972595271Z
event:
  event: finding
  recommendation: fix_now
---

high docs/strategies/redesign-architecture.md:207-210: base readers discover started items by a branch field that start writes only on the work branch, so the base still advertises ready work and cannot orient the lead or prevent duplicate starts. Today's site enumerates all local refs (crates/atelier-site/src/lib.rs:329), not base-record branch fields. Specify ref discovery, branch-versus-base precedence including done items, and atomic start collision handling; test start --new and two workers from separate checkouts.
