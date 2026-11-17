---
schema: atelier.review
schema_version: 3
round: 1
commit: 290988f7543005bf37df4df220a4c14ae78e5ecd
reviewers:
- thread:delegated-task:command%3Amcp%3Ad807e815-e857-4f1c-8031-33485690827e%3Adelegate-task%3A739eceb3-ade8-441d-87c8-43f4cc966438
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T16:31:57.796048253Z
---

## Summary

Complexity: low. The only code change raises one existing wait from 20 to
90 seconds and prints its elapsed time.

Value: medium. Avoiding false failures helps the extension's CI job, which
currently gates unrelated changes as well.

Risk of merging it: low. The change affects a test harness, and a wrong
assumption costs extra CI time and is easy to revert.

The small implementation is worth its cost if slow startup is established,
but the timeout-only exception in the Done when lines is not established
yet. The timing output meets the third line; the first two remain open.

I reviewed `git diff master...290988f7543005bf37df4df220a4c14ae78e5ecd`,
the startup and cleanup paths, `connectPage`, the test runner, and the CI
job. I independently read the two failed run logs and verified every one
of the Report's 17 first-PASS timings against the historical CI logs.
Those numbers and the quoted Chrome log timestamps match.

`BROWSER=/usr/bin/google-chrome node t3-extension/run-tests.mjs` passes all
three tests locally and prints `Browser started in 0.3s`.
`git diff --check master...HEAD` passes. PR #192's CI run 44126785937 also
passes all three extension tests and prints `Browser started in 1.1s`.
Neither validation reproduced a launch that exceeded the old limit.

## Finding 1: Establish that a slow CI launch reaches readiness before accepting the timeout-only fix
Recommendation: fix now

At `.atelier/items/atelier-4bxj/item.md:47-64` the Report concludes that the
failed launches were still starting rather than stuck and are the tail of
the successful runs' distribution. `t3-extension/test-extension.mjs:214-218`
uses that conclusion to justify the only behavioral fix, a 90-second wait.
The evidence establishes that Chrome had not opened its debugging port at
20 seconds, but does not establish that waiting longer would let either
launch finish.

In runs 43987730356 and 44123741376, the last Chrome messages are D-Bus
connection errors; neither log contains a DevTools listening message or a
successful debugging connection before cleanup kills Chrome. Messages
shortly before the cutoff do not distinguish a slow launch that will
finish from a launch that will remain blocked or fail. The other 17 values
measure time to the first extension assertion, including fixture setup and
extension initialization, and every one is below 20 seconds. They support
the hypothesis of slow startup but cannot establish recovery beyond the
old limit. The new branch's measured 1.1-second CI start does not test it
either.

The first two Done when lines explicitly require a cause supported by the
logs and allow a longer timeout alone only with CI measurements showing
the browser is merely slow. Capture a real slow CI launch through a
successful debugging connection, with its elapsed time and Chrome log, or
identify and fix what prevents readiness. Update the Report and comment
to separate the observation from the hypothesis. A locally delayed
browser wrapper would test the larger budget but would not establish the
cause of these CI failures.
