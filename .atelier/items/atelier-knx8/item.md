---
schema: atelier.item
schema_version: 3
id: atelier-knx8
title: Open the page for what a thread holds
status: changes_requested
mission: atelier-61gh
labels: []
priority: medium
blocked_by:
- atelier-prx5
- atelier-fnm5
implementers:
- thread:delegated-task:command%3Amcp%3Abedbfab3-88f2-4387-b923-ba6bd0a0f9b1%3Adelegate-task%3A9e810a46-c60e-44f1-8c75-1fb7fed292e8
ready:
  by: 'sumeet, in conversation on 2026-11-15: get that mission going'
  at: 2026-11-16T02:21:33.165230332Z
trivial: null
caused_by: null
created_at: 2026-11-16T02:20:38.348985349Z
---

## Goal

Open the embedded Desk on what the host's current thread is doing. Today the
panel opens on the repository page in every thread, so Sumeet has to find the
item an implementer's thread is working on himself.

## Done when

- [x] The contract gains one message from the host: the identity of the thread it is showing. `docs/architecture/desk.md` documents it with the rest.
- [x] On that message the embedded Desk chooses from the thread's current ties only, as atelier-prx5's read marks them: the item's page where the thread implements an item or reviews one, the mission's page where it leads, plans or reviews the plan of a mission, and the inbox where it has no current tie, including a thread tied only to finished work. With several current ties the order is the item it implements, then the item it reviews, then the mission.
- [x] The page names the thread's part in one line, such as "This thread implements this item".
- [x] A new message for another thread changes the page. A message for the same thread leaves the page the user has since navigated to alone.
- [x] The interface test in the fixture host page covers an implementer, a lead, a thread with no tie, a thread with a past tie to a done item and a current review (it shows the review's item), a thread with only past ties (the inbox), and the two rules above.
- [x] `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/src`, `desk/e2e`, `docs/architecture/desk.md`.

Out: showing live T3 Code state, such as a thread's elapsed time or its output; any thread list beyond what the pages show today.

## Context

The thread lookup is atelier-prx5's read. The embedded mode is atelier-vg1l's, and the contract and its fixture host page are atelier-fnm5's.

## Report

Open the Desk's panel in a thread and it now shows what that thread is
working on: the item for an implementer or a reviewer, the mission for a
lead, a planner or a plan reviewer, and the inbox otherwise.

- **The message** is `host:thread {thread}`, from the host, with the
  identity in the same form `desk:open-thread` sends. It is read under the
  same rules as `host:theme` (listed parent origin only). `host.ts` keeps
  the last identity and ignores a message that names it again, or that has
  no non-empty string. `docs/architecture/desk.md`, "The host contract",
  lists it in the table and gives the rules under "Following the thread".
- **The choice** is `place()` in `desk/src/lib/follow.ts`, a pure function
  over `GET /api/ties`: current ties only, in the order item it implements,
  item it reviews, mission it leads, plans, reviews the plan of. Nothing
  current (no ties, only done work, or only a decider's tie) gives the
  inbox. Unit tests in `follow.test.ts` cover each role, the order, past
  ties and the decider.
- **The navigation** is `useFollowThread` in `EmbeddedShell`
  (`desk/src/components/Shell.tsx`). It runs once per new identity, not on
  location changes, so a page the human moved to stays. It replaces the
  history entry instead of pushing one, because a framed page's history is
  the host window's, and the host's back button should not step through
  threads. A failed read of the ties leaves the page where it is, with no
  line. A late answer for an identity the host has since replaced is dropped.
- **The line** sits above the page, under the top bar (`.thread-part`):
  "This thread implements this item.", "reviews this item.", "leads this
  mission.", "plans this mission.", "reviews this mission's plan.", and on
  the inbox "This thread holds no open work." It shows only while the
  human is on the page it was chosen for.

### Validation

- `pnpm test`: 57 passed. `pnpm typecheck`, `pnpm build`.
- `pnpm e2e`: 106 passed on desktop and phone. `desk/e2e/host.spec.ts`
  frames the Desk in the fixture host page and sends `host:thread` for an
  implementer, a lead, a thread with no tie, a thread that implemented a
  done item and reviews another (it lands on the review's item), and a
  thread with only past ties (the inbox), checking the URL, the line and
  the `desk:navigated` the host hears. Another test moves to the inbox,
  resends the same thread (the page stays), then sends another thread (the
  page changes), then the first again (it changes back). A host on an
  unlisted origin cannot move the Desk. `/api/ties` answers from
  `ties` in `desk/e2e/fixtures.ts`. A thread whose ties answer 503 after
  a followed one leaves the page and removes the last line.
