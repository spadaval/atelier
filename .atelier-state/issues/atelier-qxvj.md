---
acceptance: []
blocks:
- "atelier-1z0u"
- "atelier-6kkz"
- "atelier-8o8v"
- "atelier-nwug"
- "atelier-t79u"
- "atelier-ujm4"
- "atelier-yvk6"
created_at: "2026-06-10T16:00:59.223312574+00:00"
depends_on: []
evidence_required: []
id: "atelier-qxvj"
issue_type: "task"
labels:
- "activity"
- "markdown"
- "record-store"
- "storage"
links: []
parent: "atelier-r4cf"
priority: "P1"
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Define canonical issue activity sidecars"
updated_at: "2026-06-10T16:00:59.223312574+00:00"
---

Implement the canonical issue activity sidecar model.

What:
- Add support for `.atelier-state/issues/<issue-id>.activity/<timestamp-id>.md` activity files.
- Generate UTC IDs in `YYYYMMDDTHHMMSSffffffZ` form, adding `-01`, `-02`, etc. only for same-timestamp collisions.
- Store typed front matter with `schema: atelier.activity`, `schema_version: 1`, `id`, `subject_kind: issue`, `subject_id`, `event_type`, `actor`, `created_at`, and `summary`.
- Keep human text or event details in the markdown body.
- Support V1 event types: `comment`, `note`, `handoff`, `decision`, `plan`, `close_reason`, `status_changed`, `field_changed`, `work_started`, `work_finished`, and `evidence_attached`.

Out of scope:
- The `atelier history` CLI rendering/query surface.
- SQLite comment migration.

Acceptance criteria:
- Activity entries round-trip through structured parsing/serialization.
- Collision handling refuses overwrites and produces deterministic suffixes for same-timestamp collisions.
- Invalid schema versions, subject kinds, and event types are rejected with useful diagnostics.
- Focused tests cover ID generation, sidecar paths, front matter, body handling, and collision behavior.

Recommended subskill: agent-factory implement.
