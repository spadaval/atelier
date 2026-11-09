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
    id: "atelier-yfqu"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Validate one-click-inbox: fetch the published inbox and every link over Tailscale"
updated_at: "2026-11-09T01:10:59.475283494+00:00"
---

## Goal

Independently show the site mission's criterion one-click-inbox holds on this host, with the site installed and published.

## Done when

- [ ] An agent that did not build the site fetches the inbox over the Tailscale hostname, compares its entries with `atelier status` run on each repository's base branch, and finds the same set.
- [ ] Every entry's primary link returns the page the entry asks for with no intermediate page; for each debrief read, the returned HTML is that mission's debrief. Before checking, every open needs-human item on this host that asks the human to read a debrief has exactly one `reads` link to that mission (for example atelier-3km4 reads atelier-vt8q, whose only mission link today is `related` atelier-gg6b); the Report lists the links added.
- [ ] A receipt for one-click-inbox records the commit, the hostname fetched, the counts and any failures.

## Context

Runs after loose end atelier-6rts installs and publishes the site.
