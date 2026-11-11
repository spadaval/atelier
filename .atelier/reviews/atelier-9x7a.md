---
schema: atelier.review
schema_version: 2
id: atelier-9x7a
title: finding on atelier-a4rs @753b046c
target: atelier-a4rs
identity: GPT-6.1 Sol independent planning reviewer
commit: 753b046cec958c26e0504daaf8cb680ba44f181c
created_at: 2026-11-11T04:32:58.620499776Z
event:
  event: finding
  recommendation: fix_now
---

.atelier/issues/atelier-a4rs.md:52-55 (753b046c): medium, FIX NOW. Two jobs do not bound the number of eval agents. harness/run.py:430-433 caps concurrent run_one calls only; harness/delegate.py:27-30 starts another real agent without consulting that pool, and this plan also permits Codex spawn_agent. Two threaded runs can therefore run their roots and their reviewers concurrently, and a lead/implementer/reviewer chain adds another level. This does not establish the owner's limit of at most two eval agents at once. Specify how the smoke runs and later matrix will cap the entire agent tree, including delegate and native children, and make the README command and a non-agent check demonstrate that bound. A simple serial scheduling/restricted delegation policy is sufficient if it actually maintains the bound; a second scheduler is not required.
