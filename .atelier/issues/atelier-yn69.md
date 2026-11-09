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
updated_at: "2026-11-09T14:32:17.104610926+00:00"
---

## Goal

Show every mission's brief, debrief and strategy as pages Sumeet can read on his phone: a Vite, TypeScript and React app in `crates/atelier-site/web/`, embedded in the `atelier` binary and served by `atelier serve` over atelier-zpmb's API (decision atelier-8p7u). This is the first page a human sees, and it proves the build ships a Node app inside a Rust binary through `cargo install` and the release.

## Done when

- [ ] Shipping first: the built app is embedded with `rust-embed`, and the server answers `/` and any non-`/api/` path with its `index.html`. `cargo install --path crates/atelier-cli --locked` from a clean clone with Node and pnpm installed produces a binary that serves the app, and without pnpm it fails with an error naming the missing step, never a binary without the app; `cargo nextest run` needs no Node. `.github/workflows/release.yml` builds the app before `dist` builds the binaries, and a `dist build` for one Linux target in CI (or on boat) embeds it; the Report records how.
- [ ] `/` lists the repositories with a link to each mission. `/<repo>/mission/<id>` shows the brief while the mission is in its workflow's initial status and the debrief afterwards, linking `/brief`, `/debrief` and `/strategy`; each shows what the corresponding Markdown command prints, laid out for reading. Vitest and Testing Library tests render each page from atelier-zpmb's fixture JSON, covering a draft mission, a closed one, and a mission with and without a strategy.
- [ ] Markdown renders with `react-markdown`, raw HTML skipped, under one URL policy: relative, fragment, `http`, `https` and `mailto` links and images are kept, and `javascript:`, `data:`, `vbscript:` and any other scheme render as plain text. A test renders a strategy holding raw HTML and each of those destinations, and a title holding `<script>` appears as text on every page.
- [ ] A Playwright suite starts `atelier serve` on a fixture repository and loads every route at a 390 px viewport with no console error and no horizontal scroll, including an unknown mission, which shows a not-found page.
- [ ] CI gains a `web` job, chosen by `scripts/ci-jobs.sh` (with a case in `scripts/ci-jobs.test.sh`) when `crates/atelier-site/` changes, that runs `pnpm install --frozen-lockfile`, the typecheck, Vitest, the build and Playwright; it passes on the pull request.
- [ ] `pnpm dev` in `web/` serves the app with reloads and proxies `/api` to an `atelier serve` named by an environment variable; the site's architecture page and `README.md` say how to develop, test and install it, and `AGENTS.md`'s install line still works as written or is updated.
- [ ] The repository ships a systemd user unit for `atelier serve` with `Restart=on-failure`, `MemoryMax=` and `CPUQuota=`; `docs/` says how to install it, restart it after `cargo install`, and publish it with `tailscale serve` without changing T3 Code's published port.
- [ ] Run against this host's repositories with the installed build, every mission's brief and debrief page renders in Playwright without an error; the Report records the command and counts.

## Scope

Brief, debrief and strategy pages, the repository list, shipping, CI and the unit file. No board, item or inbox pages (atelier-ahpf, atelier-uma8), no removal of `--html` (atelier-59fz).

## Context

Blocked by atelier-zpmb, whose API, generated types and fixture JSON this builds on. Decision atelier-8p7u settles the stack: Vite, TypeScript, React, pnpm, React Router, `react-markdown`, Vitest, Testing Library and Playwright. Node and pnpm are on this host for T3 Code. Heavy builds, such as a full `dist build` across targets, go to boat (the `boat` skill), not this host. The existing HTML renderers (`crates/atelier-cli/src/review_page.rs`, `render_html` and `render_brief_html` in `commands/mission_review.rs`) show what the pages hold today; they are not ported, and atelier-59fz deletes them.

## Unknowns

- How the build is wired: `build.rs` running pnpm when the embedded directory is missing or stale, or a feature that release and install builds turn on. Either meets the first Done when; record a decision note.
