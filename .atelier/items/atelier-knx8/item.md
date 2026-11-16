---
schema: atelier.item
schema_version: 3
id: atelier-knx8
title: Open the page for what a thread holds
status: building
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

- [ ] The contract gains one message from the host: the identity of the thread it is showing. `docs/architecture/desk.md` documents it with the rest.
- [ ] On that message the embedded Desk chooses from the thread's current ties only, as atelier-prx5's read marks them: the item's page where the thread implements an item or reviews one, the mission's page where it leads, plans or reviews the plan of a mission, and the inbox where it has no current tie, including a thread tied only to finished work. With several current ties the order is the item it implements, then the item it reviews, then the mission.
- [ ] The page names the thread's part in one line, such as "This thread implements this item".
- [ ] A new message for another thread changes the page. A message for the same thread leaves the page the user has since navigated to alone.
- [ ] The interface test in the fixture host page covers an implementer, a lead, a thread with no tie, a thread with a past tie to a done item and a current review (it shows the review's item), a thread with only past ties (the inbox), and the two rules above.
- [ ] `pnpm test` and `pnpm e2e` pass.

## Scope

In: `desk/src`, `desk/e2e`, `docs/architecture/desk.md`.

Out: showing live T3 Code state, such as a thread's elapsed time or its output; any thread list beyond what the pages show today.

## Context

The thread lookup is atelier-prx5's read. The embedded mode is atelier-vg1l's, and the contract and its fixture host page are atelier-fnm5's.
