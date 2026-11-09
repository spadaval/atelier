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
title: "See every Atelier repository and which agent did what from one site on this host"
updated_at: "2026-11-09T00:45:57.707570089+00:00"
---

## Outcome

A site on this host, reached over Tailscale, shows Sumeet every Atelier repository here: an inbox of everything waiting on him with one-click links to what he must read, each repository's state, and each mission's strategy, items and debrief. Every `atelier` command run in a T3 thread records which thread, harness, provider, model, effort and role did it, so the site can show who is working on what and report work, review outcomes and token cost by model and role. See `docs/strategies/fleet-dashboard.md`.

## Success criteria

- one-click-inbox: from a device on the tailnet, the inbox lists every open item across this host's Atelier repositories that waits on the human, and each entry opens the page he needs in one click. Shown by an agent fetching the inbox and every linked page over the Tailscale hostname, plus Sumeet's read on his phone.
- who-did-what: for one real item implemented by a Claude model and reviewed by a Codex model after attribution lands, the item's page names each thread's provider, model, effort and role, links each thread into T3, and gives each role's token cost. Shown by an agent comparing the page with T3's thread records and the provider transcripts.
