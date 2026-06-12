---
created_at: "2026-06-12T19:18:46.007776918+00:00"
id: "atelier-gjaz"
issue_type: "epic"
labels:
- "markdown"
- "mission"
- "records"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-6aor"
  - kind: "issue"
    id: "atelier-7r55"
  - kind: "issue"
    id: "atelier-8ec6"
  - kind: "issue"
    id: "atelier-sxyy"
  - kind: "issue"
    id: "atelier-ys5p"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "open"
title: "Replace escaped mission data JSON with readable mission records"
updated_at: "2026-06-12T19:29:26.567235806+00:00"
---

## Description

Mission records currently store constraints, risks, validation criteria, and
related mission metadata as escaped JSON inside YAML front matter. That makes
canonical Markdown technically durable but hostile to review, hand editing,
merge conflict resolution, and mission closeout audits.

Replace the generic mission `data` blob with a readable mission record contract
and a mission-specific internal model. The target record should keep compact
identity/state metadata in front matter, put user-authored mission content where
normal Markdown review works, and preserve rebuildable projection behavior.

This epic coordinates the record contract, typed mission model, relationship
semantics, migration, and end-to-end validation as separate child issues.

## Outcome

- The readable mission record contract is documented before implementation.
- Mission parsing, rendering, command behavior, rebuild, export/check, lint, and
  projection refresh use a mission-specific model instead of mission semantics
  hidden in generic escaped JSON.
- Mission relationship semantics distinguish work, blockers, validation,
  evidence, checkpoints, and supporting records explicitly.
- Existing mission records are migrated directly to the new shape, with no
  writer that keeps producing escaped mission `data` JSON.
- Product and architecture docs explain where mission narrative, constraints,
  risks, validation expectations, relationships, and evidence links live.
- Child issues close with focused proof; this epic closes only after their
  evidence shows the full mission-record workflow is coherent.

## Evidence

- All child issues are closed with linked evidence.
- Final closeout includes focused tests, migration transcript, representative
  mission command output, `atelier lint`, `atelier export --check`, and
  `atelier doctor`.

## Notes

The parent epic should stay small. The child issues own the contract,
implementation, migration, relationship cleanup, and validation details.
