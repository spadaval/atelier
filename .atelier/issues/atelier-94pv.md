---
created_at: "2026-11-11T02:17:26.233459064+00:00"
id: "atelier-94pv"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: master
  work_branch: atelier-94pv
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
closed_at: "2026-11-11T06:57:43.710890327+00:00"
status: "done"
title: "Move the spike's app to desk/ on the new API: the repositories, mission, item and inbox pages"
updated_at: "2026-11-11T06:57:43.710890327+00:00"
---

## Goal

Give Sumeet the pages: move the spike's app to `desk/` at the repository root, reading `atelier-desk`'s API (atelier-uwxy). After this item, opening the Desk shows the repositories, a mission's page with what it is for, its documents and its items, an item's page and the inbox, at desktop and phone widths. It replaces what `atelier brief` and `atelier debrief` showed, as the table "The brief and the debrief on the mission page" in `docs/architecture/desk.md` maps them.

## Done when

- [x] `desk/` holds the app on the spike's stack (Vite, React, react-router, react-markdown, lucide icons, plain CSS with tokens), with the spike's layout, shell and styles carried over from `crates/atelier-site/deck/` on branch `spike/mission-control`. `pnpm typecheck`, `pnpm test` and `pnpm build` pass in `desk/`.
- [x] The app's API types are the generated files in `desk/src/api/` and no response type is written by hand.
- [x] **Repositories** (`/`): each repository with its counts of missions in progress, drafts and items waiting, linking to its missions; a repository with `error` shows the error text.
- [x] **Mission** (`/r/:repo` and `/r/:repo/mission/:id`): the rail of the repository's missions, with the count of items in no mission shown as text that links nowhere (the API gives only that count, and a page for loose work is not in this item), and the mission's page with its title, status and per-status counts; an Overview with the Why, Outcome, Success criteria, Scope, Authority and Result sections present, what waits on the human, and the plan reviews with their findings and what changed since each; a Work tab with the items grouped by status in lifecycle order, each with its holder, blockers and what it needs; a Documents tab that lists the mission's documents and renders one, with relative links between documents resolved. A done item shows its Report and the findings fixed in review.
- [x] **Item** (`/r/:repo/item/:id`): its goal, Done when lines with their ticks, scope, report, decisions and responses, reviews with findings, blockers, and whether it was read from the base or its branch.
- [x] **Inbox** (`/inbox`): for every repository, the needs-human items grouped by mission, the draft missions and the open questions, each linking to its page.
- [x] No page shows a control for an act, a thread or the history: those arrive with atelier-h3hl and atelier-e05n, and the history is out of the mission. The spike's Health, Insights, Agents, digest and activity-feed code is not carried over.
- [x] Unit tests cover the logic that is not layout, among them resolving a document's relative link and grouping items by status. Playwright tests on fixture responses load each of the four pages at a desktop and a phone width and check the mission page's title, status and counts, a rendered document, that every link in the rail opens a page that renders, and that nothing overflows the phone's width.
- [x] `pnpm dev` proxies `/api` to a running `atelier-desk` at an address taken from an environment variable with a loopback default, so a change to the app needs no Rust build. `desk/README.md` gives the commands for both.
- [x] CI has a `desk` job that runs the typecheck, unit tests, build and Playwright tests when a change touches `desk/`, and only then; `scripts/ci-jobs.sh` and its test say so. It builds no Rust.
- [x] With `atelier-desk` serving the converted clone at `~/.cache/atelier-desk/clones/atelier` and the built app, the Report has a screenshot of the mission page of atelier-z4cr and of the inbox, each at desktop and phone widths, taken with a headless browser, and says that the page's title, status and counts match the new `atelier show atelier-z4cr` there.

## Scope

`desk/`, the `desk` job in `.github/workflows/ci.yml`, `scripts/ci-jobs.sh` with its test, and `docs/architecture/desk.md` where the app as built differs from it. No Rust: if a page needs a field the API does not give, add it in `next/crates/atelier-desk` only when it is a field of a view that exists, regenerate the types and say so in the Report; anything larger is an escalation. The spike's Node server (`server/sidecar.ts`) is not carried over, and nothing in `desk/` runs a server of its own outside tests. Arranging the pages within the spike's design is the implementer's choice; `VISION.md` on the spike branch says what the design is for.

Expected size: about 4,500 lines in `desk/`, of which about 2,900 are the spike's stylesheet carried over and pruned, and about 600 lines of tests and fixtures.

## Context

- The contract: `docs/architecture/desk.md`. The strategy: `docs/strategies/desk.md` (Shape: "Missions first", "A page that only lists records is not finished"; "agreeing with the CLI beats showing more; a page the human can act from beats a complete one"). The product intent: `docs/product/desk.md`.
- The spike is checked out at `/home/supadava/projects/atelier-spike-mc`. Read it there and change nothing in it. The first item's Report (atelier-1ie1) lists each spike page as carried over, changed or left.
- The old site's `web` CI job in `.github/workflows/ci.yml` shows how pnpm, Node and Playwright are set up in CI. The switch (atelier-u1ko) removes that job and the old site; this item adds a separate job and leaves the old one alone. Check before starting that neither atelier-u1ko nor atelier-wd1i is in progress, and if the switch merges while this branch is open, bring master in with `atelier merge --sync`.
- Converted clones live under `~/.cache/atelier-desk/`. Put nothing in `/tmp`.
- Sizes: complexity medium, value high, risk low. The risk is taste, which Sumeet judges after the merge, not before.

## Notes

- 2026-11-11 [note] Handoff: approved by gpt-6.1-sol-94pv-r2 (atelier-s24t) at f5121a96 after two review rounds; round 1's three fix_now findings fixed and confirmed. Synced master (666a3545) once on the orchestrator's instruction. Only ci.passed is outstanding; merge with atelier merge atelier-94pv --push --wait. See the Report.

