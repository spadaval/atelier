---
schema: atelier.item
schema_version: 3
id: atelier-ic22
title: Add a Cmd-K finder to the Desk, so any mission or item is one keystroke and a few letters away
status: draft
mission: null
labels:
- feature
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T04:52:37.912217282Z
---

## Goal

Give the Desk one place to find anything. Sumeet asked for it on 2026-11-12, when he went looking for mission atelier-g961 and had no way to search for it. Cmd-K (Ctrl-K off a Mac) opens a combobox over any page; typing filters missions and items across all the repositories the Desk serves, by ID and by title, and Enter opens the chosen one.

## Done when

- [ ] Cmd-K or Ctrl-K on any Desk page opens the finder, and Escape closes it and returns focus to where it was.
- [ ] Typing part of an ID (`g961`) or of a title finds the mission or item in any served repository, with its repository, kind and status shown in the row; done and dropped records are found too, ranked below open ones.
- [ ] Arrow keys move through the results and Enter opens the record's page; the whole flow works without a pointer, and a screen reader announces it as a combobox.
- [ ] It is reachable on a phone, where there is no Cmd-K: a search control in the page header opens the same finder.
- [ ] An e2e test opens the finder, types an ID from the fixtures and lands on that record's page.

## Scope

In: the Desk's app under `desk/`, and whatever read of the records the finder needs through the existing API.

Out: searching record bodies, reviews or transcripts; running commands or acts from the finder; anything under `crates/` beyond a read the API lacks.

## Context

The Desk's product and architecture notes are `docs/product/desk.md` and `docs/architecture/desk.md`. Filed for later: it is a draft with no go-ahead.
