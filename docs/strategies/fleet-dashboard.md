---
status: draft
last_changed: "2026-11-09: planning review fixes: base-ref bootstrap, server-chosen review, asset routing, web CI on every Rust change, build.rs and dist wiring, full routes"
---

# Why

Sumeet reviews Atelier work at two points: a mission's brief before it starts
and its debrief after it ends. Today each is a file `--html` writes into
`.atelier/runtime/reviews/` of whichever checkout ran it, or an ad-hoc artifact
an agent publishes, so he gets a different document in a different place each
time and his phone cannot reach most of them. Between those points he can only
learn where a project stands by asking an agent, which he does in nearly every
session. He wants the site to be agent Jira, not agent Gmail: pages that stay
current from the base branch, with a queue of what waits on him pointing into
them.

# Outcome

A site on this host, reached over Tailscale, shows every Atelier repository
here. Every mission's brief and debrief is a page, current from its
repository's base branch, and agents send Sumeet that page instead of writing
an HTML file or publishing an artifact. Each repository has a board that
answers where are we, what's done and what's next. An inbox lists what waits
on him and links into those pages. The CLI's `--html` output and
`.atelier/runtime/reviews/` are gone; the Markdown brief and debrief stay for
agents.

# Success Criteria

- site-matches-cli: with the site installed and published on this host, loaded
  in a headless browser over the Tailscale hostname, every mission of every
  Atelier repository here has a brief and a debrief page that render without
  an error and show the mission's title as `atelier issue show` gives it, and
  the criteria and item counts that `atelier brief -q` and `atelier debrief -q`
  print, all run on that repository's base branch; each repository's board
  lists the same in-progress, ready and needs-human items as `atelier status`
  on its base branch; and every inbox link opens a page that renders without
  an error. Shown by validation item atelier-mtb3.

# Sizing

- Complexity: high. A new crate and a web app in a second language, a
  long-running server, a loader that reads records from a commit instead of a
  directory, a JSON contract between them, and URLs that agents and Sumeet will
  rely on.
- Value: high. The brief and debrief are the human's two review points under
  the mission model (atelier-fl12), and "where are we?" is asked in nearly
  every session.
- Risk: medium. The site is read-only and reaches only this host; a wrong or
  missing page shows and the service can be stopped. The silent hazards are a
  board that disagrees with the records, checked against `atelier status` by
  fixture tests and the validation, and a release or install that ships
  without the app, which the build refuses.

