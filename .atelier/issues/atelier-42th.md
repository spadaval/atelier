---
created_at: "2026-11-07T01:11:19.111580398+00:00"
id: "atelier-42th"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-hwxm"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-11T03:55:48.071955467+00:00"
status: "declined"
title: "Remove the read path for separate disposition events"
updated_at: "2026-11-11T03:55:48.071955467+00:00"
---

## Goal

Contract ADR 0025's expand-and-contract: once no repository the shared installed atelier serves holds separate disposition events, stop reading them as folded into their findings and remove `atelier migrate dispositions`.

## Done when

Every repository using the shared binary (atelier, astromech, demiurge, cosplay-studio, life-on-rails, and any adopted since) has committed `atelier migrate dispositions --apply` on its base and no open work branch there holds a disposition event; then the fold in record loading, the retired-event lookups in the gates, prune and merge, and the migrate subcommand are gone, a separate disposition event in a checkout fails to load naming the last release that reads it, history that holds them still parses, and ADR 0025 says the read path is removed.

## Context

Trigger and owner per ADR 0025 decision 6. History keeps old events, so `parse_review_file` keeps parsing them for the append-only and tracing checks.

## Report

Dropped in the cutover sort (atelier-5zre), reason (a). It removes `migrate dispositions` and the old review events' read path, and the What is removed table removes `migrate` and the old review subcommands.
