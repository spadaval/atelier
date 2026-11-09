---
created_at: "2026-11-09T00:45:58.669636931+00:00"
id: "atelier-6rts"
issue_type: "loose_end"
labels:
- "fleet-dashboard"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-yn69"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run atelier serve as a user service and publish it on the tailnet"
updated_at: "2026-11-09T14:32:38.068835559+00:00"
---

## Goal

Install `atelier serve` on this host from the reviewed unit file and publish it on the tailnet, so the site is reachable from Sumeet's phone.

## Done when

- [ ] The installed `atelier` matches `master`, the shipped user unit is installed, enabled and active (`systemctl --user is-enabled` and `is-active`), and the running service was started after that install.
- [ ] After `systemctl --user kill -s KILL` on the service, it is active again within 30 seconds and `systemctl --user show -p NRestarts` has gone up by one. No host reboot.
- [ ] `tailscale serve status` shows the site on a port or path that leaves T3 Code's published port unchanged, and `/` and one mission's debrief page, loaded in a browser over the Tailscale hostname, render the app with the mission's title.

## Context

Can only happen after atelier-yn69 merges and the binary is installed with the app embedded. Steps are in the docs that item ships. Later site increments need only `cargo install` and a service restart, which their own validation does.