Worth it: the cheaper paths are keeping `--html` files and asking agents
(Sumeet's direction rejects both) or static pages regenerated after `land`
(rejected in decision atelier-7bs1); a week or two of work removes a question
asked every session and gives both review points one place.

# Shape

- Decision atelier-7bs1 allows a read-only site and server; ADR 0030
  (atelier-tu8c) records it and narrows `PRODUCT_INTENT.md`'s Boundaries.
  Records in Git stay the only source of truth, and guard decisions read only
  Git.
- Decision atelier-c5vk settles the crate and server; decision atelier-8p7u,
  which supersedes its HTML choice, settles the app (see Design).
- The debrief is made fast first (atelier-t7pb): it runs one history walk per
  item today, and the site would inherit that cost.
- Pages, in Sumeet's order: brief and debrief first, then the board, then the
  inbox, which is a notification queue and not the home page.
- What a page says about missions and work comes from one captured commit of
  each repository's base branch, never from a checkout, so a stale or detached
  worktree cannot mislead it. The one exception is the overlay of work in
  flight (the board's branches, approvals and findings), which comes from each
  owned work branch's own records, read at a pinned commit of that branch.
  Nothing is read from a working tree, and nothing is written.
- The site shows what the CLI computes. The API serializes `atelier-app`'s
  views; the app lays them out and decides nothing about work.
- When qualities conflict: correct and current beats complete, read-only beats
  convenient, the T3 host's responsiveness beats page speed, and plain pages
  beat polish.

# Design

**Components.** A library crate, `crates/atelier-site`, owns the server:
repository discovery, the JSON API, serving the embedded app, the cache and
the load limits. Its `web/` directory holds the app: Vite, TypeScript and
React, with pnpm, React Router and `react-markdown` (decision atelier-8p7u).
`atelier-cli` gains `atelier serve`, which parses its flags and calls into the
crate, so the site ships in the one `atelier` binary. Dependency direction:
`atelier-cli` → `atelier-site` → `atelier-app`; the site never depends on the
CLI.

**Reading a repository at a commit.** `atelier-app` gains a record loader that
builds a `RecordIndex` from a commit's tree (`git ls-tree` and
`git cat-file --batch`, as `git_freshness` already does) in memory, with no
temporary directory. The brief, debrief and status views take that one captured
commit and read everything from it. Today they read the checkout in these
places, each of which must change: `RecordIndex::load` reads a directory and
validates against the working tree's `workflow.yaml`
(`record_index.rs:1232`); `mission_review.rs` reads the strategy from the
working tree (lines 887 and 1443) and pins history to `HEAD` (lines 360 and
1470); and `criteria.rs` reads the strategy's criteria from the working tree
(line 48) and runs `git rev-parse HEAD` for receipt history (line 249). Run
against a common Git directory, `HEAD` is whatever the main checkout has out,
not the base. Git runs against the repository's common Git directory with
optional locks off, so nothing the site does writes to a repository. The
loader lives in `atelier-app` because it is a records concern;
`Snapshot::at`'s tarball extraction (atelier-3i7x) can move to it later.

**Discovery.** `atelier serve --root <dir>...` (default `~/projects`) scans
for checkouts holding `.atelier/`, groups them by Git common directory so each
repository appears once, and reads each one's base branch without a checkout: it takes the repository's default branch (`refs/remotes/origin/HEAD`, else whichever one of local `main` and `master` exists), reads the workflow policy committed there, and uses the `base_branch` that policy names, which must name itself in its own committed policy. A repository with no default branch, both `main` and `master` and no `origin/HEAD`, or a policy whose base does not name itself shows that error on its own page; nothing falls back to `HEAD` or a working tree. The URL slug is the project's `project_slug`; a
repository whose records cannot be read shows its error on its own page while
the others render.

**API.** Read-only `GET` routes returning JSON: `/api/repos` (repositories, their
counts and their missions, each with its id, title, status and which review
the server says to show: the brief while the mission is in its workflow's
initial status, the debrief afterwards), `/api/<repo>/mission/<id>` (that
summary for one mission), `/api/<repo>/mission/<id>/brief`, `/api/<repo>/mission/<id>/debrief` and
`/api/<repo>/mission/<id>/strategy` (the strategy's Markdown as text). Later increments add `/api/<repo>/board`,
`/api/<repo>/item/<id>` and `/api/inbox`. Each response is an `atelier-app`
view model serialized with `serde`, plus the base commit it was read from;
the API adds no fields the CLI's views do not compute, apart from links and
that commit. TypeScript types come from the Rust types through `ts-rs` and are
checked in; a Rust test fails when they are stale. An unknown repository,
mission or item is a 404 with a JSON error, and a repository whose records
fail to load is a 500 naming the error.

**App.** Client routes mirror the pages: `/` lists repositories,
`/<repo>/mission/<id>` shows the review the API names for it (the app never
interprets statuses), linking `/<repo>/mission/<id>/brief`,
`/<repo>/mission/<id>/debrief` and `/<repo>/mission/<id>/strategy`; later `/<repo>`, `/<repo>/item/<id>` and `/inbox`. The server
answers a path that names an embedded asset with that asset and its content
type; a missing path under the asset directory is a 404, never `index.html`;
and any other non-API path gets the app's `index.html`. Markdown renders with
`react-markdown`, raw HTML skipped, and one URL policy for links and images:
relative paths, fragments, `http`, `https` and `mailto` are kept, and any other
scheme (`javascript:`, `data:`, `vbscript:` and the rest) renders as plain
text. React escapes every other string. Pages read at a 390 px viewport.

**Build and ship.** The built app is embedded in the binary with `rust-embed`.
A `cargo install` or release build either builds the app or fails naming the
missing step, so no binary ships without it: `crates/atelier-site/build.rs` builds the app (`pnpm install --frozen-lockfile` and `pnpm build` in `web/`) when the release profile is built and `web/dist` is missing or older than the app's sources, and fails naming the missing step when pnpm is absent; debug builds embed `web/dist` if it exists and otherwise serve a page saying how to build it, so Rust tests need no Node. `dist` runs that `build.rs` like any release build; its runners get Node and pnpm through `dist-workspace.toml`'s `github-build-setup`, and `.github/workflows/release.yml` is regenerated with `dist generate`, never edited by hand. While developing, `pnpm dev` serves the app with reloads and proxies
`/api` to a running `atelier serve`.

**Tests.** Rust tests check each API route against fixture repositories, and
write each response as checked-in fixture JSON that a Rust test keeps current.
Vitest and Testing Library test components against that JSON, so the
frontend's fixtures are real API output. One Playwright suite starts
`atelier serve` on a fixture repository and loads every route at a 390 px
viewport. CI gains a `web` job, chosen by `scripts/ci-jobs.sh` whenever the
Rust jobs run (the API is the shared view models, so any Rust change can
change it) or `web/` changes, and also when the base is missing, that runs the typecheck, Vitest, the build and Playwright.

**Server.** `axum` on a `tokio` runtime, bound to loopback; `tailscale serve`
publishes it beside T3 Code's port. View builds run in `spawn_blocking`, at
most two at a time (a `tower` concurrency limit); a request that waits more
than 5 seconds for a slot gets a 503. Only work that depends on nothing but the
base commit, such as a parsed record index or a debrief view, is cached, keyed
by repository and base commit, and only the latest base commit per repository
is kept. Anything read from other refs (work branches, review state) is
recomputed per request or keyed by every ref it read, and anything
date-dependent by the date. A systemd user unit runs it with
`Restart=on-failure`, `MemoryMax=` and `CPUQuota=`.

**Dependencies chosen.** `axum`, `tokio` and `tower` (decision atelier-c5vk):
maintained, and its layers give the concurrency cap and timeout without
hand-written code. Vite, TypeScript, React, React Router, `react-markdown`,
Vitest, Testing Library and Playwright (decision atelier-8p7u): fast reloads,
component and browser tests, and the stack agents write most reliably. `serde`
and `ts-rs` for the contract, `rust-embed` for shipping. Node and pnpm are
already on this host for T3 Code.

`docs/architecture/` gains a page for the site, written by atelier-zpmb and
extended by atelier-yn69.

# Derisking

Riskiest assumption: the brief, debrief and status views can be built from a
base commit alone, without a checkout, cheaply enough to serve from the host
T3 Code runs on. If it fails, the site needs checkouts of its own or a
different read path, and every later page changes.

What is known (2026-11-09, this repository): `atelier debrief atelier-k8h8 -q`
(45 items) takes 7.4 s and 63 MB, and tracing it shows 88 git processes, 45 of
them a full-history `git log` per item in `done_when_edits` at about 0.18 s
each; one walk over `.atelier/issues/` takes 0.28 s. atelier-t7pb removes that
cost first. `atelier brief atelier-yfqu -q` takes 0.13 s and `atelier status`
1.6 s. Reading the code found the checkout reads the Design lists. So the work
is a loader and one captured commit threaded through those sites.

atelier-zpmb tests it first: it serves the brief and debrief API from the base
commit with a read-only proof and the load bound measured on this host, before
any app or board work starts. The second risk, shipping a Node build inside a
Rust binary through `cargo install` and `dist`, is atelier-yn69's first Done
when, before its pages.

# Scope

In: the debrief speed fix; ADR 0030 and the documents it changes;
`atelier-site`, its app and `atelier serve`; brief, debrief, strategy, board,
item and inbox pages; the CI job; the unit file and setup docs; removing
`--html`, its renderers and `.atelier/runtime/reviews/`, and pointing Agent
Factory and the docs at the site; the validation.

Out: attribution, model metrics and live T3 thread status (mission
atelier-bnek); actions from the site, which need a later decision under ADR
0030; a dashboard inside T3 Code; agents reading the JSON API; CLI orientation
views (mission atelier-4pzj), though shared view fixes reach the site; the
debrief's content bugs (atelier-jdpf and its ten items, groom digest
atelier-8i86 G8), except that atelier-59fz rewrites their Done when lines that
name the HTML debrief.

Environment: this host only, every Atelier repository under `~/projects`.
Installing and publishing the service (loose end atelier-6rts) and Sumeet's
phone read (loose end atelier-pywh) sit outside the mission.

# Authority

The orchestrator may change items, their order, routes, API shapes, page
layout and design detail, and libraries below the framework, within ADR 0030,
decisions atelier-c5vk and atelier-8p7u, and this Shape. Changing the Outcome
or the criterion, the crate, server or app framework, keeping `--html`, or
adding write actions to the site needs Sumeet.

# Risks and Unknowns

- If atelier-t7pb leaves a large debrief over about 2 s, the cache by base
  commit carries it; warming the cache after the base moves is a later item.
- The build may not run cleanly in `dist`'s cross builds (musl, aarch64);
  atelier-yn69 proves the release path first, and building the app once and
  sharing it across targets would need Sumeet's approval as a change to this
  wiring.
- `tailscale serve` may conflict with T3 Code's own setup; atelier-6rts must
  leave T3's published port unchanged.
- The installed binary serves every repository; one whose records are newer
  than the binary shows an error page until reinstall, and the service needs a
  restart after `cargo install` (atelier-6rts says so).
- Draft missions appear on the inbox and board with their brief; the inbox does
  not yet know which drafts have finished planning review. If that proves
  noisy, the inbox item settles it.
- Removing `--html` breaks any agent or script in another repository that
  calls it; clap's error names the flag, and Agent Factory's guidance changes in
  the same item.
