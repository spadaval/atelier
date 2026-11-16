---
schema: atelier.review
schema_version: 3
round: 2
commit: a1f7d56fcd8ed30b81135e04e8f0e4faa3ff4a75
reviewers:
- thread:delegated-task:command%3Amcp%3A27fa4239-dd67-4ddb-959b-393a1d0e6b23%3Adelegate-task%3Ae995704e-9302-4b7b-8f08-9ca0b33dc34e
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T03:41:30.213637067Z
---

## Summary

Complexity: medium. The change adds host thread state and asynchronous
navigation, with a small selector that chooses from current ties.

Value: high. Opening the current thread's work is a required outcome of the
embedded Desk mission and removes the manual lookup.

Risk of merging it: medium. It changes the page and role line shown to
embedded Desk users; failures are visible and reversible.

The value justifies the size. Each new part serves the required behavior;
a materially smaller change would omit part of the contract. Both earlier
findings are fixed, and I found no new problem requiring action.

Reviewed the full change from master to
`a1f7d56fcd8ed30b81135e04e8f0e4faa3ff4a75`, the changes since round 1,
every Done when line and the Report. Checked message origin and source
validation, identity deduplication, role selection and priority, current-tie
semantics against the existing read API, stale-response handling, navigation
and the role line against the documented host contract.

Created a detached worktree from origin/master, then detached it at the
submitted commit for validation. The installed CLI explicitly directed
review registration and verdict recording to the item branch checkout.
No application or test source was changed.

Validation on the submitted commit: `pnpm test` passed all 57 tests;
`pnpm build` passed its TypeScript check and production build;
`DESK_E2E_PORT=4797 pnpm e2e --workers=2` passed all 106 desktop and phone
interface tests. The fixture host covers the required roles, current versus
past ties, repeated identities after user navigation, switching threads,
unlisted origins and the failed-lookup regression. `git diff --check
master...HEAD` passed. CI run 43799652167's changes job has passed; its Desk
job was still pending at the last check. Final CI success remains a merge
gate. I did not test the external T3 extension or a live Desk server; the
interface tests use the typed fixture API.

## Finding 1: Hide the previous thread's role when the new lookup fails
Recommendation: fix now
Was: 1-qfzg.1
Response: fixed in 2fe204791
Now: fixed

`useFollowThread` clears `followed` when the identity changes, before
starting the new lookup. A failed response therefore leaves the page alone
and cannot retain the previous thread's role line. The new fixture-host
regression passed on desktop and phone: a successful implementer lookup
followed by a 503 leaves the item page open with no role line.

## Finding 2: Remove the trailing blank line that blocks CI
Recommendation: fix now
Was: 1-qfzg.2
Response: fixed in 2fe204791
Now: fixed

The extra blank line is removed. `git diff --check master...HEAD` now exits
successfully, and the corrected branch's CI changes job passed instead of
blocking the Desk job.
