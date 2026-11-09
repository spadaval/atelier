---
created_at: "2026-11-09T01:04:22.768330108+00:00"
id: "atelier-mtb3"
issue_type: "validation"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-6rts"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-ahpf"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-uma8"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate site-matches-cli: fetch every brief, debrief, board and inbox link over Tailscale"
updated_at: "2026-11-09T06:42:34.189248338+00:00"
---

## Goal

Independently show the site mission's criterion site-matches-cli holds on this host, with the site installed and published.

## Done when

- [ ] An agent that did not build the site fetches, over the Tailscale hostname, the brief and debrief page of every mission in every Atelier repository on this host, and each returns 200 and shows the mission's title as `atelier issue show <mission>` gives it, and the criteria and item counts that `atelier brief -q` and `atelier debrief -q` print, all run in a detached worktree of that repository's base branch.
- [ ] For each repository, the board lists the same in-progress, ready and needs-human items as `atelier status` run in a detached worktree of its base branch (created under `$HOME/.cache/` and removed afterwards).
- [ ] Every inbox entry's links return 200.
- [ ] A receipt for site-matches-cli records the commit, the hostname fetched, the counts and any failures.

## Context

Runs after loose end atelier-6rts installs and publishes the site, and after atelier-ahpf and atelier-uma8 merge. A bad result files a linked bug.
