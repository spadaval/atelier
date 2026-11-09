---
status: draft
last_changed: "2026-11-09: planning review fixes: one captured base commit through every view read, the branch-overlay read contract, a Markdown URL policy, an executable title check"
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

- site-matches-cli: with the site installed and published on this host, fetched
  over the Tailscale hostname, every mission of every Atelier repository here
  has a brief and a debrief page that return 200 and show the mission's title
  as `atelier issue show` gives it, and the criteria and item counts that
  `atelier brief -q` and `atelier debrief -q` print, all run on that
  repository's base branch; each repository's board lists the same
  in-progress, ready and needs-human items as `atelier status` on its base
  branch; and every inbox link returns 200. Shown by validation item
  atelier-mtb3.

# Sizing

- Complexity: high. A new crate, a long-running server, a loader that reads
  records from a commit instead of a directory, and URLs that agents and
  Sumeet will rely on.
- Value: high. The brief and debrief are the human's two review points under
  the mission model (atelier-fl12), and "where are we?" is asked in nearly
  every session.
- Risk: medium. The site is read-only and reaches only this host; a wrong or
  missing page shows and the service can be stopped. The silent hazard, a
  board that disagrees with the records, is checked against `atelier status`
  by fixture tests and by the validation.

Worth it: the cheaper paths are keeping `--html` files and asking agents
(Sumeet's direction rejects both) or static pages regenerated after `land`
(rejected in decision atelier-7bs1); a few days of work removes a question
asked every session and gives both review points one place.

# Shape

- Decision atelier-7bs1 allows a read-only site and server; ADR 0030
  (atelier-tu8c) records it and narrows `PRODUCT_INTENT.md`'s Boundaries.
  Records in Git stay the only source of truth, and guard decisions read only
  Git.
- Decision atelier-c5vk settles where the site lives and what it is built
  with (see Design).
- Pages, in Sumeet's order: brief and debrief first, then the board, then the
  inbox, which is a notification queue and not the home page.
- What a page says about missions and work comes from one captured commit of
  each repository's base branch, never from a checkout, so a stale or detached
  worktree cannot mislead it. The one exception is the overlay of work in
  flight (the board's branches, approvals and findings), which comes from each
  owned work branch's own records, read at a pinned commit of that branch.
  Nothing is read from a working tree, and nothing is written.
- When qualities conflict: correct and current beats complete, read-only beats
  convenient, the T3 host's responsiveness beats page speed, and plain pages
  beat polish.

# Design

**Components.** A new library crate, `crates/atelier-site`, owns the site:
repository discovery, the HTTP routes, the page renderers, the cache and the
load limits. `atelier-cli` gains `atelier serve`, which parses its flags and
calls into the crate, so the site ships in the one `atelier` binary. Dependency
direction: `atelier-cli` → `atelier-site` → `atelier-app`; the site never
depends on the CLI.

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
not the base. Git runs against the
repository's common Git directory with optional locks off, so nothing the site
does writes to a repository. The loader lives in `atelier-app` because it is
a records concern; `Snapshot::at`'s tarball extraction (atelier-3i7x) can move
to it later.

**Discovery.** `atelier serve --root <dir>...` (default `~/projects`) scans
for checkouts holding `.atelier/`, groups them by Git common directory so each
repository appears once, and reads each one's base branch from its workflow
policy at the base commit. The URL slug is the project's `project_slug`; a
repository whose records cannot be read shows its error on its own page while
the others render.

**Routes.** `/` lists repositories with their counts.
`/<repo>/mission/<id>` shows the brief while the mission is in its draft
status and the debrief afterwards, with links to both at
`/<repo>/mission/<id>/brief` and `/debrief`, and to the strategy rendered at
`/<repo>/mission/<id>/strategy`. Later increments add `/<repo>` (the board),
`/<repo>/item/<id>` and `/inbox`.

**Rendering.** The brief and debrief HTML renderers and their helpers
(`crates/atelier-cli/src/review_page.rs`, `render_html` and
`render_brief_html` in `commands/mission_review.rs`) move into `atelier-site`
as pure functions returning a `String`. New pages use the same helpers, so
there is one HTML style and one `escape`. Whole Markdown documents (strategies,
item bodies) render with `pulldown-cmark`, with raw HTML escaped and one
shared URL policy for links and images: relative paths, fragments, `http`,
`https` and `mailto` are kept, and any other scheme (`javascript:`, `data:`,
`vbscript:` and the rest) is rendered as plain text. No front-end
framework, no JavaScript beyond what the existing pages carry.

**Server.** `axum` on a `tokio` runtime, bound to loopback; `tailscale serve`
publishes it beside T3 Code's port. Renders run in `spawn_blocking`, at most
two at a time (a `tower` concurrency limit); a request that waits more than 5
seconds for a slot gets a 503 page. Only work that depends on nothing but the
base commit, such as a parsed record index or a rendered debrief, is cached,
keyed by repository and base commit, and only the latest base commit per
repository is kept. Anything read from other refs (work branches, review
state) is recomputed per request or keyed by every ref it read, and anything
date-dependent by the date. A systemd user unit runs it with
`Restart=on-failure`, `MemoryMax=` and `CPUQuota=`.

**Dependencies chosen.** `axum`, `tokio` and `tower` (decision atelier-c5vk):
maintained, and its layers give the concurrency cap and timeout without
hand-written code; they cost about sixty crates of compile time. `pulldown-cmark`
for Markdown: small, and the standard Rust parser. No template engine: the
existing pages are hand-written HTML, and one style beats rewriting them.

`docs/architecture/` gains a page for the site, written by atelier-zpmb.

# Derisking

Riskiest assumption: the brief, debrief and status views can be built from a
base commit alone, without a checkout, cheaply enough to serve from the host
T3 Code runs on. If it fails, the site needs checkouts of its own or a
different rendering path, and every later page changes.

Planner's spike (2026-11-09, this repository, installed binary from a
checkout): `atelier debrief atelier-k8h8 -q` (48 items) took 7.7 s wall
(5.6 s user, 2.1 s sys) and 64 MB peak RSS; `atelier brief atelier-yfqu -q`
0.13 s; `atelier status` 1.6 s. Reading the code found the checkout reads the Design
lists: records and policy from the working tree, the strategy in two views and
the criteria parser, and `HEAD` pins in the views and receipt history. So the
work is a loader and one captured commit threaded through those sites, and the
cost is real but bounded; a large debrief must be cached by base commit and
capped at two renders.

atelier-zpmb tests it first: it serves brief and debrief pages from the base
commit with a read-only proof and the load bound measured on this host, before
any board or inbox work starts.

# Scope

In: ADR 0030 and the documents it changes; `atelier-site` and `atelier serve`;
brief, debrief, strategy, board, item and inbox pages; the unit file and setup
docs; removing `--html` and `.atelier/runtime/reviews/`, and pointing Agent
Factory and the docs at the site; the validation.

Out: attribution, model metrics and live T3 thread status (mission
atelier-bnek); actions from the site, which need a later decision under ADR
0030; a dashboard inside T3 Code; CLI orientation views (mission atelier-4pzj),
though shared view fixes reach the site; the debrief's content bugs (atelier-jdpf
and its ten items, groom digest atelier-8i86 G8), except that atelier-59fz
rewrites their Done when lines that name the HTML debrief.

Environment: this host only, every Atelier repository under `~/projects`.
Installing and publishing the service (loose end atelier-6rts) and Sumeet's
phone read (loose end atelier-pywh) sit outside the mission.

# Authority

The orchestrator may change items, their order, routes, page layout and design
detail within ADR 0030, decision atelier-c5vk and this Shape. Changing the
Outcome or the criterion, the crate or library choice, keeping `--html`, or
adding write actions to the site needs Sumeet.

# Risks and Unknowns

- A cold debrief of a large mission takes about 8 s. If that is too slow to
  read, warming the cache after the base moves is a later item; zpmb's Report
  records the numbers.
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
