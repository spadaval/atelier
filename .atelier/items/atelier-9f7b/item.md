---
schema: atelier.item
schema_version: 3
id: atelier-9f7b
title: Put a thread waiting on an approval under Needs you on the mission page
status: ready
mission: atelier-osel
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: lead of atelier-osel, from deferred finding atelier-5cqz/1-uejn.1
  at: 2026-11-18T00:47:42.106818405Z
trivial: null
caused_by: null
created_at: 2026-11-18T00:47:32.568269286Z
---

## Goal

The mission page exists to say what needs the human. A thread stopped on an
approval is exactly that, and the page lists it under *At work*, so the page
can say "Nothing needs you" while a thread waits. The mission's
needs-you-first criterion asks for a row for each thread that needs the
human.

Taken from deferred finding atelier-5cqz/1-uejn.1, in .atelier/items/atelier-5cqz/reviews/1-uejn.md:

> Finding 1: Approval-waiting threads appear under At work instead of Needs you
> Recommendation: defer
>
> The Report's limitation is real. The fixture waits for `pendingRequestCount > 0` in `t3-extension/t3-instance.mjs:302-305`, but the mission API reports that thread as running and the mission page places it under At work. `T3Thread` in `crates/atelier-desk/src/t3.rs:60-72` discards pendingRequestCount, and `liveWorkers` in `t3-extension/src/mission/model.ts:94` recognises waiting only from a status string. The committed running screenshot and the independently started instance both show the mismatch.
>
> Carry T3's pending-request state through the Desk and use it when classifying workers in a follow-up. This predates the fixture change, is already disclosed in the Report, and needs Desk and extension source changes explicitly excluded from this item. It does not block the requested fixture and connection improvements.

## Done when

- [ ] The Desk's mission and thread reads carry how many requests of the human a T3 thread has pending, taken from T3's `pendingRequestCount`.
- [ ] The mission page lists a thread with a pending request under *Needs you*, with the button that opens the thread, and not under *At work*, whatever status T3 gives it. Its sentence counts that thread among what needs the human.
- [ ] A check in `t3-extension` fails without the change: a fixture thread that is `running` with one pending request lands under *Needs you*. A Rust test covers the Desk passing the count through, and a T3 that omits the field still reads as zero.
- [ ] Seen in the test instance (`node t3-extension/t3-instance.mjs start`, which already holds a thread waiting on an approval): a screenshot in the Report shows that thread under *Needs you*.

## Scope

In: `T3Thread` in `crates/atelier-desk/src/t3.rs` and what serialises it,
`liveWorkers` in `t3-extension/src/mission/model.ts` and its types, and
their tests and fixtures.

Out: This thread and the project home, which atelier-9l8h and atelier-wvst
draw; if either should show the same state, say so in the Report. The
Desk's own mission control page, which atelier-yy4j deletes.
