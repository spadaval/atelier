---
schema: atelier.review
schema_version: 2
id: atelier-xaqd
title: finding on atelier-z4cr @07418b1b
target: atelier-z4cr
identity: GPT-6.1 Sol (xhigh), independent Desk re-review
commit: 07418b1b66757d2b6f02ff7ba53b3504d10faf7e
created_at: 2026-11-11T02:35:44.671699763Z
event:
  event: finding
  recommendation: fix_now
---

R1 (medium) docs/strategies/desk.md:114-124: The revised executor still cannot satisfy act-is-a-record. It says ready commits the paths it wrote and forbids the Desk from committing, but next/crates/atelier-cli/src/plan.rs:218-293,658-764 only writes records and returns output::planned; output.rs:718-735 explicitly prints 'Wrote, and did not commit' and a Git command for the caller. main.rs:302,338-359 adds no commit. The base tip therefore stays draft, the commit-keyed page stays draft, and the criterion's one commit never exists. This leaves atelier-ja82 unresolved despite correcting the checkout selection. Settle and authorize the commit owner and checkout/index preservation using the actual new command contract, make the strategy, mission Outcome and durable product promise agree, and require proof that saying go creates the intended single commit without committing unrelated edits. Authority currently forbids fixing this by changing the tracker command.
