---
created_at: "2026-06-12T04:55:44.832214824+00:00"
id: "atelier-igzl"
issue_type: "task"
labels:
- "markdown"
- "validators"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-0j6e"
  - kind: "issue"
    id: "atelier-n1ys"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-v3cw"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T05:22:46.442482975+00:00"
status: "done"
title: "Integrate parsed issue sections across validators and command surfaces"
updated_at: "2026-06-12T05:22:46.442482975+00:00"
---

## Description

Route validators and command surfaces through the parsed issue-section model
instead of letting each command inspect raw Markdown or the legacy description
blob independently.
- Workflow validators can inspect required issue sections through a shared API.
- Lint, work start, issue close, mission closeout, issue show, issue list/status
  summaries, and future transition/options output all use the same parsed
  section data.
- Validators report missing or empty sections with consistent issue IDs, section
  names, and file paths.
- Commands that only need compact summaries can ask the model for section
  presence/emptiness without loading unrelated activity or evidence state.
- No new validator or command surface reintroduces string matching against raw
  Markdown headings.
- Documentation identifies the parsed-section model as the single integration
  point for issue structure.
- Add focused tests for the shared section API used from lint and workflow
  validation.
- Add CLI tests proving lint and work start report the same missing-section
  diagnostics for the same malformed issue.
- Add mission closeout or workflow-validation coverage proving linked issue
  section failures are reported through validators.
- Run focused validator and CLI integration tests.
This task should land after the parser/rendering task and before strict
enforcement. Its purpose is to prevent the new section parser from becoming a
display-only helper.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