## Report

All ten Done when lines hold. `desk/` is the spike's app on atelier-desk's API: Vite, React, react-router, react-markdown, lucide icons and the spike's stylesheet with its tokens, pruned of the palette, threads, feed, charts, digest, health and action forms (1,948 lines kept). `pnpm typecheck` passes, `pnpm test` passes 22 of 22 in 4 files, `pnpm build` passes, and `pnpm e2e` passes 20 of 20 (10 tests at 1440 and 390 pixels wide), all at the final code commit. The API types are the generated `desk/src/api/` files, unchanged from master; the fixtures in `desk/e2e/fixtures.ts` are typed with them, and no response type is written in `src/`.

- Repositories (`/`): `the repositories page shows each repository's counts and missions, and a repository's error`.
- Mission (`/r/:repo`, `/r/:repo/mission/:id`): `the mission page shows the title, status and counts atelier show prints` (Overview with Waiting on you, Why, Outcome under the title, Success criteria, Scope, Authority, Result, plan reviews with findings and changed-since, answered questions); `the Work tab groups items by status in lifecycle order, and a done item shows its Report and fixes`; `the Documents tab renders a document and follows its relative links`; `every mission in the rail opens a page that renders, and the loose count links nowhere`; `a repository opens on the mission with work in progress, and on a phone on its rail`; `a repository that cannot be read says so in the rail, where a phone shows it`. Opened findings are checked to span their list on both pages. Unit tests: `resolveLink` (7 cases), `splitFrontMatter`, `slug`, `groupByStatus`, `waitingOnYou`, `fixedInReview` and `findingFates` over fixed, withdrawn, kept-then-fixed, kept-and-open, decided and uncarried histories, the counts line.
- Item (`/r/:repo/item/:id`): `the item page shows its goal, Done when ticks, report, reviews and blockers`, including Read from the base or its branch, decisions, responses and links into the mission's Documents tab.
- Inbox (`/inbox`): `the inbox lists every repository's needs-human items by mission, its drafts and its questions`.
- No controls: every page test calls `expectNoActs` (no form, input or act button). Nothing from the spike's Health, Insights, Agents, digest or feed code is in `desk/`.
- Phone width: every page load in the suite fails if the page is wider than the screen.
- `pnpm dev` proxies `/api` to `ATELIER_DESK` (default `http://127.0.0.1:3780`); checked against a running atelier-desk on port 3794, where `/api/repos` answered through Vite. `desk/README.md` has the commands for both ways of running it.
- CI: a `desk` job in `.github/workflows/ci.yml` runs the four checks with no Rust toolchain, gated on a new `desk` output of `scripts/ci-jobs.sh`, true only for paths under `desk/` (and, as for every job, when there is no base to compare). `scripts/ci-jobs.test.sh` passes 29 of 29 cases, and all 29 fail against the old script.
- Screenshots, from the built app served by `~/.cache/atelier-desk/bin/atelier-desk` over `~/.cache/atelier-desk/clones/atelier` (base tip 2116defd), headless Chromium, retaken after the sync on the binary that serves atelier-e05n's routes: `~/.cache/atelier-desk/shots/94pv/mission-z4cr-desktop.png`, `mission-z4cr-phone.png`, `inbox-desktop.png`, `inbox-phone.png`. The page reads title "Build the Desk: the human's place to plan missions and read results, on the redesigned records", status "Ready", counts "5 draft, 2 ready"; `atelier show atelier-z4cr` there prints `Mission atelier-z4cr: Build the Desk: the human's place to plan missions and read results, on the redesigned records`, `Status: ready`, and five items under `Draft:` and two under `Ready:`. The inbox lists atelier-j86z, atelier-8i86, atelier-pywh, atelier-7w9s and atelier-3km4, the five `atelier list --needs-human` prints there.

### Surprises

- atelier-e05n merged while this branch was in review (666a3545) and added required `threads` and `t3` fields to the mission, item and inbox responses. On the orchestrator's instruction the branch synced master once (`atelier merge atelier-94pv --sync`, d399cb13, clean) and the fixtures gained those fields; the pages still show no threads. The suites above and the screenshots were rerun after the sync, on the updated `atelier-desk` binary.

### Decisions

- Every item links to its own page; the spike's side sheet over the page is not carried over, nor its ⌘K palette, which searched boards the new API does not have.
- A done item's Report and its fixed findings load from `/items/{id}` when opened on the Work tab, rather than adding fields to the mission view. "Fixed in review" follows the core's carry rule (`is_open` in `atelier-core/src/review.rs`): a first-raised fix-now finding counts as fixed only when the chain of later findings that carry it (`was`) ends in one whose `now` is fixed and that no decision settled. Every finding's shown fate comes from the same chain (`desk/src/lib/findings.ts`).
- Documents are listed by file name with their first heading beneath, and their front matter is shown as fields, because the converted `design.md` files start with front matter and `# Sizing`. A relative link to any Markdown file of the repository opens in the mission's Documents tab; a link to another file shows as text.
- The mission's status pill says the record's status, as `show` does; work in progress is a separate tag.
- The Playwright suite answers `/api` in the browser from typed fixtures and serves the built app with `vite preview`, so it needs no server of the Desk's own.

### Open items

- The converted clone has no reviews, questions or done items, so those parts were checked on fixtures only.
- Where later items add controls: Say go and the planning-session button in `MissionView`'s header (`desk/src/pages/Mission.tsx`, under the status line where the draft note is), an answer form on each question line (`QuestionLine` there and on the item page's decisions), and the agents list as a card in the mission's side pane and the item's side pane (`desk/src/pages/Item.tsx`).
