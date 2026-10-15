---
created_at: "2026-10-15T18:45:05.853636797+00:00"
id: "atelier-4xyr"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-15T19:06:43.467536517+00:00"
status: "done"
title: "Make advances direction agree across bundle, CLI and docs"
updated_at: "2026-10-15T19:06:43.467536517+00:00"
---

## Goal

One model for the `advances` relationship, accepted by both bundle v2 and `issue link`, and documented the same way.

## Done when

Tests show bundle v2 and `atelier issue link` both accept the chosen direction and reject or normalise the other consistently; docs and help text show the same direction.

## Scope

Bundle validation, issue link, docs and skill reference text for advances.

## Context

Bundle v2 rejects advances on an epic ("only mission issue resources can declare advances") while docs show `atelier issue link <epic-id> <mission-id> --role advances`, which works.

## Close Reason

Model: advances joins exactly one mission to one non-mission issue and is stored on both records; either endpoint may declare it. Bundle v2 accepts advances on work issues and stores the link on both records, as issue link does. Both refuse non-mission pairs. Docs updated. Test advances_joins_a_mission_and_work_the_same_way_from_bundle_and_cli covers both entry points.
