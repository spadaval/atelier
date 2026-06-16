---
created_at: "2026-06-15T21:37:24.795700733+00:00"
id: "atelier-3gr9"
issue_type: "feature"
labels:
- "cli"
- "evidence"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9p3t"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Surface evidence requirements in status commands"
updated_at: "2026-06-15T21:37:24.795700733+00:00"
---

## Description

Make status-like commands show what evidence exists, what evidence is missing, and where to record proof before an agent attempts a terminal transition.

## Outcome

- Root status, mission status, issue show, and issue transition options surface evidence state in bounded human-readable sections.
- Output distinguishes attached proof, missing required proof, evidence that is present but failing or irrelevant, and the command to record or attach the right proof.
- Evidence information is shown before terminal transitions fail, not only after a blocked close or complete attempt.
- The implementation reuses workflow/evidence gate logic and does not add red-tape forms or duplicate evidence contracts.

## Evidence

- CLI transcripts cover root status, mission status, issue show, and `issue transition --options` for records with complete proof, missing proof, failing proof, and irrelevant proof.
- Focused CLI tests assert bounded evidence summaries and next commands for recording or attaching proof.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant cargo tests pass.
