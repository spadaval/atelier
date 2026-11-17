---
schema: atelier.item
schema_version: 3
id: atelier-4bxj
title: Stop the extension's CI job failing when the browser is slow to start
status: in_review
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

The browser is not stuck when the test gives up: it is still starting. The
wait for it was 20 seconds, and a hosted runner's first start of Chrome
sometimes takes longer. The wait is now 90 seconds, and the test prints
`Browser started in <n>s`.

What the logs show, from the 19 runs of the `extension` job on 2026-11-16
and 17 that reached the browser:

- In both failures (runs 43987730356 and 44123741376) Chrome wrote its first
  line to its log 16.0 and 16.8 seconds after the test began, and was still
  writing lines 0.9 and 1.1 seconds before the test gave up. It never
  printed a DevTools error, such as a port in use. `fetch failed` is the
  test's own poll finding the debugging port not open yet.
- In the other 17 runs, the time from the test's start to its first
  `PASS` line, which is mostly the browser's start, was 1.4, 1.6, 1.8, 2.0,
  2.4, 3.6, 3.9, 7.4, 8.6, 9.3, 9.5, 9.8, 12.5, 12.6, 13.9, 19.0 and 19.5
  seconds. The two failures are the end of that spread, cut off at 20, not
  a second kind of run.
- On this host, with Chrome already in the page cache, the browser starts
  in 0.2 seconds.

What I did not establish is what Chrome spends those seconds on. A runner
is a fresh machine whose disk has not been read, and Chrome is a large
program, so a slow first read is my inference; no log line shows it. The
`dbus` errors in the browser's log come from a runner with no session bus
and I did not measure whether they add delay. The new log line gives every
later run the number, and 90 seconds is 4.5 times the slowest start seen.
If starts near 90 show up, that is the time to look at Chrome's own
logging, and the comment at the wait says so.

This is a longer timeout, which the item allowed only with measured times
showing the browser is slow and not stuck; the list above is that
measurement. No retry was added. The other waits keep their 20 seconds: no
run in the sample failed at one.

Out of scope and unchanged: the three runs of 2026-11-16 that failed with
`TypeError: fetch failed` at 0 seconds. They fail before the browser is
started and predate atelier-7tg6's fixture Desk.

### Validation

- `BROWSER=/usr/bin/google-chrome node t3-extension/run-tests.mjs`, with
  `atelier-desk` and `desk/` built from this branch: 3 of 3 pass, and the
  log has `Browser started in 0.2s`.
- `git diff --check`: no output.
