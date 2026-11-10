---
schema: atelier.review
schema_version: 2
id: atelier-0tx8
title: finding on atelier-zrss @1e0e6d51
target: atelier-zrss
identity: gpt-6.1-sol
commit: 1e0e6d51877194c06d65736e7f7c7e760715d6e7
created_at: 2026-11-10T21:35:59.541130891Z
event:
  event: finding
  recommendation: fix_now
---

Medium: .atelier/issues/atelier-zrss.md:35,49: The bypass definition omits existing rules outside the act table. The Context claim that every command refusal already comes from acts.rs is false: next/crates/atelier-cli/src/plan.rs has off_base, set status restrictions and save validation; merge.rs:108,137 refuses failed CI and configured merge checks outside acts::check. A worker can satisfy all five named positive tests while those rules remain unskippable, contrary to the specifications saying every rule of a command can be skipped. Distinguish a successful forge query reporting failed CI or an executed check returning failure from a Git/forge operation that could not complete. Also distinguish parseable record validation from an unreadable record. Smallest fix: remove the act-table-only definition and false Context premise, explicitly cover these existing command rules, and require positive bypass tests for a configured merge-check failure, a reported CI failure, a direct planning guard and parseable validation, alongside the existing negative tests for parse, missing target, conflict, moved inputs and adapter failures. Keep one bypass interface; this does not require adding commands, options or another rules framework. Recommendation: FIX NOW. Planning verdict: merge after this contract correction; the intended scope is worth building.
