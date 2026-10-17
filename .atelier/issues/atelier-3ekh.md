---
created_at: "2026-10-17T01:10:57.974649958+00:00"
id: "atelier-3ekh"
issue_type: "feature"
labels:
- "cli-output"
- "evidence"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Print a short confirmation after evidence record"
updated_at: "2026-10-17T01:36:28.791554342+00:00"
---

## Goal

`atelier evidence record` prints the whole new receipt through `print_new_record` (crates/atelier-cli/src/commands/evidence.rs:646): Producer/Path/URI `(none)`, Created/Updated, the transcript body, and then the same stdout/stderr again under "Output Summary" (evidence.rs:719). One scratch capture printed about 650 lines. In transcripts, every call was piped to `tail`/`grep`/`head`, sent to `/dev/null`, or run with `-q`: 28 of 28 in this repository and 159 of 159 in astromech-atelier. A failing capture exits 0, and nothing says that the receipt now blocks completion.

## Done when

- A pass prints one line: `✓ atelier-az3w pass test for issue/atelier-nqgr @be418737: grep -q goodbye proof-sample.txt (exit 0)`.
- A fail prints `⚠ atelier-ly6d fail test for issue/atelier-nqgr @be418737 (exit 1); newer than pass atelier-mi12, so atelier-nqgr cannot merge until a later pass`, followed by the last 5 lines of stderr (or stdout).
- Manual receipts print the same single line. `evidence show <id>` is still the full view.
- Tests pin the pass and fail confirmations.

## Notes

- 2026-10-17 [note] From the 2026-10-16 transcript retrospective: because a failing capture exits 0, `atelier evidence record … -- <cmd> && …` chains kept going after a failed test, and in one case the next step's hard-coded summary said the suite passes (atelier repo, nextest exit 100). Decide whether a recorded failing capture should exit nonzero, as distinct from a refused capture.
