---
schema: atelier.review
schema_version: 1
id: atelier-ep6p
title: 'crates/atelier-cli/src/main.rs:101-103,114-116,566 with docs/product/human-cli-output.md:198-199 — Done when requires enumerated values to show as [possible values: ...], and the new Help section states that every option with a fixed set of values does so. Fixed sets still appear only in prose and are checked at runtime: review finding --recommendation and review disposition --decision (fix_now|defer|no_action, validated in commands/review.rs:9-15), evidence attach <TARGET_KIND> (issue|evidence), and evidence record/attach --role (validates only). Failure scenario: ''atelier help review finding'' shows no possible values; ''review finding ... --recommendation bogus'' fails with ''Error: Invalid decision'' and exit 1, not clap''s usage error with exit 2, so the governing doc is false on a current path. Fix: declare value_parser = [...] for these, which turns bad input into a usage error with exit 2 as the doc''s argument rules intend, or narrow the doc sentence and the Done when to the options actually covered.'
target: atelier-7w1g
identity: Independent reviewer (Claude subagent)
commit: b88141491d3879d251fd8f2effa493ff119c1c0b
created_at: 2026-10-17T07:31:05.859123617Z
event:
  event: finding
  recommendation: fix_now
---

crates/atelier-cli/src/main.rs:101-103,114-116,566 with docs/product/human-cli-output.md:198-199 — Done when requires enumerated values to show as [possible values: ...], and the new Help section states that every option with a fixed set of values does so. Fixed sets still appear only in prose and are checked at runtime: review finding --recommendation and review disposition --decision (fix_now|defer|no_action, validated in commands/review.rs:9-15), evidence attach <TARGET_KIND> (issue|evidence), and evidence record/attach --role (validates only). Failure scenario: 'atelier help review finding' shows no possible values; 'review finding ... --recommendation bogus' fails with 'Error: Invalid decision' and exit 1, not clap's usage error with exit 2, so the governing doc is false on a current path. Fix: declare value_parser = [...] for these, which turns bad input into a usage error with exit 2 as the doc's argument rules intend, or narrow the doc sentence and the Done when to the options actually covered.
