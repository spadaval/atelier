---
schema: atelier.item
schema_version: 3
id: atelier-4bxj
title: Stop the extension's CI job failing when the browser is slow to start
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
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

- [ ] The Report says why the browser's debugging port is not answering when the test gives up: the cause, from a failing run's log, not a guess.
- [ ] The fix is at that cause. A longer timeout or a retry alone is not one unless the Report shows the browser is only slow, with the times measured in CI.
- [ ] The job passes 20 runs in a row on one commit.

## Scope

In: `t3-extension/test-extension.mjs`, `t3-extension/run-tests.mjs` and the
`extension` job in `.github/workflows/ci.yml`.

Out: run 43950803020's `TypeError: fetch failed` at 0 seconds, unless it
turns out to share the cause.
