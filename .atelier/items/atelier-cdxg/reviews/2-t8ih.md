---
schema: atelier.review
schema_version: 3
round: 2
commit: 7072cae3db9527aed8f3a35bede0ffbe452c37ce
reviewers:
- thread:delegated-task:command%3Amcp%3A1418d1cf-deda-41a8-aa76-047c132ca4a2%3Adelegate-task%3A3a2eb515-e5b3-43bb-9c87-93baa33538bd
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-16T16:08:58.080814252Z
---

## Summary

Complexity: high. Three new views, six host-facing reads, caches and a host
message contract span the Rust server, generated types and React app.

Value: high. This brings the views the human approved into the maintained
Desk while preserving the routes and fields its T3 extension already uses.

Risk of merging it: medium. Incorrect reads or layouts would reach the
human, but the change adds read-only views and is reversible.

The value warrants the size. The shared components, generated types and
existing repository cache keep the implementation reasonably contained;
removing a view or read would lose part of the requested outcome.

Reviewed `master...7072cae3db9527aed8f3a35bede0ffbe452c37ce` and the changes
since round 1. Compared the new routes, JSON fields, host messages, views
and retained styles with `spike/t3-home`; checked the generated type exports,
command bounds and caches, input handling, error paths, removed code and
product/architecture documents. The layout assertions cover the 860/859
pixel boundary and the 44 pixel close-button clearance; project-home tests
cover a mission whose only started work is done.

Ran `cargo nextest run -p atelier-desk` (66 passed), `pnpm test` (57 passed),
`pnpm build`, `pnpm e2e --workers=2` (134 passed),
`cargo clippy --all-targets -- -D warnings`, `cargo fmt -- --check` and
`git diff --check`. All passed. The server tests include the inherited-pipe
timeout, shared branch-time cache/invalidation and T3 read fixtures. The
interface tests exercise each carried finding's regression, host messages,
mission-control keys and the intended widths.

All five carried findings are fixed. No new blocking findings. I did not
exercise the separate T3 extension or reinstall the real Desk service, both
outside this item's scope, or independently compare the original screenshot
artifacts. T3 thread behavior was checked against the stand-in and trimmed
real response fixture, rather than a live T3 connection.

## Finding 1: Keep the command deadline active while draining output
Recommendation: fix now
Was: 1-e3g8.1
Response: fixed in c395e4aa3. The deadline covers draining both pipes; a unit test runs `sh -c 'sleep 30 & exit 0'` with a 300 ms limit.
Now: fixed

The receives for both output pipes use the original deadline. The test
where the direct child exits while a descendant retains its pipes passes.

## Finding 2: Keep children visible unless their work is actually shown
Recommendation: fix now
Was: 1-e3g8.2
Response: fixed in c395e4aa3. A child is listed unless all its current ties are drawn above, with its ties on its row; `host.spec.ts` covers a tieless parent whose child implements an item, and a lead whose child's item is in its mission.
Now: fixed

Children are excluded only when all current ties are in the displayed
records; other children retain their tie descriptions. Both the unrelated
parent and represented mission-child interface cases pass.

## Finding 3: Draw the available project home when the roadmap read fails
Recommendation: fix now
Was: 1-e3g8.3
Response: fixed in c395e4aa3. The project home shows the roadmap's error and its missions; `views.spec.ts` fails `/api/roadmap`.
Now: fixed

A failed roadmap read is settled, gets an error banner, and leaves the
mission lanes available. The failed-roadmap interface regression passes.

## Finding 4: Show when This thread cannot read what waits on the human
Recommendation: fix now
Was: 1-e3g8.4
Response: fixed in c395e4aa3. This thread and the project glance show the inbox's error and draw the rest; `host.spec.ts` fails `/api/inbox` for both.
Now: fixed

This thread renders the inbox error; its empty-thread project glance
also settles failed reads and preserves the readable blocks. Both inbox
failure cases pass.

## Finding 5: Cache the branch-time read shared by motion and launch
Recommendation: fix now
Was: 1-e3g8.5
Response: fixed in c395e4aa3. Branch times are read through the Desk's cache keyed on the repository's commits; `views.rs` checks a second motion and launch build nothing, and a new commit is read.
Now: fixed

Motion and launch share Desk::read under branch-times. The repeated-read
test keeps the build counter unchanged, and committing on the branch
invalidates the cache and updates changed_at.
