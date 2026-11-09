---
created_at: "2026-11-09T14:32:17.104610926+00:00"
id: "atelier-yn69"
issue_type: "feature"
labels:
- "fleet-dashboard"
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
status: "todo"
title: "Show briefs, debriefs and strategies as pages: a Vite and React app embedded in atelier serve"
updated_at: "2026-11-09T14:43:23.791717404+00:00"
---

## Goal

Show every mission's brief, debrief and strategy as pages Sumeet can read on his phone: a Vite, TypeScript and React app in `crates/atelier-site/web/`, embedded in the `atelier` binary and served by `atelier serve` over atelier-zpmb's API (decision atelier-8p7u). This is the first page a human sees, and it proves the build ships a Node app inside a Rust binary through `cargo install` and the release.

## Done when

- [ ] Shipping first: the built app is embedded with `rust-embed`, and the server answers a path naming an embedded asset with that asset and its content type, a missing path under the asset directory with a 404 (never `index.html`), and any other non-`/api/` path with the app's `index.html`. A test loads the built app at `/` and at a deep link and finds its JavaScript and CSS served with their content types, and requests a missing asset and gets a 404. `cargo install --path crates/atelier-cli --locked` from a clean clone with Node and pnpm installed produces a binary that serves the app, and without pnpm it fails with an error naming the missing step, never a binary without the app; `cargo nextest run` needs no Node. The wiring is the strategy's Build and ship: `crates/atelier-site/build.rs` builds the app (`pnpm install --frozen-lockfile` and `pnpm build` in `web/`) when the release profile is built and `web/dist` is missing or older than the app's sources, and fails naming the missing step when pnpm is absent; debug builds embed `web/dist` if it exists and otherwise serve a page saying how to build it, so Rust tests need no Node. `dist` runs that `build.rs` like any release build; its runners get Node and pnpm through `dist-workspace.toml`'s `github-build-setup`, and `.github/workflows/release.yml` is regenerated with `dist generate`, never edited by hand. A `dist build` for one Linux target (on boat or in CI) produces a binary that serves the app; the Report records the command.
- [ ] `/` lists the repositories with a link to each mission. `/<repo>/mission/<id>` shows the review atelier-zpmb's API names for it (the app reads no statuses), linking `/<repo>/mission/<id>/brief`, `/<repo>/mission/<id>/debrief` and `/<repo>/mission/<id>/strategy`; each shows what the corresponding Markdown command prints, laid out for reading. Vitest and Testing Library tests render each page from atelier-zpmb's fixture JSON, covering a draft mission, a closed one, and a mission with and without a strategy.
- [ ] Markdown renders with `react-markdown`, raw HTML skipped, under one URL policy: relative, fragment, `http`, `https` and `mailto` links and images are kept, and `javascript:`, `data:`, `vbscript:` and any other scheme render as plain text. A test renders a strategy holding raw HTML and each of those destinations, and a title holding `<script>` appears as text on every page.
- [ ] A Playwright suite starts `atelier serve` on a fixture repository and loads every route at a 390 px viewport with no console error and no horizontal scroll, including an unknown mission, which shows a not-found page.
- [ ] CI gains a `web` job, chosen by `scripts/ci-jobs.sh` whenever the Rust jobs run or `crates/atelier-site/web/` changes, and when the base is missing or unknown (with cases for each in `scripts/ci-jobs.test.sh`), that runs `pnpm install --frozen-lockfile`, the typecheck, Vitest, the build and Playwright; it passes on the pull request.
- [ ] `pnpm dev` in `web/` serves the app with reloads and proxies `/api` to an `atelier serve` named by an environment variable; the site's architecture page and `README.md` say how to develop, test and install it, and `AGENTS.md`'s install line still works as written or is updated.
- [ ] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=`; `docs/` says how to install it, restart it after `cargo install`, and publish it with `tailscale serve` without changing T3 Code's published port.
- [ ] Run against this host's repositories with the installed build, every mission's brief and debrief page renders in Playwright without an error; the Report records the command and counts.

## Scope

Brief, debrief and strategy pages, the repository list, shipping, CI and the unit file. No board, item or inbox pages (atelier-ahpf, atelier-uma8), no removal of `--html` (atelier-59fz).

## Context

Blocked by atelier-zpmb, whose API, generated types (`web/src/api/`) and fixture JSON (`web/src/fixtures/`) this builds on: scaffold the app around those two directories and keep them as they are. Decision atelier-8p7u settles the stack: Vite, TypeScript, React, pnpm, React Router, `react-markdown`, Vitest, Testing Library and Playwright. Node and pnpm are on this host for T3 Code. Heavy builds, such as a full `dist build` across targets, go to boat (the `boat` skill), not this host. The existing HTML renderers (`crates/atelier-cli/src/review_page.rs`, `render_html` and `render_brief_html` in `commands/mission_review.rs`) show what the pages hold today; they are not ported, and atelier-59fz deletes them.

## Unknowns

- Whether `dist`'s musl and aarch64 runners build the app as they are; the first Done when finds out, and a change to the wiring goes to Sumeet.
