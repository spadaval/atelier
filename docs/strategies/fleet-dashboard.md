---
status: draft
last_changed: "2026-11-08: reads links choose inbox targets; cache only base-derived work (planning re-review)"
---

# Why

Sumeet can only learn where his projects stand by asking an agent, and he asks
"where are we on this project?" in nearly every session. Documents written for
him, such as a mission's HTML debrief, land in `.atelier/runtime/reviews/`
inside whichever checkout generated them, where his phone cannot reach them and
he has to know which worktree to look in.

# Outcome

A site on this host, reached over Tailscale, shows Sumeet every Atelier
repository here: an inbox of everything waiting on him, each with one link that
opens what he must read; each repository's state (waiting on him, in progress,
up next, recently done); and each mission's strategy, items and debrief. It
reads only Git.

# Success Criteria

- one-click-inbox: with the site installed and published, the inbox fetched
  over the Tailscale hostname lists exactly the items each repository's
  `atelier status` lists as needing a human, and each entry's primary link
  returns its reading target with no intermediate page. Shown by an
  independent validation item; Sumeet's phone read is a separate loose end
  that does not hold the mission open.

# Shape

- Decision atelier-7bs1 and ADR 0030 (written by the first item) change
  `PRODUCT_INTENT.md` and the architecture docs: Atelier may have a human UI
  and a long-running read-only server, may keep a host-wide attribution
  database, and may depend on T3 Code. Records in Git stay the only source of
  truth for work, and guard decisions consult only Git.
- **Server.** `atelier serve` renders HTML on request, read-only, run as a
  systemd user service and published with `tailscale serve` beside T3 Code's
  published port. It reads each repository's records from its base branch ref,
  never from a checkout, so a stale or detached worktree cannot mislead it.
  Pages are server-rendered with no front-end framework and reuse the app
  layer's views. The debrief gets a pure rendering function; the CLI keeps
  writing its file from it.
- **Staying light.** This host runs the T3 Code server Sumeet's phone depends
  on. Only work that depends on nothing but the base commit, such as the
  debrief, is cached by base commit; branch and review state is recomputed or
  keyed by the refs it read, and date-dependent sections by the date. Renders
  are capped at two at a time, and the unit sets `MemoryMax=` and `CPUQuota=`.
- **Repositories** are found by scanning configured roots (default
  `~/projects`) for `.atelier/`, counting worktrees of one repository once by
  Git common directory.
- **Inbox targets.** An item names what the human should read with a new
  `reads` link to a mission, whose debrief the inbox opens directly. Membership
  and context links never choose a target; without a `reads` link the inbox
  opens the item's own page.
- When qualities conflict: correct and current beats complete, read-only beats
  convenient, the T3 host's responsiveness beats page speed, and plain pages
  beat polish.

# Scope

In: ADR 0030 and the documentation it changes, `atelier serve`, its inbox,
item, debrief, repository and mission pages, its unit file and setup docs,
installing it on this host, and the validation.

Out: attribution, model metrics and live T3 thread status (mission
`docs/strategies/agent-attribution.md`); actions from the site, which need a
later decision under ADR 0030; a dashboard inside T3 Code, which waits for T3
extensions; CLI orientation views for agents (mission atelier-4pzj), though its
shared view fixes, such as atelier-om6l's Waiting on, reach the site too.

Environment: this host only. Repositories: every Atelier repository under
`~/projects`.

# Authority

The orchestrator may change items, their order and design detail within ADR
0030 and this Shape. Changing the Outcome or the criterion, or adding write
actions to the site, needs Sumeet.

# Risks and Unknowns

- The debrief walks Git history and is the expensive page; the load bound in
  atelier-zpmb is measured against this repository's largest mission.
- Port or path conflicts with T3 Code's own `tailscale serve` setup; the
  service loose end must leave T3's published port unchanged.
