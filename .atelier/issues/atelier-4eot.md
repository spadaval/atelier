---
created_at: "2026-06-12T04:51:30.558747929+00:00"
id: "atelier-4eot"
issue_type: "task"
labels:
- "assignee:root"
- "lint"
- "markdown"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-uibk"
  - kind: "issue"
    id: "atelier-v4u7"
  children: []
  attachments:
  - kind: "evidence"
    id: "atelier-gia8"
    role: "validates"
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-12T05:02:21.466001699+00:00"
status: "done"
title: "Define issue section contract"
updated_at: "2026-06-12T05:02:21.466001699+00:00"
---

## Description

Define the issue-body section contract before implementation changes land. This
task decides the recognized heading names, which sections are required, how
unknown sections behave, and how the parser should report malformed Markdown.

## Outcome

- The canonical issue body contract names `Description`, `Outcome`, `Evidence`,
  and `Notes`.
- Required sections are defined, including whether `Evidence` is required for
  all issues or only for implementation/validation/closeout types.
- The contract explains how headings are matched, how duplicate recognized
  headings fail, how empty sections fail, and whether prose before the first
  recognized heading is allowed.
- The contract keeps large human-authored text in Markdown body sections, not
  YAML frontmatter.
- The contract removes issue-level YAML `acceptance` and `evidence_required`
  arrays from the canonical issue schema; Outcome and Evidence body sections
  replace them as the authoring surface.
- Documentation states that `Outcome` describes the desired finished world and
  `Evidence` describes required proof artifacts.

## Evidence

- Update the relevant architecture or storage documentation with the final
  section contract.
- Add or update parser contract tests before behavior enforcement work starts.
- Run `atelier lint atelier-4eot`.

## Notes

This is the sequencing gate for the implementation tasks. Do not let parser or
lint enforcement make irreversible assumptions before this contract is explicit.
