---
schema: atelier.review
schema_version: 3
round: 1
commit: 8169e44f40df0855ff2d4c52c4106f24c4805604
reviewers:
- thread:delegated-task:command%3Amcp%3A0950c503-e754-4673-a60a-b22e87c91dff%3Adelegate-task%3Aaac59384-65ea-4a6d-804a-591d9c9c7ca0
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-11T17:27:40.356953063Z
---

## Summary

Complexity: high. The change adds a history API that reconstructs acts across a Git graph, including merges, thread take-overs and invalid record versions.

Value: high. The gardener, scorecard scripts and Desk need this shared history of acts.

Risk of merging: medium. This is a read-only command and API, but incorrect reconstructed acts reach their consumers and can distort the history they use.

The value warrants the implementation. Both recovery defects below need fixing before merge.

I reviewed the diff from master through 8169e44f40df0855ff2d4c52c4106f24c4805604 against every Done when line, the act table and the Report. I read the walker, CLI formatting, shared commit reader, scenario tests and documentation. `cargo nextest run -p atelier-cli` passed all 170 tests, including the eleven log scenarios. `cargo nextest run -p atelier-app -p atelier-records` passed all 97 tests. Formatting and diff whitespace checks passed.

In astromech, read only, five runs of the proposed debug binary printed 88 lines without warnings in 0.1035 to 0.1082 seconds. An exec trace confirmed six Git processes: two rev-parse calls, symbolic-ref, for-each-ref, log and cat-file. The app library test covers reading the data through `Repo::open_committed`; I did not run the Desk, which is outside this item's scope. GitHub's Rust CI job was still pending when checked, so I do not claim CI passed.

I also drove the proposed binary in temporary Git repositories, without changing the implementation, to reproduce both findings below. The small `atelier-records` addition is a reasonable dependency for sharing the one batch reader.

## Finding 1: Pass over historical encoding errors instead of aborting the history
Recommendation: fix now

In `crates/atelier-app/src/log.rs:612-616`, `reader().blob(...)?` propagates a `ReadError::Record` before the pass-over handling can see it. The new blob reader returns that error for non-UTF-8 content (`crates/atelier-records/src/at_commit.rs:236-238`). A bad encoding in any historical record therefore makes the entire history unavailable, including after the record has been repaired. The documented contract says an invalid version is passed over, with a warning, and the valid versions on either side are compared.

Reproduction: commit a valid draft item; commit the same file with byte 0xff appended; restore valid UTF-8 and set the status to ready; run `atelier log`. The binary exits 2, prints no acts, and reports `the file is not UTF-8`. `atelier log demo-ab12` fails the same way. It should warn about the broken commit and print the repaired commit's `draft -> ready` act. Route record-content read errors through the existing pass-over path, while preserving failures for actual Git I/O problems, and add a scenario for this sequence.

## Finding 2: Keep an invalid version's predecessor specific to its occurrence
Recommendation: fix now

In `crates/atelier-app/src/log.rs:609-610` and `632-642`, the cache keyed only by `(path, blob)` stores `Read::PassedOver(before)`. The parse failure belongs to the blob, but `before` belongs to the particular occurrence in the history. When the same malformed bytes recur after a different valid state, the cache returns the first occurrence's predecessor and compares later repairs against the wrong state. This also applies when the same bad blob occurs on different branches.

Reproduction in a linear scratch history: commit the statuses `draft`, `bogus`, `ready`, `bogus`, `ready`, keeping the rest of the item identical. Both `bogus` commits have the same blob. The proposed binary prints two `ready - draft -> ready` acts, one at each repair commit. Passing over the invalid versions should compare the valid sequence `draft`, `ready`, `ready` and print only the first ready act. Instead, the second repair reuses the first bad version's cached draft predecessor and invents an act. Cache parsing independently from the last readable version on each history path, and add a regression scenario for repeated malformed blobs.
