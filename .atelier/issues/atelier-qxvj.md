---
created_at: "2026-06-10T16:00:59.223312574+00:00"
id: "atelier-qxvj"
issue_type: "task"
labels:
- "activity"
- "assignee:root"
- "markdown"
- "record-store"
- "storage"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1z0u"
  - kind: "issue"
    id: "atelier-6kkz"
  - kind: "issue"
    id: "atelier-8o8v"
  - kind: "issue"
    id: "atelier-nwug"
  - kind: "issue"
    id: "atelier-t79u"
  - kind: "issue"
    id: "atelier-ujm4"
  - kind: "issue"
    id: "atelier-yvk6"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-10T17:25:44.417915345+00:00"
status: "done"
title: "Define canonical issue activity sidecars"
updated_at: "2026-06-10T17:25:44.417915345+00:00"
---

## Description

Implement the canonical issue activity sidecar model.

What:
- Add support for `.atelier-state/issues/<issue-id>.activity/<timestamp-id>.md` activity files.
- Generate UTC IDs in `YYYYMMDDTHHMMSSffffffZ` form, adding `-01`, `-02`, etc. only for same-timestamp collisions.
- Store typed front matter with `schema: atelier.activity`, `schema_version: 1`, `id`, `subject_kind: issue`, `subject_id`, `event_type`, `actor`, `created_at`, and `summary`.
- Keep human text or event details in the markdown body.
- Support V1 event types: `comment`, `note`, `handoff`, `plan`, `close_reason`, `status_changed`, `field_changed`, `work_started`, `work_finished`, and `evidence_attached`.

Out of scope:
- The `atelier history` CLI rendering/query surface.
- SQLite comment migration.
- Activity entries round-trip through structured parsing/serialization.
- Collision handling refuses overwrites and produces deterministic suffixes for same-timestamp collisions.
- Invalid schema versions, subject kinds, and event types are rejected with useful diagnostics.
- Focused tests cover ID generation, sidecar paths, front matter, body handling, and collision behavior.

Recommended subskill: agent-factory implement.
Evidence was not specified in the legacy issue record.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
