---
schema: atelier.item
schema_version: 3
id: atelier-cdxg
title: 'Bring the T3 views into the Desk: this thread, mission control, the project home, and the reads for the sidebar and the project strip'
status: changes_requested
mission: null
labels: []
priority: medium
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A9c525daa-e7f7-44f4-9529-6989140fcc76%3Adelegate-task%3A9e564324-a27d-4e91-9392-859d25fac7ad
ready: null
trivial: null
caused_by: null
created_at: 2026-11-16T15:32:52.249898090Z
---

## Goal

Sumeet settled, over three rounds of a spike on 2026-11-16, on views of the
Desk that T3 Code shows beside its threads: "This thread" in the panel,
mission control and the project home over the main area, the inbox laid out
for a wide frame, and the reads T3 Code's sidebar and project strip draw
from. He asked for them merged. The spike (`spike/t3-home`) never merges, so
this item brings its code onto a branch and reworks it to this repository's
standard, with the look and behaviour he approved, except three changes from
the last round: mission control keeps its four panes down to 860 pixels,
leaves 44 pixels clear at the top right for the host's close button, and the
project home puts under Now any mission with an item started or done.

The T3 extension, in another repository, is already built against the
spike's routes, message names and field names, so those stay exactly as
they are: added to, never renamed or removed.

## Done when

- [x] `/embed/t`, `/embed/mission/<repo>/<id>`, `/embed/home/<repo>` and the wide `/embed/inbox` draw as the spike drew them, and every `host:thread` naming another thread opens `/t`, reading `{thread, repository?, children?, mentioned?}`.
- [x] `GET /api/sidebar`, `/api/launch?repo=`, `/api/thread?id=` (with `full=1`), `/api/history`, `/api/roadmap` and `/api/checks` answer with the spike's routes and fields, and `pull_request` on an item, `changed_at` and `ready` on `/api/motion` are given; nothing the spike named is renamed or removed.
- [x] The Desk sends `desk:resized {height}` and `desk:key {key}`.
- [x] The types of every new read are generated with `ts-rs`, and `desk/src/api/spike.ts` is gone.
- [x] Mission control keeps its four panes down to 860 pixels wide and stacks below; its top bar leaves 44 pixels clear at the top right; the project home puts a mission with any item started or done under Now.
- [x] Every read that runs `git`, `gh` or `t3_thread_read` has a deadline and a cache, fails soft so the page still draws, and puts no value from a request on a command line unchecked.
- [x] Code the old `host:thread` behaviour left dead (`follow.ts`) and what the spike left unused (round two's mission control styles, the old home page) are removed.
- [x] Server tests cover each new read; interface tests cover each page at the widths it is meant for, mission control's keys, and each new message in the fixture host page.
- [x] `docs/product/desk.md` gains the three views and where T3 Code shows each; `docs/architecture/desk.md` documents every new route, field and message, rewrites "The host contract" and "Following the thread", and says what is approximate.
- [x] `pnpm test`, `pnpm e2e`, `cargo nextest run -p atelier-desk`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt -- --check` pass.

## Scope

In: `desk/`, `crates/atelier-desk`, `docs/product/desk.md` and
`docs/architecture/desk.md`.

Out: the T3 extension; any write or action; the real Desk service's
configuration, which the planner's thread reinstalls after the merge; and
the spike's own instance on port 3781.

## Report

The views Sumeet approved in the spike are now on this branch, built to
merge: the same pages and the same JSON, with generated types, bounded
reads, tests and documents. Against the spike's server on the live
`atelier` records, `/api/sidebar`, `/api/launch`, `/api/history`,
`/api/roadmap`, `/api/motion` and `/api/thread?full=1` give the same JSON
(only clock seconds differ, and `pull_request`, which needs the forge list a
non-fetching test server never reads). Screenshots of mission control at
900 and 860 pixels and the project home at 1300 match his round three shots.

What changed from the spike:

- **Server.** The reads moved from `server/spike.rs` to `server/views.rs`,
  their types to `api.rs` with `ts-rs` (enums for the spike's string
  states, so the JSON is unchanged). Commands run through `bounded::run`,
  which kills one past its deadline: `git log` 15 seconds, `gh pr view` 15,
  `git for-each-ref` 10. History (`history.rs`) is read once per change of
  the repository's commits through the Desk's own cache, not once per
  request, and filtered by `days` and `ids` in memory; it sorts by parsed
  time, since sorting the text put commits with other offsets out of order.
  `changed_at` comes from one `git for-each-ref` per request instead of a
  `git log` per item. Ages are computed with `chrono`, not by running
  `date` per item. The thread cache is bounded at 500 entries and drops
  stale ones; `/api/thread` refuses an identity that is empty, longer than
  512 characters or holds a control character. `/api/checks` keeps at most
  200 pull requests.
- **A bug the spike showed.** Through the Desk's MCP client, which has no
  thread of its own, `t3_thread_read` returns a thread's *first* items, so
  mission control's Active Threads showed each thread's opening tool call
  (visible in round three's screenshot as `cat .../agent-factory...`). The
  full read now passes `beforePosition` past the end, and the line is the
  thread's latest. I checked both against T3 Code with a read-only token.
- **Name in the code.** The progress log hard-coded `"sumeet"` to find the
  human's ready marks. `/api/repos` now gives `caller`, the configured name,
  and the log uses it; the text still reads "Sumeet readied ...".
- **Following the thread.** The spike opened `/t` again on any change to
  the host's message, so a new child or mention pulled the human back from
  a page he had moved to. Now only another thread opens `/t`; a change for
  the same thread updates `/t` where it is shown. This keeps the rule the
  contract had before ("the same thread again leaves the page alone").
- **This thread** draws when its ties or the inbox cannot be read: a banner
  names the failure, and it no longer claims the thread holds nothing.
- **Mission control** stacks below 860 pixels (was 1000), leaves 44 pixels
  clear at the top right, and wraps its top bar when stacked, which
  overflowed by 220 pixels at 420 wide.
- **Project home** footer read "Fetched now ago."; it reads "Fetched now."
  or "Fetched 2m ago.".
- **Removed:** `desk/src/api/spike.ts`, `lib/follow.ts` and its test, the
  unused `HomePage`, `api.sidebar`/`api.launch` path helpers the app never
  called, round two's `.mc-*` styles and `.fold-done`. `pages/Home.tsx` became
  `components/Glance.tsx` (the shared rows and the project glance) and
  `pages/Roadmap.tsx` became `pages/ProjectHome.tsx`; the history logic
  moved to `lib/mission.ts` with unit tests.

Tests: `tests/views.rs` covers sidebar, launch, history, checks (with a `gh`
stand-in, its cache and a non-number `pr`), roadmap, motion's new fields and
the identity checks; `tests/threads.rs` covers `/api/thread` live, ended,
full and unknown against the T3 stand-in, from a trimmed real
`t3_thread_read` answer. `e2e/host.spec.ts` covers `host:thread` opening
`/t`, children and mentions, the same-thread rule, unreadable ties, `/t` at
420 and 720, `desk:resized`, `desk:key`, and mission control's keys;
`e2e/views.spec.ts` covers mission control at 1100, 860, 859 and 420, the
project home at 1300 and 880, and the inbox at 1300 and 720.

Where T3 Code shows each view is from the spike's code and screenshots: `/t`
in the right panel, the others over the main area. The planner's thread
should correct `docs/product/desk.md` if the extension places them
elsewhere.

`atelier-core` has a real history of actions (`log.rs`, read from record
changes). The progress log could read it instead of commit subjects and stop
being approximate; I kept subjects, as the brief asks, and left that for a
later item.
