---
created_at: "2026-11-10T00:56:08.595125805+00:00"
id: "atelier-oci2"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-mlwx"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-nafc"
    type: "blocked_by"
  - kind: "issue"
    id: "atelier-qr5s"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Take the site out of the new workspace: no serve, no web build, and the old site goes at the switch"
updated_at: "2026-11-10T04:45:25.381737978+00:00"
---

## Goal

Take the site off the cutover's path. The human's pages become a product of their own, the Desk (ADR 0032, mission atelier-z4cr), so the new workspace neither builds nor serves a site and the old one is deleted with the old workspace instead of being ported.

## Done when

- [ ] Provisional: nothing in `next/` builds or serves a site: there is no `atelier-site` crate and no `serve` command, and `--help` lists neither. The web typecheck, unit and Playwright jobs and the Node build step are gone from the CI and release workflows that the switch puts in place. The old workspace's site is not changed before the switch.
- [ ] Provisional: no instruction in the new skill or in the `AGENTS.md` texts the cutover installs tells an agent to send the human a site page; a mission is read with `atelier show <mission>` until the Desk's mission page is live.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Changed on 2026-11-11 at Sumeet's direction: it was "rebuild the site on the new records", and porting the old site would have spent the cutover's time on pages the Desk replaces. Target design: `docs/strategies/redesign-architecture.md`, Site.
