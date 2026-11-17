---
schema: atelier.review
schema_version: 3
round: 2
commit: 60e17dc4f6247d7101ec9b50b467cba6a25c6938
reviewers:
- thread:delegated-task:command%3Amcp%3Ad807e815-e857-4f1c-8031-33485690827e%3Adelegate-task%3A5c888c73-ae0e-4c75-9fba-d0819769f955
model: gpt-6.1-sol
verdict: approve
created_at: 2026-11-17T16:39:17.251230782Z
---

## Summary

Complexity: low. One existing startup wait changes from 20 to 120 seconds,
and one line prints its elapsed time.

Value: medium. The extension's CI job is in current use, and false failures
have already blocked merges.

Risk of merging it: low. A mistaken timeout costs CI time, affects no
production behavior, and is easy to revert.

The value justifies this small change. The new CI measurements satisfy the
exception permitting a longer wait for a browser shown to be slow. All
three Done when lines are met, and there are no open blocking findings.

I reviewed `git diff master...60e17dc4f6247d7101ec9b50b467cba6a25c6938`,
the changes since round 1, `until`, `connectPage`, process cleanup,
`run-tests.mjs`, and the extension workflow. The longer budget applies only
to browser startup; subsequent waits and assertions keep their limits.

I independently read the two original failed CI logs and the probe's
workflow at f844912c2e0af6051e525db7b5455a5e2c788e0f and all its recorded
results in run 44127411654. All 80 launches report `ready=yes` and have a
DevTools listening message. The 20 plain first-launch timings reproduce
the Report's table, including successful readiness at 22.612, 30.044,
32.875, and 46.976 seconds. Each launch uses a fresh profile. Plain second
launches take 0.151 to 0.394 seconds. Reading Chrome's top-level files
first takes 2.741 to 28.032 seconds and reduces the first launch to
0.353 to 6.119 seconds. The intervention supports the cold-file-read
explanation; the same D-Bus messages also occur in fast successful starts.

`BROWSER=/usr/bin/google-chrome node t3-extension/run-tests.mjs` passes all
three tests locally and prints `Browser started in 0.2s`.
`git diff --check master...HEAD` passes. Current PR #192 CI run 44127965653
passes all three extension tests and prints `Browser started in 9.5s`.

I did not reproduce a cold runner locally or trace individual disk reads.
The two original launches were killed at the old deadline, so their eventual
readiness cannot be recovered. The probe supplies the previously missing
direct CI observations of the same startup pattern reaching readiness
beyond that deadline; it does not guarantee that every future launch will
finish within 120 seconds.

## Finding 1: Establish that a slow CI launch reaches readiness before accepting the timeout-only fix
Recommendation: fix now
Was: 1-merp.1
Response: fixed in f44174304. The finding was right: the first report inferred that a slow start would finish and had no run showing it. The probe above shows four starts of 22.6 to 47.0 seconds that became ready, and what the time goes to. The Report and the comment at the wait now give the measurement, and the wait is 120 seconds on its strength.
Now: fixed

The new probe directly measures four successful first launches beyond the
old 20-second limit, with their Chrome logs and readiness response. It
also compares subsequent launches and file pre-reading on fresh runners.
This supplies the evidence the finding requested and justifies the
startup-only timeout increase under the item's Done when exception.

## Finding 2: The two observed failure rates are not equal
Recommendation: no action

At `.atelier/items/atelier-4bxj/item.md:70-72`, the Report calls four slow
starts out of 20 the rate of the historical two failures out of 19. Those
are 20% and about 10.5%, respectively. They are two small samples consistent
with intermittent slow startup, rather than an exact rate match. This is
a wording note, not a merge blocker: the measured successful launches
beyond 20 seconds independently justify the fix.
