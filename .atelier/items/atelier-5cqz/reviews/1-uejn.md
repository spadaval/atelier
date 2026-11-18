---
schema: atelier.review
schema_version: 3
round: 1
commit: 82f182e81d37c1841c348ed3959f0c24eb8c7d06
reviewers:
- thread:delegated-task:command%3Amcp%3Abebed557-bba1-4a35-8aed-23f6b357efe1%3Adelegate-task%3Abce0f89f-5711-4606-a9bb-d0196cc8b2cc
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-18T00:41:10.281406786Z
---

## Summary

- Complexity: medium. The instance gains an MCP client, temporary token handling and a second model run to keep a thread live.
- Value: medium. The existing test instance can now exercise framed pages and populated running and finished missions without a second Desk.
- Risk of merging it: low. The changes affect disposable test fixtures and are easy to undo; the isolation check passes.

The value justifies the added machinery. The second thread has a concrete purpose: the first has finished, and the mission page lists only live threads. I found no smaller change that meets all the criteria while preserving the existing answered thread and draft.

Reviewed `master...82f182e81d37c1841c348ed3959f0c24eb8c7d06`, the three changed source/documentation files, the Report and its screenshots. Traced the fixture's dated commit subjects through the Desk history API and the mission page's lane and round calculations. Checked the instance's environment, pinned CLI wrapper and MCP connection against the Desk client. No shipping extension or Desk source changed.

All six Done when lines hold. Independently started the pinned Nightly in `/var/tmp/atelier-5cqz-review-uejn`, using debugging port 9341 and backend port 47341:

- `GET /api/embed` returned exactly `t3code://app`. Opened the running mission, clicked its Open item button and opened the sidebar Inbox. Both real frames connected and received all 16 host theme tokens.
- The running mission API reported `t3.reachable: true` and the new instance's implementer thread. Its rendered page showed First item under At work and no unavailable-T3 message.
- The finished page rendered its Result, two lanes and two merged items: Reviewed once with 1 round and 4h 0m, Reviewed twice with 2 rounds and 8h 30m, ordered most rounds first. The history API returned the expected dated steps.
- `test-contract.mjs` passed all 35 probes with this instance's thread and draft.
- A separate fresh `t3-instance.mjs isolation` run passed, including both thread IDs. OpenCode held no authenticated integrations. Stopped both review instances afterwards.
- All five non-T3 test files passed. The first runner attempt encountered an occupied port 19222 and `/tmp` disk quota errors. Re-ran test-extension with only its debug port changed to 19322 in memory; re-ran test-mission and test-update with TMPDIR under the review root. These passed without changing repository files. test-components and test-fixture-desk passed on the original attempt.
- `git diff --check` passed, and PR #206's scheduled checks passed at the reviewed head.

The committed screenshots match the independently observed pages. Windows was not checked; this item changes the Linux test instance. No fix-now finding.

## Finding 1: Approval-waiting threads appear under At work instead of Needs you
Recommendation: defer
Taken: atelier-9f7b by thread:delegated-task:command%3Amcp%3Ad12574ac-fe2d-4335-bf72-c30aa232c7f3%3Adelegate-task%3A968c2e4c-7db7-40ce-813d-afb0b4162b34

The Report's limitation is real. The fixture waits for `pendingRequestCount > 0` in `t3-extension/t3-instance.mjs:302-305`, but the mission API reports that thread as running and the mission page places it under At work. `T3Thread` in `crates/atelier-desk/src/t3.rs:60-72` discards pendingRequestCount, and `liveWorkers` in `t3-extension/src/mission/model.ts:94` recognises waiting only from a status string. The committed running screenshot and the independently started instance both show the mismatch.

Carry T3's pending-request state through the Desk and use it when classifying workers in a follow-up. This predates the fixture change, is already disclosed in the Report, and needs Desk and extension source changes explicitly excluded from this item. It does not block the requested fixture and connection improvements.
