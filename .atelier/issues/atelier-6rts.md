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
    id: "atelier-zpmb"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run atelier serve as a user service and publish it on the tailnet"
updated_at: "2026-11-09T01:04:24.463133101+00:00"
---

## Goal

Install `atelier serve` on this host from the reviewed unit file and publish it on the tailnet, so the site is reachable from Sumeet's phone.

## Done when

- [ ] The installed `atelier` matches `master`, and the shipped user unit is installed, enabled and active (`systemctl --user is-enabled` and `is-active`).
- [ ] After `systemctl --user kill -s KILL` on the service, it is active again within 30 seconds and `systemctl --user show -p NRestarts` has gone up by one. No host reboot.
- [ ] `tailscale serve status` shows the site on a port or path that leaves T3 Code's published port unchanged, and the inbox fetched over the Tailscale hostname returns 200.

## Context

Can only happen after atelier-zpmb merges and the binary is installed. Steps are in the docs that item ships.
