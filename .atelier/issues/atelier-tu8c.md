---
created_at: "2026-11-09T00:45:57.947764486+00:00"
id: "atelier-tu8c"
issue_type: "task"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Record ADR 0030: Atelier may have a site, a session database and a T3 dependency"
updated_at: "2026-11-09T06:42:27.036509951+00:00"
---

## Goal

Record decision atelier-7bs1 as ADR 0030 and bring every current product and architecture document into line with it, so the site, the session database and T3 coupling do not contradict the documents workers are told to follow.

## Done when

- [ ] `docs/adr/0030-*.md` states the decision; what stays invariant (records in Git are the only source of truth for work, and the guard decisions of `merge`, `land`, `check` and `doctor` consult only Git); that the site reads records only from each repository's base branch commit and writes nothing; what the session database may hold (attribution Git never held) and may not (any copy or cache of records); that the CLI only writes it and only `atelier serve` reads it; that the site is read-only until a later decision; that humans read briefs and debriefs on the site while agents keep the Markdown forms; and that attribution goes through the CLI.
- [ ] `PRODUCT_INTENT.md` no longer rules out a human UI, a long-running read-only server, a host-wide attribution database or a T3 Code dependency; its "No database or query cache" principle and Boundaries say they mean storing or caching records, and its paragraph on the static `debrief --html` page describes the site's pages instead.
- [ ] `docs/architecture/markdown-first-record-store.md` says records have no database, cache or daemon and points to ADR 0030 for the attribution database and the server; ADR 0019 and any other current ADR whose wording ADR 0030 narrows is marked as scoped by it. Historical strategies stay as they are.
- [ ] `docs/index.md` and the ADR index link ADR 0030, and in `PRODUCT_INTENT.md`, `CONTEXT.md`, `docs/product/` and `docs/architecture/` a `git grep -n -i -E "sqlite|database|daemon|second UI|server"` finds no statement that contradicts it; the Report lists the matches reviewed.

## Scope

Docs only. Do not design the server, schema or pages beyond what the ADR must say; decision atelier-c5vk and the strategy hold the site's design. Leave `docs/product/mission-reviews.md`'s `--html` instructions to atelier-59fz, which removes the flag.

## Context

Decision atelier-7bs1, `docs/strategies/fleet-dashboard.md` and `docs/strategies/agent-attribution.md` hold the reasoning; ADR 0019 and `docs/strategies/atelier-rebuild.md` explain why the Boundaries were written. Planning review finding atelier-roov named the architecture contradiction. Sumeet's 2026-11-08 direction on atelier-yfqu replaced the HTML files with the site.
