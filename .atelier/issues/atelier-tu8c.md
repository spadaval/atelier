---
created_at: "2026-11-09T00:45:57.947764486+00:00"
id: "atelier-tu8c"
issue_type: "task"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: atelier-tu8c
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
status: "in_progress"
title: "Record ADR 0030: Atelier may have a site, a session database and a T3 dependency"
updated_at: "2026-11-09T15:11:32.101351178+00:00"
---

## Goal

Record decision atelier-7bs1 as ADR 0030 and bring every current product and architecture document into line with it, so the site, the session database and T3 coupling do not contradict the documents workers are told to follow.

## Done when

- [x] `docs/adr/0030-*.md` states the decision; what stays invariant (records in Git are the only source of truth for work, and the guard decisions of `merge`, `land`, `check` and `doctor` consult only Git); that the site writes nothing and reads no working tree: what it says about missions and work comes from one captured commit of each repository's base branch, and only the overlay of work in flight (branches, approvals and findings) comes from each owned work branch's own records at a pinned commit; what the session database may hold (attribution Git never held) and may not (any copy or cache of records); that the CLI only writes it and only `atelier serve` reads it; that the site is read-only until a later decision; that it is a web app over a read-only JSON API built from the CLI's own views, which agents do not use (they keep the Markdown, ADR 0022); that humans read briefs and debriefs on the site while agents keep the Markdown forms; and that attribution goes through the CLI.
- [x] `PRODUCT_INTENT.md` no longer rules out a human UI, a long-running read-only server, a host-wide attribution database or a T3 Code dependency; its "No database or query cache" principle and Boundaries say they mean storing or caching records, and its paragraph on the static `debrief --html` page describes the site's pages instead.
- [x] `docs/architecture/markdown-first-record-store.md` says records have no database, cache or daemon and points to ADR 0030 for the attribution database and the server; ADR 0019 and any other current ADR whose wording ADR 0030 narrows is marked as scoped by it. Historical strategies stay as they are.
- [x] `docs/index.md` and the ADR index link ADR 0030, and in `PRODUCT_INTENT.md`, `CONTEXT.md`, `docs/product/` and `docs/architecture/` a `git grep -n -i -E "sqlite|database|daemon|second UI|server"` finds no statement that contradicts it; the Report lists the matches reviewed.

## Scope

Docs only. Do not design the server, schema or pages beyond what the ADR must say; decisions atelier-c5vk and atelier-8p7u and the strategy hold the site's design. Leave `docs/product/mission-reviews.md`'s `--html` instructions to atelier-59fz, which removes the flag.

## Context

Decision atelier-7bs1, `docs/strategies/fleet-dashboard.md` and `docs/strategies/agent-attribution.md` hold the reasoning; ADR 0019 and `docs/strategies/atelier-rebuild.md` explain why the Boundaries were written. Planning review finding atelier-roov named the architecture contradiction. Sumeet's 2026-11-08 direction on atelier-yfqu replaced the HTML files with the site.

## Report

All four Done when lines hold. `docs/adr/0030-a-read-only-site-a-session-database-and-t3-code.md` (114 lines) records decision atelier-7bs1 in eight numbered decisions that follow the first line's list. `PRODUCT_INTENT.md` now limits "No database or query cache" and its Boundaries to records. It allows `atelier serve`, the session database and T3 Code, and its old `debrief --html` paragraph now describes the site's pages, with `--html` kept until atelier-yfqu ships the site. The record store doc says records have no database, cache, daemon or persistent query index and points to ADR 0030. ADR 0019 (decision 7 and the removed SQLite crate) and ADR 0022 (no JSON mode for agents) are marked as scoped by it. ADR 0030 is linked from `docs/index.md`'s ADR entry, which serves as the ADR index because the repository has no separate one, and from `docs/architecture/index.md`. `CONTEXT.md` gains the terms Site and Session database. `docs/architecture/source-layout.md` and `docs/product/development-setup.md` no longer state an unqualified "no database". Proof: the Done when's `git grep` was rerun after the final edit (35 matches, all reviewed below), along with `git diff --check` (clean) and `atelier check` (exit 0, warnings only on other items).

### Matches reviewed

- `PRODUCT_INTENT.md:27-31, 77, 114-118, 127`: the edited principle, site paragraph and Boundaries. They match ADR 0030.
- `CONTEXT.md:30`: the new Session database term. `CONTEXT.md:61`: "SQLite domain cache" listed as a retired term, which is still true.
- `docs/architecture/index.md:6-7`, `markdown-first-record-store.md:5-10`, `source-layout.md:24-28`: edited here. They match ADR 0030. `source-layout.md:16` says graph queries use the direct index, not a database adapter, which is still true.
- `docs/product/development-setup.md:50`: now reads "no record database or cache".
- `docs/architecture/provenance.md:5, 17`: Chainlink's SQLite model, labeled historical. No contradiction.
- `docs/architecture/quality/*` (13 matches across five dated audits): descriptions of the system as it was when examined. `docs/index.md` says they do not replace the current guides. No contradiction.
- `docs/product/mission-reviews.md:91`: the `--html` page is "a document, not a second UI". This is left to atelier-59fz per Scope. It describes the flag as it works today and does not rule out the site.
- `docs/product/validation.md:43`: "a test server for production", unrelated.

### Decisions

- Decision 1 of the ADR adds the forge's CI result from ADR 0031 to "consult only Git", because `merge`'s `ci.passed` already reads it through `gh`.
- Decision 2 allows the server's in-memory views, keyed by commit, which the fleet-dashboard strategy's Design relies on. It says why they are not a record cache, so the record store's "no cache" holds.

