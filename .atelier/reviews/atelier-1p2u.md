---
schema: atelier.review
schema_version: 2
id: atelier-1p2u
title: finding on atelier-z4cr @7d7551a8
target: atelier-z4cr
identity: independent-planning-review-gpt-6-astra
commit: 7d7551a890e9dcf0043882aac7e9ab1098f86309
created_at: 2026-11-11T06:24:46.202516019Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-z4cr.md:34-35 moves the browser proof to validation, but does not preserve the strategy Success Criteria requirement that act-is-a-record be shown by a test in the Desk suite. h3hl:35 compares POST with a direct core call on synthetic records; l2aj:44 uses fixture responses; va99 still only names desk-matches-cli. All of those can pass without a browser act on a converted clone matching the actual CLI. FIX NOW: explicitly reserve in va99 (its provisional goal/Done when is enough at this boundary) ownership of a repeatable Desk-suite test using the real browser, server and new CLI on twin converted clones, checking commit count/message/paths, normalized records and the ready page. The current API test remains useful component proof. Running the integration test at validation rather than the Rust-free desk CI job is fine; dropping its suite ownership is not within Authority.
