---
schema: atelier.review
schema_version: 3
round: 1
commit: 2194f67e5bea0e00075d7eb47468efd3d67aae14
reviewers:
- thread:delegated-task:command%3Amcp%3Abc79e4d7-ee86-4b64-b83e-5f8183bbf445%3Adelegate-task%3A3083f938-b1f7-4a40-a990-7ced2f3bf162
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-27T08:41:36.130194381Z
---

## Summary

Complexity: medium. The change adds Git-history aggregation as well as
reordering and shortening several status sections.

Value: high. A lead needs its pending work to survive the harness's output
limit, and the review counts make the cost of its mission visible.

Risk of merging: low. The changes affect read-only output and are easy to
revert, although an ordering mistake can hide work from the reader.

The value is worth the size of the change. I read the full diff against
master, checked the Done when lines and both documents, and ran the 404
CLI/core tests successfully, including the new size and ordering tests.
`cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, diff
whitespace checks and `evals/agent-factory/test.sh` all pass. I also independently read the
Hub's review files and each file's verdict diffs at the reported checkout
078774ac: 22 rounds for 11 items, the same sorted 22 durations, and six
new later-round findings, none fix-now. The new binary gives the reported
15m median, 31m maximum and 1,564-byte live status there.

I additionally built a scratch repository with a lead, a startable mission
item, and unrelated dropped items whose old branches remain, and another
case with a needs-human item in the led mission. These expose the two
findings below. I did not inspect the pull request's hosted CI result.

## Finding 1: Critical: Unrelated completed drops still precede the lead's pending work
Recommendation: fix now

The first Done when line says that what waits on the reader prints before
anything that does not. In `crates/atelier-cli/src/output.rs:244-256`, the
new primary section prints every entry in `dropped` and `unreached` before
calling `lead_waits`. `commands.rs` gathers those entries from all items,
without restricting them to this caller. In particular, the first block
explicitly says that nothing is left to do on its items, yet it precedes
the lead's handoff that does need action.

I reproduced this at `/tmp/f0sn-review-a76m5jog` with one mission led by
lead1, one startable item demo-a1aa, and 40 unrelated items dropped on main
while their old building branches remain. Every dropped item has a normal
Dropped reason. `atelier check` accepts every record. Running `status` as
lead1 puts the dropped block at byte 166, the pending
`Run atelier start demo-a1aa` at byte 3,987, and the mission at byte 4,167.
The first 2 KB contain no handoff. Thus the new ordering still permits
unrelated context to hide the lead's work, the failure this item addresses.

Put notices that do not wait on this caller after its pending work, and
add a test that mixes a lead's handoff with unrelated dropped branches.
The existing size fixtures contain no such notices and cannot catch this.

## Finding 2: Useful: Led needs-human items are repeated in the backlog
Recommendation: defer

The backlog filter in `crates/atelier-cli/src/output.rs:260-269` removes
led items from startable, cannot-start and started, but
`crates/atelier-cli/src/view_output.rs:637-640` prints `overview.waiting`
without the equivalent mission filter. A needs-human item in a led
mission therefore appears once under `You lead mission` and again under
the backlog's `Waiting on a human` heading. I confirmed this with
demo-h1um in the scratch repository above. This duplication existed before
the change, so it does not block approval of the rest, but it misses the
new report's stated exclusion of led items from the backlog and spends
the limited output budget twice. Apply the led-mission filter to waiting
items too, omitting the heading if none remain.
