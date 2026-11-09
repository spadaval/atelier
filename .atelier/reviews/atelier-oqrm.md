---
schema: atelier.review
schema_version: 2
id: atelier-oqrm
title: finding on atelier-7tul @ba6f39c6
target: atelier-7tul
identity: codex-reviewer (gpt-6.1-sol)
commit: ba6f39c6d5624b7ba0f503e58de4dbcb454c7b5c
created_at: 2026-11-09T03:36:05.545262647Z
event:
  event: finding
  recommendation: fix_now
---

medium: crates/atelier-app/src/ci_passed.rs:66 R1: HEAD having a second parent does not establish that this is Atelier's synthetic merge checkout. evaluate_session also calls this check in an ordinary completed work branch. Reproduced with a legacy done owner carrying src.rs, an accepted approval and no CI runs: land initially reports ci.passed open; after a clean merge of a record-only main commit, land reports ci.passed: the branch changes only records, so it needs no CI run, then land: passed. HEAD^2 is main and does not contain src.rs, so the gate has checked the wrong code. Distinguish the synthetic merge context from an ordinary branch (prefer explicit captured base/head pins), and add a regression covering land on a completed branch whose HEAD is a base-sync merge. This does not bypass the separate merge gate, but it makes land's proof false on a supported legacy path.
