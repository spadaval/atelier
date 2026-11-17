---
schema: atelier.item
schema_version: 3
id: atelier-4bxj
title: Stop the extension's CI job failing when the browser is slow to start
status: changes_requested
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- 2ebdc870-0f5d-4afb-9d4a-204040808b53
ready:
  by: sumeet
  at: 2026-11-17T16:28:54.621829728Z
trivial: null
caused_by: null
created_at: 2026-11-17T16:21:23.509642997Z
---

## Goal

The `extension` job of `ci.yml` fails some runs with `Error: Browser startup
timed out: fetch failed` in `t3-extension/test-extension.mjs`, on changes
that do not touch the extension, and passes when run again. It refused the
merge of atelier-kb2g on 2026-11-17 (run 44123741376, 24 seconds into the
test) and failed on `master` the same day (run 43987730356). The next run on
the same code passed both times. Each failure costs a merge one more CI run,
and teaches agents to rerun a red job without reading it.

## Done when

- [x] The Report says why the browser's debugging port is not answering when the test gives up: the cause, from the runs' logs, not a guess.
- [x] The fix is at that cause. A longer timeout or a retry alone is not one unless the Report shows the browser is only slow, with the times measured in CI.
- [x] The test prints how long the browser took to start, so a later run's log shows how near the limit it was.

## Scope

In: `t3-extension/test-extension.mjs`, `t3-extension/run-tests.mjs` and the
`extension` job in `.github/workflows/ci.yml`.

Out: run 43950803020's `TypeError: fetch failed` at 0 seconds, unless it
turns out to share the cause.

## Report

Chrome's first start on a fresh runner is slow because its files have not
been read from the runner's disk yet, and it does finish: measured on 40
fresh runners, it took up to 47 seconds. The test waited 20. It now waits
120 seconds for the browser and prints `Browser started in <n>s`.

### What was measured

A throwaway workflow (pull request 193, run 44127411654, branch
`atelier-4bxj-probe`, never merged) ran 40 jobs on fresh `ubuntu-latest`
runners. Each started Chrome twice with the test's flags and timed how long
until `http://127.0.0.1:19222/json/version` answered, waiting up to 120
seconds. Half the jobs first read every file in `/opt/google/chrome` with
`cat`. Every one of the 80 starts became ready.

| | Seconds until the debugging port answered |
| --- | --- |
| First start, 20 runners | 0.6, 0.7, 0.9, 0.9, 1.1, 1.8, 6.5, 8.4, 8.9, 9.3, 9.3, 10.1, 11.1, 12.7, 15.1, 15.7, 22.6, 30.0, 32.9, 47.0 |
| Second start, same 20 runners | 0.2 to 0.4 |
| First start after reading Chrome's files, 20 runners | 0.4 to 2.0 on 18, and 6.1 on 2 |
| Reading Chrome's files, those 20 runners | 2.7 to 28.0 |

So:

- Four of 20 first starts took longer than the old 20 seconds, and each
  finished. That is the rate the job failed at: 2 of the 19 runs of
  2026-11-16 and 17 that reached the browser.
- The second start on the same runner is always fast, so the delay is a
  cost of the first use, not of the flags or the port.
- Reading the files first takes the time that the start otherwise takes,
  and the start is then fast. The delay is the first read of Chrome's
  files from the runner's disk.

The `dbus` errors in the failed runs' browser logs are in every probe log
too, fast starts included (`DBUS_SESSION_BUS_ADDRESS` is unset on a
runner). They are not the cause.

### The fix

The cause is a property of a fresh runner, which nothing in this repository
can change, so the fix is that the test allows for it: 120 seconds, 2.5
times the slowest of 20. A step that reads Chrome's files before the test
would move the same seconds to a step with no limit and help only this one
workflow; the test's own wait also covers a first start on any other
machine. No retry was added. The other waits keep their 20 seconds, since
no run failed at one.

Out of scope and unchanged: the three runs of 2026-11-16 that failed with
`TypeError: fetch failed` at 0 seconds. They fail before the browser is
started and predate atelier-7tg6's fixture Desk.

The probe's pull request is closed. Its remote branch stays, since deleting
a remote branch is the human's call.

### Validation

- `BROWSER=/usr/bin/google-chrome node t3-extension/run-tests.mjs`, with
  `atelier-desk` and `desk/` built from this branch: 3 of 3 pass, and the
  log has `Browser started in 0.2s`.
- `git diff --check`: no output.

### Responses

- 1-merp.1: fixed in f44174304. The finding was right: the first report inferred that a slow start would finish and had no run showing it. The probe above shows four starts of 22.6 to 47.0 seconds that became ready, and what the time goes to. The Report and the comment at the wait now give the measurement, and the wait is 120 seconds on its strength.
