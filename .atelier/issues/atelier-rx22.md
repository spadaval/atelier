---
created_at: "2026-10-17T17:25:59.393637922+00:00"
id: "atelier-rx22"
issue_type: "feature"
labels:
- "review"
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
status: "backlog"
title: "Review a finished mission with atelier debrief"
updated_at: "2026-10-17T18:53:19.567771438+00:00"
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
