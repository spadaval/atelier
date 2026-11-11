---
created_at: "2026-11-11T02:17:25.617313128+00:00"
id: "atelier-1ie1"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Write the Desk's API as a document first: each page's fields against the new core's views, and the views it lacks"
updated_at: "2026-11-11T02:17:25.617313128+00:00"
---

## Goal

Find out on paper whether the redesigned core can feed the Desk's pages, before any server is written. The new view layer (`next/crates/atelier-app/src/view.rs`) has three functions; the old site had views for a board, an inbox, a brief and a debrief. This item writes the Desk's API as a document and names every view the core lacks, so the next increment is sized on facts.

## Done when

- [ ] `docs/architecture/desk.md` has a route table for the mission, item, inbox, repositories and live-threads pages and the act endpoint, with each response's fields.
- [ ] For every field, the document names the `atelier-app` view or `atelier-records` reader in `next/` that gives it, or marks it missing. Each missing view is listed with what it must compute and a size on the three scales.
- [ ] The document says what the mission page shows in place of each part of the old brief and debrief (`crates/atelier-site` and `atelier brief`, `atelier debrief` on today's binary), and which record or act each comes from.
- [ ] The document lists the functions the Desk calls in `atelier-app` for the acts `ready` and `decide` and for opening a repository by path, with the result each returns, as atelier-psjw is to provide them, and marks which exist today.
- [ ] The document shows, with an example read from a converted clone, how a T3 thread is tied to an item from the thread IDs the records hold, and what is shown for an item with none.
- [ ] A scratch clone of this repository converted with the new converter exists outside any work tree, and the Report gives the commands that made it and `atelier check`'s result in it with the new binary.
- [ ] The Report lists each page of the spike (`crates/atelier-site/deck/src/pages` on branch `spike/mission-control`) as carried over, changed, or left for a later mission, with one line of why.

## Scope

A document and a scratch clone. No crate, no app code. The Health, Insights and digest pages of the spike are out of this mission; say so in the list and do not design them.

## Context

Strategy: `docs/strategies/desk.md`. Decisions: ADR 0032. Target design of the records and commands: `docs/strategies/redesign-product.md` and `redesign-architecture.md`. The T3 Code interface is described in the 2026-11-10 note on atelier-aab1.
