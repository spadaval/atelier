---
schema: atelier.review
schema_version: 2
id: atelier-q4p1
title: finding on atelier-t747 @753b046c
target: atelier-t747
identity: GPT-6.1 Sol independent planning reviewer
commit: 753b046cec958c26e0504daaf8cb680ba44f181c
created_at: 2026-11-11T04:33:49.630630863Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-t747.md:54 (753b046c): medium, FIX NOW. The Limits kept proof equates --jobs 2 with at most two eval agents at once, but it limits top-level scenario jobs, not their agent trees. harness/run.py:430-433 starts two run_one calls; each threaded run can start delegate.py's unpooled subprocess (lines 27-30), and atelier-a4rs explicitly allows native Codex children as well. A lead can add an implementer and that implementer's reviewer. The current proof can pass while the owner's agent concurrency limit is exceeded. Consume the whole-tree concurrency bound supplied by the corrected atelier-a4rs plan (finding atelier-9x7a), specify the matrix scheduling policy that keeps it, and report observed concurrency including children rather than --jobs alone. Do not add an independent scheduling implementation in this item.
