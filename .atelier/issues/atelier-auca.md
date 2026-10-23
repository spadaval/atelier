---
created_at: "2026-10-17T19:00:36.342496349+00:00"
id: "atelier-auca"
issue_type: "bug"
labels:
- "evidence"
- "retro-2026-10-17"
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
closed_at: "2026-10-23T14:28:11.190755860+00:00"
status: "done"
title: "Stop evidence records ending with a trailing blank line"
updated_at: "2026-10-23T14:28:11.190755860+00:00"
---

## Goal

New evidence records end with an extra blank line at EOF. Reviewers raised it as a finding three times (7u4o, ihns, deyy), each time costing a disposition.

## Done when

Records written by `evidence record` end with exactly one newline; `git diff --check` is clean on them. A test pins it.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. 533 of 590 receipts end with a blank line: the transcript keeps its final newline (evidence.rs:368 -> lib.rs:2030) and render_evidence_body returns it untrimmed (lib.rs:2013-2016).
- 2026-10-23 [decision] The evidence codec, not just capture, writes exactly one final newline, so an old receipt rewritten for another reason (such as attach) also loses its trailing blank line; the compatibility fixtures are verbatim copies exempted from whitespace checks in .gitattributes.
  Reason: Manual and bundle-imported bodies can end in a newline too; one rule in the codec covers every writer, and parse/render stays a fixed point.
  Rejected: Trimming only the capture transcript, which preserves old bodies byte for byte but leaves other writers able to produce a blank last line

## Close Reason

Evidence records now end with exactly one newline: the evidence codec trims the body's trailing newlines before writing the final one, so new receipts pass `git diff --check` (commit fc115d5b; an integration test runs `git diff --cached --check` over freshly captured pass and fail receipts). Evidence atelier-x7ws, full suite atelier-2c5x.

### Deviations
None.

### Decisions
The fix lives in the codec, so an old receipt rewritten for another reason also loses its blank last line; verbatim compatibility fixtures are exempted in `.gitattributes`. See this item's decision note.

### Surprises
An existing records test pinned the trailing blank line (`ends_with("```\n\n")`); it now asserts exactly one newline.

### Open items
None.
