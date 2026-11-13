---
schema: atelier.item
schema_version: 3
id: atelier-bbia
title: 'Show a gardener''s run on the Desk: its numbers, its proposals and whether it waits for a session'
status: draft
mission: null
labels:
- feature
priority: medium
blocked_by:
- atelier-v958
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-13T22:36:10.883993719Z
---

## Goal

Let Sumeet see a gardener's run where he sees everything else. On 2026-11-13 two runs, atelier-0019 and atelier-qb4f, were approved and waiting for their session with him, and the Desk showed both as plain ready items: their reports live on their branches, nothing marks them as his, and the Desk has no page for a run. He found out from `atelier list` in a conversation. Tending is the Desk's second job, and a run's page is what he tends from.

## Done when

- [ ] An item labelled `garden` has a page that shows its Report as the gardener wrote it: the measures table against the last run and the baseline, the worst spots, each proposal with its cause, evidence and change, and what was fixed and settled. It is read from the item's branch while the run has not merged.
- [ ] The page says where the run stands in words: running, in review, approved and waiting for its session, or merged with what the Session section records.
- [ ] A run that is approved with an empty Session section is listed in the inbox as waiting on Sumeet, in every repository, with a link to the gardener's thread where T3 Code knows it.
- [ ] A repository's page links its latest run and says when it was made.
- [ ] The page offers no act: the session happens in the gardener's thread.
- [ ] An e2e test finds the page and the inbox line for a fixture run at a desktop and a phone width, and `docs/product/desk.md` and `docs/architecture/desk.md` describe the page.

## Scope

In: `desk/`, `crates/atelier-desk`, a read in the core if the Report's sections are not given already, and the Desk's two documents.

Out: trends across runs, the daily feed and the host's database, which are the Desk's health work; any change to the gardener's procedure or its Report's format; starting a run from the Desk.

## Context

`skills/agent-factory/procedures/garden.md` gives the Report's sections. `docs/architecture/desk.md`, "Spike pages left for a later mission", names the spike's digest view (`Digest.tsx`) as the earlier attempt. It waits on atelier-v958, which reads items from their branches and rewrites the inbox. atelier-e7iy asks whether `status` should say an approved run waits for its session; the inbox line here is the Desk's side of the same gap.
