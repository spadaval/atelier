---
created_at: "2026-11-09T00:45:57.707570089+00:00"
id: "atelier-yfqu"
issue_type: "mission"
labels:
- "fleet-dashboard"
strategy:
  path: docs/strategies/fleet-dashboard.md
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "See where every Atelier repository on this host stands, from one site"
updated_at: "2026-11-09T03:58:08.021551790+00:00"
---

## Outcome

A site on this host, reached over Tailscale, shows Sumeet every Atelier repository here: an inbox of everything waiting on him, each with one link that opens what he must read, each repository's state (waiting on him, in progress, up next, recently done), and each mission's strategy, items and debrief. It reads only Git, so he no longer has to ask an agent where a project stands. See `docs/strategies/fleet-dashboard.md`. Attribution and model metrics are mission atelier-bnek.

## Success criteria

- one-click-inbox: with the site installed and published on this host, the inbox fetched over the Tailscale hostname lists exactly the open items across this host's Atelier repositories that `atelier status` on each base branch reports as needing a human, and each entry's primary link returns the page that entry asks him to read (for a debrief read, that mission's rendered debrief) with no intermediate page. Shown by validation item atelier-mtb3.

## Background

Split on 2026-11-08 at Sumeet's direction, after planning review finding atelier-i8h1, from the first draft that also held attribution and metrics. Sumeet's own read of the site on his phone is loose end atelier-pywh, which does not hold this mission open (finding atelier-gnh5).

## Notes

- 2026-11-08 [plan] Revised after planning review (findings atelier-b9wq, i8h1, 79wo, gnh5, bihd, 5yz1, roov, ntq9, 8j0j, 726p). On Sumeet's authority (2026-11-08) split attribution and metrics into mission atelier-bnek, and moved acceptance to independent validation items (atelier-mtb3 here, atelier-wo22 there) with his phone read as loose end atelier-pywh. Other fixes: ADR item reconciles architecture docs; serve item gains direct debrief links, a pure renderer, an unwritable-repository proof, a load bound and the unit file; the service loose end keeps only install and publish; attribution records item reads, explicit roles, and a write-only store; the hook keeps the light path; metrics validates the attribution rule on real threads first.
- 2026-11-08 [plan] Fixed re-review findings atelier-bihd and atelier-mhb4: a new reads link role names an inbox item's reading target (membership and related links never do), and validation adds reads links to existing debrief reads such as atelier-3km4; only base-derived work is cached by base commit, branch and review state is recomputed or keyed by its refs, date sections by the date, with a warm-cache fixture on atelier-ahpf.
- 2026-11-08 [note] 2026-11-08, Sumeet's direction for the replan (not yet applied to the strategy or items): the site is agent Jira, not agent Gmail. Primary: every mission's brief and debrief as pages, current from the base branch, replacing ad-hoc artifacts; the CLI's --html output and .atelier/runtime/reviews/ go away, while Markdown brief and debrief stay for agents. Second: a backlog board per repository (missions with progress, all open work by mission, label and status, recently done) answering where are we, what's done, what's next. The inbox is a notification queue linking into those pages, not the home page. Proposed by Claude and not yet decided: the site lives in this workspace as a new crate (crates/atelier-site) shipped in the atelier binary as atelier serve, Rust with axum and maud. Replan this mission under the planner procedure from mission 'missions are a human review boundary' once it lands.
