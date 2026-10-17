---
created_at: "2026-10-17T17:25:59.393637922+00:00"
id: "atelier-rx22"
issue_type: "feature"
labels:
- "review"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-rx22
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fip8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ho7o"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-r6bt"
    type: "advances"
  - kind: "issue"
    id: "atelier-sfsq"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T19:40:36.752829447+00:00"
status: "done"
title: "Review a finished mission with atelier debrief"
updated_at: "2026-10-17T19:40:36.752829447+00:00"
---

## Goal

S4 of docs/strategies/review-and-introspection.md: `atelier debrief <mission>` reviews a finished mission from its records and Git history, as dense Markdown for agents and a self-contained HTML page for humans, both rendered from one view model, so a human can make post-mission calls from one page and an agent can answer "why is it like this?".

## Done when

- `atelier debrief <mission>` prints dense Markdown (ADR 0022) with: the mission's outcome, strategy, generating commit and start; each success criterion with its mark, deciding receipt and age (atelier-ho7o's proof lookup); each scoped item with its implementer report (Outcome and parts, or the Close Reason alone for older records); the decision log, one-way first, then decision records, reversible notes, superseded records; derived deviations: scope added after the mission started, scope dropped (superseded items with their reason), deferred findings with their follow-up, non-pass evidence still newest on an item, trivial exemptions, and items reopened after a done claim; review events on the mission itself (plan approval, S5); and follow-ups (open items in scope and defer follow-ups).
- `atelier debrief <mission> --html` writes one self-contained page under `.atelier/runtime/reviews/` (inline CSS with light and dark palettes, no external resources, every record string escaped, no scripts or forms), prints its path, and writes nothing else. The page leads with a summary and uses tables and collapsible sections.
- Derived facts come from one Git record-history query over the mission's issue records (its walks scale with those records, not with review events); nothing new is stored.
- Fixture tests check both forms, and one test checks that every item ID, criterion slug and mark, and decision choice in the Markdown appears in the HTML. A run against this repository's done mission atelier-kgcr succeeds.
- docs/product (a new mission-reviews.md, cli-surface.md, human-cli-output.md quiet token) and PRODUCT_INTENT.md (a sentence that static HTML review pages are documents, not a second UI) describe it.

## Unknowns

How proof that predates later code is presented (atelier-ho7o's open item): the strategy forbids presenting a receipt as passing without checking freshness. Default to test: show each mark with its age, and list proven criteria whose proof predates later code commits under deviations.

## Design

The view model lives in a new atelier-app module (`mission_review`), with one builder per review kind; the CLI renders it twice. The record-history query reads, with one `git log` over the scoped record paths and the batched object reader, each version of each scoped issue record, oldest first. Mission start is the first version whose status is not `draft` (or its creation). An item was added after the start when the first version linking it to the mission is newer. An item reopened when a done version is followed by a version that is not done.

## Notes

- 2026-10-17 [decision] Count a debrief criterion's proof age in code commits up to the mission's close, and list proven criteria with later code as aged-proof deviations
  Reason: Counting to HEAD would flag every proof of a finished mission once unrelated work lands; the close bounds what the mission's own code could have changed, and listing aged proof keeps the strategy's no-unchecked-pass rule visible
  Rejected: count to HEAD
  Rejected: apply item-level evidence freshness
- 2026-10-17 [decision] Write the HTML page to .atelier/runtime/reviews/<mission>-debrief.html and print only its path
  Reason: The strategy keeps pages in the ignored runtime directory and has T3 Code open them; a fixed name per mission lets a regenerated page replace the old one
- 2026-10-17 [decision] Date an item's joining from the earlier of the first item version and the first mission version holding the advances link
  Reason: Older records stored the link on the mission's side and a later change moved it to the item, which made kgcr's items look added after the mission closed

## Close Reason

`atelier debrief <mission>` reviews a mission from its records and Git history, as dense Markdown and as a self-contained HTML page from one view model: criteria with proof and its age, each item's implementer report, the decision log, derived deviations, mission review events and follow-ups.

### Deviations
Review atelier-gq88 and atelier-yx83 changed the history query from a date-ordered walk to HEAD's first-parent history with merges read as their change to the base, and added reading of retired epic records; without them the real mission kgcr missed seven deviations and merged branch work looked reopened.

### Decisions
See this item's decision notes: proof aged to the mission's close, the page's fixed path, and joining dated from either record.

### Surprises
The repository's own history had moved link storage and retired the epic shape, so a debrief of an old mission must read records as they were then. The page could not be viewed in a browser here; the preview sandbox is blocked on this host.

### Open items
- atelier-lfs7: the human's review of a real HTML debrief (S7).
