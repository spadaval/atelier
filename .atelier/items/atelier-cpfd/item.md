---
schema: atelier.item
schema_version: 3
id: atelier-cpfd
title: Show what a later review round cost in the pace line, from one verdict to the next, not only the reviewer's time
status: in_review
mission: atelier-oqsz
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3Aad94841c-cb71-4522-99ce-e39e71555f54%3Adelegate-task%3Aoqsz-impl-cpfd
ready: null
trivial: null
caused_by: null
created_at: 2026-11-27T08:59:58.479123042Z
---

## Goal

`atelier-f0sn` gave a lead's `status` a line of review pace. Its time is
the reviewer's alone, from a review's `created_at` to its verdict. On
`hub-v2-bg14` that reads "a round took 15m at the median, 31m at most",
while a second round cost the mission 33 to 86 minutes from the first
verdict to the second, and found nothing to fix on any item. The human
asked for the line because "your last five reviews took over an hour each
and caught nothing" would have changed behaviour. With the reviewer's time
alone that message would not have appeared on the night that prompted it.
The lead ruled on 2026-11-27 that the line counts what a later round cost.

## Done when

- [x] For rounds after the first, the pace line gives the time from the
  previous round's verdict to this round's verdict. It still fits on one
  line and says plainly what each time measures.
- [x] The Report gives the line for `hub-v2-bg14` beside the 33 to 86
  minutes counted by hand in `atelier-f0sn`'s Report.
- [x] The two size tests of `atelier-f0sn` still pass, and
  `docs/product.md` shows the new line.

## Scope

`atelier_core::pace`, where `status` prints the line, their tests and the
line in `docs/product.md`. Nothing else in `status`.

## Report

A lead's pace line now says what a later round cost the mission, not only
what the reviewer spent. On `hub-v2-bg14` it reads 55 minutes at the
median from one verdict to the next, where the old line said 15. Code in
`457edca19`.

For `hub-v2-bg14`, on a scratch clone of its v2 checkout at `078774ac`, as
its lead:

    Review pace: 22 rounds for 11 items, 2.0 each; a reviewer took 15m a round at the median, 31m at most; from one verdict to the next took 55m at the median, 1h 22m at most; later rounds found 6 new findings, 0 to fix now.

`atelier-f0sn`'s Report counted 33 to 86 minutes from round 1 to round 2
by hand. My hand count, one `git log -1 -G"^verdict: [ac]"` per review
file, gives 11 later rounds, in minutes: 29 33 41 42 42 55 59 64 67 81 82.
The 6th is 55 and the longest 82, as the tool says. The 29 is `hub-v2-9xrx`
round 2 to round 3, which `atelier-f0sn` left out by counting only round 2.
The 82 against its 86 is `hub-v2-7fyn`; its Report does not say how it
timed that one, and I did not chase the four minutes.

How it counts: for each round after the first whose verdict commit was
found, the time runs from the latest verdict commit of any earlier round of
the same item to this round's. So it holds the implementer's fixing and the
wait for a reviewer as well as the review. Taking the latest of all earlier
rounds, not round n-1 by number, keeps a split round with two review files
counted once from its later verdict. The reviewer's own time stays in the
line, renamed from "a round took" to "a reviewer took … a round" so the
two times say what each measures.

### Validation

- `cargo nextest run -p atelier-cli -p atelier-core`: 411 passed. Both size
  tests of `atelier-f0sn` pass, the twelve-item mission under 2 KB and the
  Hub's state under 4 KB, with the longer line. The pace test now also expects 35m from
  `demo-d001`'s first verdict (10:40) to its second (11:15).
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings` and
  `git diff --check` pass.
- `docs/product.md` shows the new line and says what each time measures.

