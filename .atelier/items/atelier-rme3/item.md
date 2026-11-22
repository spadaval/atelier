---
schema: atelier.item
schema_version: 3
id: atelier-rme3
title: Show open items in the Desk and the T3 extension with no draft or ready, and one backlog list for loose items
status: building
mission: atelier-dh7a
labels: []
priority: medium
blocked_by:
- atelier-v06d
implementers:
- thread:delegated-task:command%3Amcp%3A134a3ba0-c5f0-4828-a968-a4cfb219e451%3Adelegate-task%3Af9b8e4e8-be66-415b-9cab-18cee314ff97
ready: null
trivial: null
caused_by: null
created_at: 2026-11-19T15:25:03.105897690Z
---

## Goal

The Desk and the T3 extension sort unstarted items by `draft` and `ready`:
the Desk's project page splits loose items into next and later, the
extension's home page puts ready items in "Next" and drafts in "Later", and
its mission page counts drafts apart from items that can start. Once
atelier-v06d merges every unstarted item is `open`, and these pages should
say so. The human chose on 2026-11-19 to give up the next and later split
for loose items, which was the ready mark.

## Done when

- [ ] No page of the Desk's app or the extension shows the word draft or
  ready as an item's status, and `Status` in `desk/src/api` has `open` in
  place of both.
- [ ] Loose open items are one list on the Desk's project page and on the
  extension's home page, ordered by priority and then ID.
- [ ] A mission's page says of each unstarted item whether it can start or
  what it waits on, and counts no drafts.
- [ ] A proposed mission and an agreed one still look different, by the
  mission's own state and not by an item status borrowed for it
  (`desk/src/lib/words.ts` maps a proposed mission to `draft` today).
- [ ] `pnpm test` and `pnpm e2e` pass in `desk/`, and `node
  t3-extension/run-tests.mjs` passes against a Desk built from this branch.
- [ ] The Report holds a screenshot of the project page and of a mission
  page with unstarted items, from the Desk and from the extension.

## Scope

`desk/src`, `t3-extension/src`, their tests, and `docs/product/desk.md` and
`docs/architecture/desk.md` where they describe these pages. No new page,
lane, filter or control. Not the Desk's server, which atelier-v06d changes.
