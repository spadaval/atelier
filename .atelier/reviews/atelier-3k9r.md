---
schema: atelier.review
schema_version: 2
id: atelier-3k9r
title: finding on atelier-67nn @136b0461
target: atelier-67nn
identity: sol-review-1
commit: 136b0461779ef72dee612c10a0365dabaf3b1ce9
created_at: 2026-11-10T19:18:46.308474668Z
event:
  event: finding
  recommendation: fix_now
---

medium: evals/agent-factory/scenarios/next-loop/grade.py:25 R1: A successful enclosing shell hides submit refusals from the grader. Using bin-r3/atelier on the real fixture after start, `atelier submit; true` exits 0 and prints "`atelier submit` refused; nothing changed.", but failures() returns []. `atelier submit; atelier submit; true` likewise returns no failures despite two real refusals. REFUSED recognizes "Nothing changed." with an uppercase N, whereas the normal submit refusal uses lowercase. This breaks the required counting of refused calls and can report within_limit for a run that exceeds the limit. Recognize the binary's actual refusal forms and preserve each refused invocation when the shell succeeds; add regression fixtures for these cases and recount the saved transcripts. No new eval agents are needed.

Review at 136b0461779ef72dee612c10a0365dabaf3b1ce9: the six final transcripts show independent approvals and completed merges, with only Codex 9487 visibly refused once. Fresh checks passed: required root diff check, cargo fmt and locked metadata; next/ fmt, clippy -D warnings and all 105 nextest tests; all 7 harness tests and 6 grader fixtures. The counter reproduction above fails the claimed measurement contract. Approval withheld pending R1 and R2. The production workspace under crates/ is unchanged.
