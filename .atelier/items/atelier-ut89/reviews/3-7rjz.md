---
schema: atelier.review
schema_version: 3
round: 3
commit: d6caf8102fd893e584b57e9dd3a2798d83ea4d06
reviewers:
- thread:delegated-task:command%3Amcp%3Ad82b0dfa-fccf-4758-8e2f-e6405235c5b5%3Adelegate-task%3A2c306d78-9296-44bf-ac72-e5f468798a5e
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T23:39:09.139767568Z
---

## Summary

Complexity: high. This adds a native React page, multi-path polling, thread classification and history-derived lanes, and changes the shared layer's navigation and focus.

Value: high. The running briefing and finished retrospective are central outcomes of atelier-osel.

Risk of merging it: medium. The page guides the user's attention and changes a released extension, but it is read-only and can be reverted.

The value warrants the implementation. I found no substantially smaller change that provides the requested running and finished views, and no new problem requiring a fix.

I reviewed the full diff from master through d6caf8102, the changes since round 2, each Done when line and the Report, the model and React page, Markdown renderer, polling hooks, native/framed layer transitions, fixtures and assertions, and documentation. I checked the Desk's live-thread join, full-thread cache and history endpoint against the model's assumptions. The page-title callers in This thread and the project home remain in atelier-9l8h and atelier-wvst; this item supplies openMission for them.

Validation passed: pnpm typecheck, pnpm build, all five suites in node run-tests.mjs, scripts/t3-extension-release.test.sh including bundle and verify, and git diff --check master...HEAD. The mission suite passed all 13 checks, including the new list-running-to-waiting transition with a failed full read, both earlier failed-read cases, the dropped item's lane, keyboard navigation and responsive layouts. The integration test verifies the native page has no iframe, item framing and Back, and proposed-plan framing while preserving the inbox checks.

I inspected the committed 728-pixel running fixture, populated done fixture, Nightly running-dark and Nightly done-light screenshots. I did not start or drive a new Nightly instance; the sparse Nightly done view and unread T3 threads are disclosed in the Report, and the populated states are exercised by browser fixtures. At the last CI check, changes, bundle and test passed; extension remained pending. CI completion and merge remain the implementer's responsibility.

## Finding 1: Do not let a saved full-thread answer hide the live list's waiting status
Recommendation: fix now
Was: 2-2o0e.4
Response: fixed in 7c80fb9be. The live list in the mission's read now says whether a thread is live and what it is doing, either it or the full answer saying `waiting` lists it, and the full answer gives only the latest line and times. A new check in `test-mission.mjs` has a thread's list status turn to waiting while its next full read fails; it is listed under *Needs you* and the sentence says one thread waits on you. The check fails on the previous model.
Now: fixed

Verified fixed. liveWorkers now classifies liveness from the mission's live-list mark and recognizes waiting reported by either that mark or the full answer. A saved running response cannot erase the live list's waiting status. The new browser regression check first draws the worker under At work, changes the mission's mark to waiting while the next full read fails, and verifies that the worker moves to Needs you and the sentence says one thread waits on you. The earlier quiet-thread and never-read checks continue to pass.
