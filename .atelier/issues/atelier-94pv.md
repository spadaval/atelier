---
created_at: "2026-11-11T02:17:26.233459064+00:00"
id: "atelier-94pv"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-uwxy"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-z4cr"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Move the spike's app to desk/ on the new API: the repositories, mission, item and inbox pages"
updated_at: "2026-11-11T05:26:29.800159730+00:00"
---

## Goal

Give Sumeet the pages: move the spike's app to `desk/` at the repository root, reading `atelier-desk`'s API (atelier-uwxy). After this item, opening the Desk shows the repositories, a mission's page with what it is for, its documents and its items, an item's page and the inbox, at desktop and phone widths. It replaces what `atelier brief` and `atelier debrief` showed, as the table "The brief and the debrief on the mission page" in `docs/architecture/desk.md` maps them.

## Done when

- [ ] `desk/` holds the app on the spike's stack (Vite, React, react-router, react-markdown, lucide icons, plain CSS with tokens), with the spike's layout, shell and styles carried over from `crates/atelier-site/deck/` on branch `spike/mission-control`. `pnpm typecheck`, `pnpm test` and `pnpm build` pass in `desk/`.
- [ ] The app's API types are the generated files in `desk/src/api/` and no response type is written by hand.
- [ ] **Repositories** (`/`): each repository with its counts of missions in progress, drafts and items waiting, linking to its missions; a repository with `error` shows the error text.
- [ ] **Mission** (`/r/:repo` and `/r/:repo/mission/:id`): the rail of the repository's missions, with the count of items in no mission shown as text that links nowhere (the API gives only that count, and a page for loose work is not in this item), and the mission's page with its title, status and per-status counts; an Overview with the Why, Outcome, Success criteria, Scope, Authority and Result sections present, what waits on the human, and the plan reviews with their findings and what changed since each; a Work tab with the items grouped by status in lifecycle order, each with its holder, blockers and what it needs; a Documents tab that lists the mission's documents and renders one, with relative links between documents resolved. A done item shows its Report and the findings fixed in review.
- [ ] **Item** (`/r/:repo/item/:id`): its goal, Done when lines with their ticks, scope, report, decisions and responses, reviews with findings, blockers, and whether it was read from the base or its branch.
- [ ] **Inbox** (`/inbox`): for every repository, the needs-human items grouped by mission, the draft missions and the open questions, each linking to its page.
- [ ] No page shows a control for an act, a thread or the history: those arrive with atelier-h3hl and atelier-e05n, and the history is out of the mission. The spike's Health, Insights, Agents, digest and activity-feed code is not carried over.
- [ ] Unit tests cover the logic that is not layout, among them resolving a document's relative link and grouping items by status. Playwright tests on fixture responses load each of the four pages at a desktop and a phone width and check the mission page's title, status and counts, a rendered document, that every link in the rail opens a page that renders, and that nothing overflows the phone's width.
- [ ] `pnpm dev` proxies `/api` to a running `atelier-desk` at an address taken from an environment variable with a loopback default, so a change to the app needs no Rust build. `desk/README.md` gives the commands for both.
- [ ] CI has a `desk` job that runs the typecheck, unit tests, build and Playwright tests when a change touches `desk/`, and only then; `scripts/ci-jobs.sh` and its test say so. It builds no Rust.
- [ ] With `atelier-desk` serving the converted clone at `~/.cache/atelier-desk/clones/atelier` and the built app, the Report has a screenshot of the mission page of atelier-z4cr and of the inbox, each at desktop and phone widths, taken with a headless browser, and says that the page's title, status and counts match the new `atelier show atelier-z4cr` there.

## Scope

`desk/`, the `desk` job in `.github/workflows/ci.yml`, `scripts/ci-jobs.sh` with its test, and `docs/architecture/desk.md` where the app as built differs from it. No Rust: if a page needs a field the API does not give, add it in `next/crates/atelier-desk` only when it is a field of a view that exists, regenerate the types and say so in the Report; anything larger is an escalation. The spike's Node server (`server/sidecar.ts`) is not carried over, and nothing in `desk/` runs a server of its own outside tests. Arranging the pages within the spike's design is the implementer's choice; `VISION.md` on the spike branch says what the design is for.

Expected size: about 4,500 lines in `desk/`, of which about 2,900 are the spike's stylesheet carried over and pruned, and about 600 lines of tests and fixtures.

## Context

- The contract: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md` (Shape: "Missions first", "A page that only lists records is not finished"; "agreeing with the CLI beats showing more; a page the human can act from beats a complete one"). The product intent: `docs/product/desk.md`.
- The spike is checked out at `/home/supadava/projects/atelier-spike-mc`. Read it there and change nothing in it. The first item's Report (atelier-1ie1) lists each spike page as carried over, changed or left.
- The old site's `web` CI job in `.github/workflows/ci.yml` shows how pnpm, Node and Playwright are set up in CI. The switch (atelier-u1ko) removes that job and the old site; this item adds a separate job and leaves the old one alone. Check before starting that neither atelier-u1ko nor atelier-wd1i is in progress, and if the switch merges while this branch is open, bring master in with `atelier merge --sync`.
- Converted clones live under `~/.cache/atelier-desk/`. Put nothing in `/tmp`.
- Sizes: complexity medium, value high, risk low. The risk is taste, which Sumeet judges after the merge, not before.
