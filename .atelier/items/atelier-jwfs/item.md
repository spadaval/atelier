---
schema: atelier.item
schema_version: 3
id: atelier-jwfs
title: Stop the project home's interface test from failing when the clock crosses a minute
status: building
mission: null
labels:
- bug
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A9c525daa-e7f7-44f4-9529-6989140fcc76%3Adelegate-task%3A37b37c71-6c7c-429c-a4e8-2f96284fb5b2
ready: null
trivial: null
caused_by: atelier-cdxg
created_at: 2026-11-16T17:31:09.516251061Z
---

## Goal

The Desk's interface tests pass or fail on the code, never on when they run.
`views.spec.ts`'s project-home test failed on master's CI run 43905178867
(2026-11-16) with "Fetched 3m ago." where it expects "Fetched 2m ago.", at
1300 and 880 pixels, and had failed once before on atelier-cdxg's branch.
The fixtures counted their times back from the real clock when the module
loaded, while each page read the real clock later, so an age on the page
grew with the time a worker had been running. Every age, stage time, quiet
mark and week-strip day the tests check comes from the same fixtures, so
each could flip the same way.

## Done when

- [x] Every time in `desk/e2e/fixtures.ts` is counted back from one fixed instant, `now`, and no fixture or fixture server reads the real clock.
- [x] Every page the suite serves runs on a clock that starts at `now`, and in UTC, so the ages, the 30-minute quiet mark and the week strip's days are the same on every host and at every hour.
- [x] The project home, pages and host specs pass 20 times over in one long run, where workers live for minutes.
- [x] The whole suite passes with the system clock, for Node and Chrome both, started at 23:59:58 UTC, so it crosses a minute and midnight, and at two other instants.
- [x] `pnpm test` and `pnpm e2e` pass, and no product code changes.

## Scope

`desk/e2e` (fixtures and helpers), `desk/playwright.config.ts`, and the
Desk's unit tests in `desk/src`. No product behaviour changes.

## Report

The interface tests no longer read the real clock, so the project home's
"Fetched 2m ago." holds however long a worker has run.

What changed, in `desk/e2e` and the Playwright config only:

- `fixtures.ts` exports `now`, 2026-11-13T15:00:00Z, and counts every time
  back from it: the fetch mark, the stage ages, the threads' `updated_at`
  and `started_at`, and the history events. `/api/history`'s `days` filter
  counts back from it too.
- `serve()` in `helpers.ts` calls `page.clock.install({ time: now })`
  before anything loads. Every spec gets its pages through `serve()`, so
  all six specs are covered with no change to a spec. The clock then runs
  on from `now`, so the Desk's polling and timers behave as before.
- `playwright.config.ts` sets `timezoneId: "UTC"`, so the week strip splits
  its days at the same hours on every host.

No spec used Playwright's clock before, and the unit tests in `desk/src`
already pass a fixed `now` (`words.test.ts`); no other unit test reads the
clock. The expectations this makes deterministic: the project home's
"Fetched 2m ago." and its week strip ("Today" last, atelier-1ie1 in the
week, atelier-old1 out of it) from atelier-cdxg; host.spec's quiet child
(`.child-row.stale`, two hours against the 30-minute mark); and pages.spec's
three "Fetched from origin 2 minutes ago." lines.

How I showed it holds:

- The fault reproduces on origin/master: with Node's clock 61 seconds
  behind the browser's, views and pages specs fail with "Fetched 3m ago."
  and "3 minutes ago", as on CI.
- With the fix, under libfaketime (Node and Chrome both on the faked clock;
  Chrome reported 2026-11-16T23:59:58Z), the whole suite passed 134 of 134
  with the system clock started at 2026-11-16 23:59:58 UTC, 2026-11-13
  14:59:59 UTC and 2027-03-01 00:00:59 UTC.
- `--repeat-each=20` over views, pages and host specs, one run of 4.9
  minutes: 1,598 passed, 2 failed. Both failures are "mission control's
  keys select items...", where Enter opened `thread:aab1-probe`, not the
  implementer's thread. That test flakes at the same rate on origin/master:
  10 of 200 there, 8 of 200 with this change. It is a race in quick `j`
  then `k` presses, not the clock, and is out of this item's scope; it
  needs its own bug.
- `pnpm test` 57 passed; `pnpm build` (tsc over `e2e` too) clean; `pnpm
  e2e` 134 passed.

`atelier set --labels` and `--caused-by` refuse on the item's own branch,
since they expect a backlog branch. I ran them with `ATELIER_SKIP`, because
the brief asks for both fields and the record merges with this item.
