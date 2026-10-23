---
created_at: "2026-10-17T19:00:36.354527404+00:00"
id: "atelier-hulj"
issue_type: "task"
labels:
- "retro-2026-10-17"
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Stop review records repeating their body in the title"
updated_at: "2026-10-23T14:32:24.016203099+00:00"
---

## Goal

Review records take 1.17 MB in the window and each record's `title` field repeats its body text.

## Done when

New finding, disposition and approval records carry a short title (or none) and the text once. `review show` and `review list` still render the same information. A test pins the record shape.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Title = summary first line (crates/atelier-records/src/review.rs:266,298); 1147/1149 records repeat the body. Decided on atelier-rc8j: generate the title from event kind, owner and short pin (no schema change); do not rewrite existing records.
- 2026-10-23 [decision] Generated titles read '<event> on <owner> @<pin8>', and a base exemption's '<event> of <base> @<pin8>'; ReviewEvent::name in atelier-core gives the stored name to records and CLI.
  Reason: Follows the mission decision (kind, owner, short pin) and keeps the records crate free of CLI code.
