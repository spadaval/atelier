---
schema: atelier.review
schema_version: 2
id: atelier-jhri
title: finding on atelier-engh @b35525f1
target: atelier-engh
identity: gpt-6.1-sol
commit: b35525f1c74d993546622f48551e87d27e42561c
created_at: 2026-11-10T16:17:25.708401804Z
event:
  event: finding
  recommendation: fix_now
---

medium: next/crates/atelier-cli/src/commands.rs:107 A failed commit leaves a lifecycle act applied only to the checkout/index and breaks retry. Reproduced with the new binary in a scratch repository: after start, install a pre-commit hook that exits 1, then submit a ticked item with a Report. submit exits 2 and HEAD does not move, but item.md is staged as in_review; status now tells the implementer to wait for a reviewer. Remove the hook and retry: submit refuses because it reads in_review, so no submission commit is ever made. The take-over path at lines 81-87 has the same failure: after its commit is rejected, t2 appears to hold the item locally, and rerunning start succeeds as already held without creating the take-over commit; another checkout still sees t1. Restore the pre-command record and index state on add/commit failure, preserving existing ticks, Report and unrelated staged work, or otherwise make the failed operation safely retryable without claiming an uncommitted act completed. Add failure/retry scenarios for both submit and take-over. The existing 61 tests and the isolated build without root crates pass but do not exercise these failures.
