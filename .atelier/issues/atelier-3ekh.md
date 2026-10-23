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
    id: "atelier-rc8j"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Print a short confirmation after evidence record"
updated_at: "2026-10-23T16:08:34.399758555+00:00"
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
- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done by policy: human-cli-output.md Verdict Line now names evidence record. print_new_record (evidence.rs:761) still prints the full 48-line record, with streams twice. Decided on atelier-rc8j: a recorded fail exits 0 (recording succeeded) and the verdict line names the result; quiet output stays the ID.
- 2026-10-23 [decision] The confirmation marks the result word (atelier-az3w ✓ pass ...), shows the command on fail lines too, and moves the fail's consequence to the verdict line: evidence record: recorded fail — <item> cannot close or merge until a later pass
  Reason: Dense Output puts a symbol beside the status word, not before the ID; the verdict is the last line, so | tail -1 reads the consequence, and the line 1 clause would say it twice. newest_failure blocks the done transition as well as merge, so 'cannot merge' alone understated it.
  Rejected: The Done-when's literal line 1 (⚠ <id> fail ... (exit 1); newer than pass <p>, so <item> cannot merge until a later pass) with a bare verdict
- 2026-10-23 [decision] A capture that refuses to run (dirty checkout, HEAD changed, bad target) stays an error on stderr with no 'evidence record: refused' verdict line
  Reason: Capture refusals have no check codes for the verdict line's 'refused — n checks (codes)' form, and the Done when covers recorded results only.
  Rejected: Printing 'evidence record: refused — <message>' without codes
