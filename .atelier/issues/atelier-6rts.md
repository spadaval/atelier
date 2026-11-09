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
updated_at: "2026-11-09T00:45:58.669636931+00:00"
---

## Goal

Run `atelier serve` on this host as a systemd user service and publish it on the tailnet with `tailscale serve`, beside T3 Code's own published port, so the site is reachable from Sumeet's phone.

## Done when

- [ ] A systemd user unit runs the installed `atelier serve`, restarts it on failure, and survives a reboot (`systemctl --user is-enabled` and `is-active`).
- [ ] `tailscale serve status` shows the site on a port or path that does not disturb T3 Code's, and fetching the inbox over the Tailscale hostname from another tailnet device returns 200.
- [ ] The unit file and the commands are written down in `docs/` (or the README) so the setup can be repeated.

## Context

Can only happen after the serve item merges and the new binary is installed.
