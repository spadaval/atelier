---
schema: atelier.review
schema_version: 2
id: atelier-4em4
title: finding on atelier-zrss @5959a1ba
target: atelier-zrss
identity: reviewer-sol-r1
commit: 5959a1ba4d5f77746047b7eca95f9699bc275362
created_at: 2026-11-11T00:16:33.620287049Z
event:
  event: finding
  recommendation: fix_now
---

Medium: next/crates/atelier-cli/src/git.rs:273: R3. Successful Git subprocess output is captured and discarded, so a rule skipped by a hook inside an Atelier command is never printed to the caller. Reproduced in a scratch clone with the new hooks installed: start an item, tick its checklist and write its Report, then set blocked_by to a nonexistent item. Plain submit fails in pre-commit validation; submit with ATELIER_SKIP succeeds but prints only Submitted and review instructions, with no skipped rule or reason on either stream. Its hook deliberately writes no trailer, so this also removes the only required explanation of the bypass from the output. Preserve and relay hook skip output from successful commit and push subprocesses, and add an integration test that invokes submit through the installed scratch hook rather than invoking Git directly. Done when 1 requires each skipped rule and reason to be printed.
