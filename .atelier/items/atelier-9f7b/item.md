---
schema: atelier.item
schema_version: 3
id: atelier-9f7b
title: Put a thread waiting on an approval under Needs you on the mission page
status: building
mission: atelier-osel
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A57e793d0-a6ba-4a2b-9999-f8cb06fe0316%3Adelegate-task%3A52a6bc15-f78f-4837-b60d-c62ae23a4219
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

- [x] The Desk's mission and thread reads carry how many requests of the human a T3 thread has pending, taken from T3's `pendingRequestCount`.
- [x] The mission page lists a thread with a pending request under *Needs you*, with the button that opens the thread, and not under *At work*, whatever status T3 gives it. Its sentence counts that thread among what needs the human.
- [x] A check in `t3-extension` fails without the change: a fixture thread that is `running` with one pending request lands under *Needs you*. A Rust test covers the Desk passing the count through, and a T3 that omits the field still reads as zero.
- [x] Seen in the test instance (`node t3-extension/t3-instance.mjs start`, which already holds a thread waiting on an approval): a screenshot in the Report shows that thread under *Needs you*.

## Scope

In: `T3Thread` in `crates/atelier-desk/src/t3.rs` and what serialises it,
`liveWorkers` in `t3-extension/src/mission/model.ts` and its types, and
their tests and fixtures.

Out: This thread and the project home, which atelier-9l8h and atelier-wvst
draw; if either should show the same state, say so in the Report. The
Desk's own mission control page, which atelier-yy4j deletes.

## Report

A thread stopped on an approval now shows under *Needs you* on the mission
page, with *Open implementer*, and the sentence counts it ("One item and one
thread wait on you."). Seen in the test instance:
`artifacts/mission-approval-light.png` and `artifacts/mission-approval-dark.png`.

**For the lead: the mission read does not carry the count, because T3's list
does not give it.** `pendingRequestCount` is on `t3_thread_read`'s thread
(`OrchestratorMcpThreadDetail`), not on `t3_thread_list`'s items
(`OrchestratorMcpThreadListItem`). I checked T3's contract and a live list on
this host. The mission read's `ThreadMark` comes from the polled list, so
giving it the count would mean a `t3_thread_read` per live thread on every
10-second poll. The mission page already reads every listed thread in full
(`/api/thread?id=&full=1`, kept 20 seconds), so the count rides on that read
instead. The first Done when is met for the thread read and not for the
mission read; I ticked it on that basis. Defaulting a list field to 0 would
have said "no request pending" for every thread, which hides the case this
item is about.

**What changed.**

- `ThreadInfo` (`GET /api/thread`) has `pending_requests`: T3's
  `pendingRequestCount` from `t3_thread_read`, 0 where T3 omits it, `null`
  when a live thread is answered from the last poll without a read. A full
  read always reads the thread. `desk/src/api/ThreadInfo.ts` is regenerated
  and `docs/architecture/desk.md` documents the field.
- `liveWorkers` in `t3-extension/src/mission/model.ts` counts a thread as
  waiting when its full read has a pending request, whatever status T3 gives
  it. *Needs you*, *At work* and the sentence all follow from `waiting`, so
  nothing else in the page changed.
- The Desk's e2e fixture (`desk/e2e/fixtures.ts`) gives the field as the Desk
  does, so the Desk app typechecks.

**Checks.**

- `test-mission.mjs` has a new `approval` scenario: a thread `running` in the
  list and in its full read, with `pending_requests: 1`. It lands under *Needs
  you*, not *At work*, the sentence says "One thread waits on you." and its
  button opens the thread. With the `model.ts` change reverted, that check
  fails (`thread:thread:approval` expected under Needs you).
- `crates/atelier-desk/tests/threads.rs`:
  `a_full_read_says_how_many_requests_of_the_human_are_pending` reads a
  `running` thread with `pendingRequestCount: 1` and gets `pending_requests:
  1`. `any_thread_is_read_by_its_identity_live_or_ended` now checks that a
  read whose T3 omits the field gives 0 (the committed fixture capture has
  none), and that a live thread answered from the list gives `null`.
- `cargo nextest run -p atelier-desk` (67 passed), `cargo clippy -p
  atelier-desk --all-targets -- -D warnings`, `cargo fmt -- --check`, `pnpm
  typecheck` in `t3-extension` and `tsc --noEmit` in `desk`, `pnpm test` in
  `desk` (57 passed), and every `t3-extension` test `run-tests.mjs` runs
  (`test-extension.mjs` with `ATELIER_DESK` set to this branch's build).
- Test instance (`node t3-extension/t3-instance.mjs start`, T3 Code Nightly
  `v0.1.11-nightly.20261117.4851`, the Desk built from this branch): the
  waiting thread's `/api/thread?...&full=1` answered `"status":"running"`,
  `"pending_requests":1`. `__t3Custom.openMission('atelier', 'atelier-fm01')`
  listed it under *Needs you* with *Open implementer*, and *At work* was
  empty.

**Same gap elsewhere, outside this item.** The Desk's Inbox
(`crates/atelier-desk/src/server.rs:672`, waiting threads) and the sidebar's
count of missions waiting on a person (`crates/atelier-desk/src/server/views.rs:108`
and `:162`) test `status == "waiting"` on the polled list, so a thread on an
approval is missing from both. This thread page (atelier-9l8h) and the project
home (atelier-wvst) should show the same state; This thread can use
`pending_requests` from its full read as the mission page does. The fix at
the cause is for T3 to put `pendingRequestCount` on `t3_thread_list`'s items
(one line in `OrchestratorMcpService.ts` beside the detail's). The Desk could
then read it in its poll and drop the per-page logic. I recommend that as a
follow-up item.

**Environment note.** `/tmp` was over this user's quota during the work
(other threads' review clones and `atelier-extension-test-*` folders the
extension tests leave there), so command output to Claude Code's temp folder
failed. I ran everything with `TMPDIR` and `CARGO_TARGET_DIR` under my home
folder and deleted nothing of others'.

