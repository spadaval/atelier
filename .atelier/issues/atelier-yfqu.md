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
  relates:
  - kind: "issue"
    id: "atelier-c5vk"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "draft"
title: "Read every mission's brief and debrief, and each repository's board, on one site for every Atelier repository here"
updated_at: "2026-11-09T06:42:58.238297051+00:00"
---

## Outcome

A site on this host, reached over Tailscale, shows every Atelier repository here. Every mission's brief and debrief is a page, current from its repository's base branch, and agents send Sumeet that page instead of writing an HTML file or publishing an artifact. Each repository has a board answering where are we, what's done and what's next, and an inbox lists what waits on him and links into those pages. The CLI's `--html` output and `.atelier/runtime/reviews/` are gone; the Markdown brief and debrief stay for agents. See `docs/strategies/fleet-dashboard.md`; decision atelier-c5vk settles the crate and libraries. Attribution and model metrics are mission atelier-bnek.

## Success criteria

- site-matches-cli: with the site installed and published on this host, fetched over the Tailscale hostname, every mission of every Atelier repository here has a brief and a debrief page that return 200 and show the title, criteria count and item count that `atelier brief -q` and `atelier debrief -q` give on that repository's base branch; each repository's board lists the same in-progress, ready and needs-human items as `atelier status` on its base branch; and every inbox link returns 200. Shown by validation item atelier-mtb3.

## Background

Split on 2026-11-08 at Sumeet's direction, after planning review finding atelier-i8h1, from the first draft that also held attribution and metrics. Replanned on 2026-11-09 from his 2026-11-08 direction (note below): brief and debrief pages first, a board per repository second, the inbox as a notification queue, `--html` removed. Installing and publishing the service is loose end atelier-6rts, and Sumeet's phone read is loose end atelier-pywh; neither holds this mission open (finding atelier-gnh5).

## Notes

- 2026-11-08 [plan] Revised after planning review (findings atelier-b9wq, i8h1, 79wo, gnh5, bihd, 5yz1, roov, ntq9, 8j0j, 726p). On Sumeet's authority (2026-11-08) split attribution and metrics into mission atelier-bnek, and moved acceptance to independent validation items (atelier-mtb3 here, atelier-wo22 there) with his phone read as loose end atelier-pywh. Other fixes: ADR item reconciles architecture docs; serve item gains direct debrief links, a pure renderer, an unwritable-repository proof, a load bound and the unit file; the service loose end keeps only install and publish; attribution records item reads, explicit roles, and a write-only store; the hook keeps the light path; metrics validates the attribution rule on real threads first.
- 2026-11-08 [plan] Fixed re-review findings atelier-bihd and atelier-mhb4: a new reads link role names an inbox item's reading target (membership and related links never do), and validation adds reads links to existing debrief reads such as atelier-3km4; only base-derived work is cached by base commit, branch and review state is recomputed or keyed by its refs, date sections by the date, with a warm-cache fixture on atelier-ahpf.
- 2026-11-08 [note] 2026-11-08, Sumeet's direction for the replan (not yet applied to the strategy or items): the site is agent Jira, not agent Gmail. Primary: every mission's brief and debrief as pages, current from the base branch, replacing ad-hoc artifacts; the CLI's --html output and .atelier/runtime/reviews/ go away, while Markdown brief and debrief stay for agents. Second: a backlog board per repository (missions with progress, all open work by mission, label and status, recently done) answering where are we, what's done, what's next. The inbox is a notification queue linking into those pages, not the home page. Proposed by Claude and not yet decided: the site lives in this workspace as a new crate (crates/atelier-site) shipped in the atelier binary as atelier serve, Rust with axum and maud. Replan this mission under the planner procedure from mission 'missions are a human review boundary' once it lands.
- 2026-11-09 [plan] Replanned on Sumeet's authority (his 2026-11-08 direction above), because it changes the Outcome, criterion and page order, which the old Authority reserved to him. Strategy rewritten in place with Sizing, Design and Derisking. Decision atelier-c5vk settles the crate (crates/atelier-site in the atelier binary), axum/tokio/tower, the existing HTML helpers rather than maud, pulldown-cmark, and an in-memory commit-tree record loader in atelier-app. Criterion one-click-inbox replaced by site-matches-cli; the planned reads link role is dropped. Items: atelier-zpmb rewritten as the first, riskiest increment (brief and debrief pages from the base commit) and no longer blocked by atelier-tu8c; atelier-tu8c valid, lowered to medium, now also covers the --html paragraph of PRODUCT_INTENT.md; atelier-ahpf rewritten as the board and item pages, moved to backlog past zpmb's evidence boundary; new atelier-uma8 (inbox, backlog) and atelier-59fz (remove --html, backlog, after loose end atelier-6rts); atelier-mtb3 rewritten for the new criterion; loose ends atelier-6rts and atelier-pywh revised. Mission atelier-bnek's atelier-wpj2 names the item, repository and inbox pages, which now come from atelier-ahpf and atelier-uma8; its replan should move that blocker.
