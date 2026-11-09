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
updated_at: "2026-11-09T01:04:22.768330108+00:00"
---

## Goal

Independently show the site mission's criterion one-click-inbox holds on this host, with the site installed and published.

## Done when

- [ ] An agent that did not build the site fetches the inbox over the Tailscale hostname, compares its entries with `atelier status` run on each repository's base branch, and finds the same set.
- [ ] Every entry's primary link returns the page the entry asks for with no intermediate page; for each debrief read, the returned HTML is that mission's debrief. The check covers a real needs-human debrief item, adding its `related` link to the mission first if it lacks one.
- [ ] A receipt for one-click-inbox records the commit, the hostname fetched, the counts and any failures.

## Context

Runs after loose end atelier-6rts installs and publishes the site.
