---
created_at: "2026-06-13T00:37:50.973932126+00:00"
id: "atelier-jk6c"
issue_type: "task"
labels:
- "import"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-v9id"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Repair Beads import issue sections"
updated_at: "2026-06-13T01:17:11.416978729+00:00"
---

## Description

Repair the Beads JSONL import path so imported issue Markdown conforms to the
current section contract. The broad suite currently fails because the fixture
import writes `## Acceptance Criteria`, which `atelier lint` and issue parsing
correctly reject as an unknown issue body section.

## Outcome

- `atelier import-beads` maps Beads `description` into the Atelier Description
  section and Beads `acceptance_criteria` into the Atelier Outcome section.
- Imported issues include a concrete Evidence section that names import
  round-trip validation instead of leaving the issue structurally invalid.
- Imported canonical issue Markdown contains only recognized issue sections:
  Description, Outcome, Evidence, and optional Notes.
- The failing broad-suite test `test_import_beads_jsonl_fixture_round_trip`
  passes, and focused import tests assert that `Acceptance Criteria` is not
  emitted as a Markdown section.

## Evidence

- Focused import unit and CLI integration tests for Beads fixture import.
- `cargo nextest run test_import_beads_jsonl_fixture_round_trip` passes.
- `atelier lint`, `atelier export --check`, `git diff --check`, and the
  default broad-suite failure probe pass after the repair.
