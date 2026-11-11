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

Take the tracker's site off the cutover's path. The human's pages become a product of their own, the Desk (ADR 0032, mission atelier-z4cr), so the `atelier` binary built from the new workspace has no site and no `serve`, and the old site is deleted with the old workspace instead of being ported. The Desk's own crate, app and CI jobs in the new workspace are not this item's to remove.

## Done when

- [ ] Provisional: the `atelier` binary built from `next/` has no site: there is no `atelier-site` crate, `atelier --help` lists no `serve`, and building or releasing `atelier` needs neither Node nor pnpm. The CI and release workflows the switch puts in place have no job or build step for the old site's app (`crates/atelier-site/web`). The old workspace's site is not changed before the switch.
- [ ] Provisional: this removes only the tracker's site. The Desk's crate (`atelier-desk`) and app (`desk/`), where mission atelier-z4cr has added them, keep their own build and their own CI jobs, and those jobs do not gate a change that touches neither.
- [ ] Provisional: no instruction in the new skill or in the `AGENTS.md` texts the cutover installs tells an agent to send the human a site page; a mission is read with `atelier show <mission>` until the Desk's mission page is live.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-3-cutover.md` says. Changed on 2026-11-11 at Sumeet's direction: it was "rebuild the site on the new records", and porting the old site would have spent the cutover's time on pages the Desk replaces. Target design: `docs/strategies/redesign-architecture.md`, Site.
