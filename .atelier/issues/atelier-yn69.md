---
created_at: "2026-11-09T14:32:17.104610926+00:00"
id: "atelier-yn69"
issue_type: "feature"
labels:
- "fleet-dashboard"
workflow_branch:
  branch_base: master
  work_branch: atelier-yn69
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
  - kind: "issue"
    id: "atelier-zpmb"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-09T19:10:26.925196975+00:00"
status: "done"
title: "Show briefs, debriefs and strategies as pages: a Vite and React app embedded in atelier serve"
updated_at: "2026-11-09T19:10:26.925196975+00:00"
---

## Goal

Show every mission's brief, debrief and strategy as pages Sumeet can read on his phone: a Vite, TypeScript and React app in `crates/atelier-site/web/`, embedded in the `atelier` binary and served by `atelier serve` over atelier-zpmb's API (decision atelier-8p7u). This is the first page a human sees, and it proves the build ships a Node app inside a Rust binary through `cargo install` and the release.

## Done when

- [x] Shipping first: the built app is embedded with `rust-embed`, and the server answers a path naming an embedded asset with that asset and its content type, a missing path under the asset directory with a 404 (never `index.html`), and any other non-`/api/` path with the app's `index.html`. A test loads the built app at `/` and at a deep link and finds its JavaScript and CSS served with their content types, and requests a missing asset and gets a 404. `cargo install --path crates/atelier-cli --locked` from a clean clone with Node and pnpm installed produces a binary that serves the app, and without pnpm it fails with an error naming the missing step, never a binary without the app; `cargo nextest run` needs no Node. The wiring is the strategy's Build and ship: `crates/atelier-site/build.rs` builds the app (`pnpm install --frozen-lockfile` and `pnpm build` in `web/`) when the release profile is built and `web/dist` is missing or older than the app's sources, and fails naming the missing step when pnpm is absent; debug builds embed `web/dist` if it exists and otherwise serve a page saying how to build it, so Rust tests need no Node. `dist` runs that `build.rs` like any release build; its runners get Node and pnpm through `dist-workspace.toml`'s `github-build-setup`, and `.github/workflows/release.yml` is regenerated with `dist generate`, never edited by hand. A `dist build` for one Linux target (on boat or in CI) produces a binary that serves the app; the Report records the command.
- [x] `/` lists the repositories with a link to each mission. `/<repo>/mission/<id>` shows the review atelier-zpmb's API names for it (the app reads no statuses), linking `/<repo>/mission/<id>/brief`, `/<repo>/mission/<id>/debrief` and `/<repo>/mission/<id>/strategy`; each shows what the corresponding Markdown command prints, laid out for reading. Vitest and Testing Library tests render each page from atelier-zpmb's fixture JSON, covering a draft mission, a closed one, and a mission with and without a strategy.
- [x] Markdown renders with `react-markdown`, raw HTML skipped, under one URL policy: relative, fragment, `http`, `https` and `mailto` links and images are kept, and `javascript:`, `data:`, `vbscript:` and any other scheme render as plain text. A test renders a strategy holding raw HTML and each of those destinations, and a title holding `<script>` appears as text on every page.
- [x] A Playwright suite starts `atelier serve` on a fixture repository and loads every route at a 390 px viewport with no console error and no horizontal scroll, including an unknown mission, which shows a not-found page.
- [x] CI gains a `web` job, chosen by `scripts/ci-jobs.sh` whenever the Rust jobs run or `crates/atelier-site/web/` changes, and when the base is missing or unknown (with cases for each in `scripts/ci-jobs.test.sh`), that runs `pnpm install --frozen-lockfile`, the typecheck, Vitest, the build and Playwright; it passes on the pull request.
- [x] `pnpm dev` in `web/` serves the app with reloads and proxies `/api` to an `atelier serve` named by an environment variable; the site's architecture page and `README.md` say how to develop, test and install it, and `AGENTS.md`'s install line still works as written or is updated.
- [x] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=`; `docs/` says how to install it, restart it after `cargo install`, and publish it with `tailscale serve` without changing T3 Code's published port.
- [x] Run against this host's repositories with the installed build, every mission's brief and debrief page renders in Playwright without an error; the Report records the command and counts.

## Scope

Brief, debrief and strategy pages, the repository list, shipping, CI and the unit file. No board, item or inbox pages (atelier-ahpf, atelier-uma8), no removal of `--html` (atelier-59fz).

## Context

Blocked by atelier-zpmb, whose API, generated types (`web/src/api/`) and fixture JSON (`web/src/fixtures/`) this builds on: scaffold the app around those two directories and keep them as they are. Decision atelier-8p7u settles the stack: Vite, TypeScript, React, pnpm, React Router, `react-markdown`, Vitest, Testing Library and Playwright. Node and pnpm are on this host for T3 Code. Heavy builds, such as a full `dist build` across targets, go to boat (the `boat` skill), not this host. The existing HTML renderers (`crates/atelier-cli/src/review_page.rs`, `render_html` and `render_brief_html` in `commands/mission_review.rs`) show what the pages hold today; they are not ported, and atelier-59fz deletes them.

## Unknowns

- Whether `dist`'s musl and aarch64 runners build the app as they are; the first Done when finds out, and a change to the wiring goes to Sumeet.

## Report

All eight Done when lines hold. Shipping was proven first: `cargo install --path crates/atelier-cli --locked` from a clean clone built the app through `crates/atelier-site/build.rs` and served it, and with no pnpm on `PATH` it failed naming the step and installed nothing (receipt in the evidence). A `dist build --artifacts=local --target=x86_64-unknown-linux-gnu` on a fresh boat VM (`boat run --type large`, Node 24 and pnpm 12.18 installed by the script, no `web/dist`) built the app and its binary served a deep link, the JS and CSS with their content types and a missing asset as a 404. The asset routing has unit tests over fixed assets (`app::tests`, 4) and an integration test through the real router with or without a built app (`paths_outside_the_api_get_the_app_and_a_missing_asset_is_a_404`); the Playwright suite checks the built app's JS and CSS content types and the missing-asset 404. Vitest (22 tests) renders every page from the fixture JSON: draft alpha-m1 with a strategy, closed alpha-m2 without one, beta-m2 open past its plan, the not-found pages, the URL policy for each destination and raw HTML, and a `<script>` title as text on all five pages. Playwright (21 tests) loads every route of the fixture repositories at 390 px. CI's new `web` job passed on PR #79. Against this host, the installed build served 5 repositories and 42 missions, and all 84 brief and debrief pages rendered with no console error, alert or horizontal scroll: `ATELIER_SERVE=http://127.0.0.1:7398 pnpm exec playwright test -c playwright.host.config.ts` in `crates/atelier-site/web`. The workspace suite passed (1122) and `scripts/smoke-repos.sh` found no regression.

### Deviations

- `/` used to answer a JSON 404; it is now the app, so `unknown_repositories_missions_and_routes_are_json_404s` checks `/api/no/such/route` instead.
- Strategy Markdown headings render one level down (`#` as `h2`), so each page has one `h1`.

### Decisions

- The Playwright suite builds its fixture repositories with an ignored Rust test, `write_fixture_fleet`, so the e2e fixtures are the same alpha and beta the API's fixture JSON comes from. `e2e/serve.sh` runs it and serves them with `target/debug/atelier`.
- Not-found pages may log the browser's own "status of 404" console message for the API call; the suite allows that one message on those pages only.
- The unit caps the service at `CPUQuota=100%` and `MemoryMax=1G`; the docs publish it with `tailscale serve --bg --http=7341`, a port T3 Code does not use here (T3 holds 80, 3847, 6080 and 8443).

### Open items

- The musl and aarch64 targets were not built. `dist plan` shows every target on a native runner (`ubuntu-22.04`, `ubuntu-22.04-arm`, macOS) with no container, so `build.rs` runs pnpm on the runner's host as it did for x86_64 gnu; the first tagged release proves them.
- The unit is shipped and documented, not installed; installing and publishing it is loose end atelier-6rts.
