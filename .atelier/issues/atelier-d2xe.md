---
created_at: "2026-10-17T01:10:57.983690051+00:00"
id: "atelier-d2xe"
issue_type: "bug"
labels:
- "cli-output"
- "status"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-4pzj"
    type: "advances"
  - kind: "review"
    id: "atelier-5pwb"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:36.957640893+00:00"
status: "declined"
title: "Show every scope item, in plain review and evidence words, in status's owner block"
updated_at: "2026-11-11T03:55:36.957640893+00:00"
---

## Goal

status's owner block (crates/atelier-cli/src/commands/status.rs, `Owner`), which took over `prime`'s in atelier-sdb0, still has rendering problems (checked 2026-11-07):
- Owner Work lists only active and blocked scope items, so done and superseded items in the owner's scope are hidden.
- Evidence rows print the result with Rust `Debug` formatting (`recorded {:?}`), e.g. `recorded Pass`.

## Done when

- [ ] Every scope item appears in Owner Work, one line each with its exact status.
- [ ] Review and evidence rows use CLI words (`approval`, `finding`, `trivial exemption`, `passed`, `failed`), each cut to one line.
- [ ] Tests assert a done child listed and no `Debug`-style variant name in the owner block.

## Context

The empty `blockers:` suffix the original reported is gone. Decision atelier-mzpn pointed this at status instead of `prime` (retired by atelier-sdb0); review words follow atelier-md0v's smaller vocabulary, where a `fix_now` finding has no disposition and only `defer` and `no_action` record a decision.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It changes evidence and review rows in the old `status` owner block, and the What is removed table removes evidence records.
