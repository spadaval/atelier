---
schema: atelier.review
schema_version: 1
id: atelier-oy90
title: finding on atelier-h1nc @60f6746f
target: atelier-h1nc
identity: Independent reviewer 3 (Claude Sonnet subagent)
commit: 60f6746fda2847823f0253bb6cf33535cbaef11d
created_at: 2026-10-23T15:16:58.251423755Z
event:
  event: finding
  recommendation: defer
---

low: crates/atelier-cli/tests/support/next_steps.rs:78 the helper proves each printed fix exits 0 and changes state, but not that it clears the failing check (no gate rerun), and the tests only exercise blocked_by unlink and a todo child; the blocks-declared unlink, backlog/blocked/review children, closed-ancestor blocker, and custom-workflow routes (no supersede, extra terminal transition) pass only by my manual runs. Suggested: after each alternative rerun the gate and assert that check is gone; add those fixtures.
