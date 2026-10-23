---
created_at: "2026-10-23T15:01:14.879470764+00:00"
id: "atelier-3xq4"
issue_type: "bug"
labels: []
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
closed_at: "2026-10-23T15:35:23.680123195+00:00"
status: "done"
title: "Rerun receipts from merge --sync nest the old receipt's whole body in their summary"
updated_at: "2026-10-23T15:35:23.680123195+00:00"
---

## Goal

`atelier merge <id> --sync` reruns current command-backed receipts with the summary `<old summary> (rerun after merging <base>)`, but the old summary it reads (`record.summary` in `crates/atelier-cli/src/commands/merge_sync.rs:316`) is the whole receipt body: Summary, Command, Exit status and stream sections. The rerun receipt then nests the full old body inside its own Summary, every later sync nests it again, and field-like lines such as `Test summary:` inside it can be read as the new receipt's fields.

## Done when

A CLI test syncs a branch with a current captured receipt and asserts the rerun receipt's Summary section is the old Summary text plus the rerun suffix, with no Command or stream lines.

## Context

Seen 2026-10-23 on feature/atelier-h1nc: rerun receipts atelier-7t38 and atelier-w253 (commit 80953e4b) carry atelier-8z2v's whole body in their summary. Check whether `EvidenceRecord.summary` holds the body by design (then take the Summary section) or the parser is wrong since atelier-dg3h.

## Notes

- 2026-10-23 [decision] EvidenceRecord.summary holds the whole body by design; the sync takes the Summary section through atelier_records::evidence_summary_text
  Reason: render_evidence_record_file writes summary back as the body, so a captured receipt round-trips byte for byte; the parser is right and the sync read the wrong field for a summary.
  Rejected: Changing the parser to store only the Summary section (breaks round-tripping of every committed receipt)

## Close Reason

merge --sync rerun receipts now carry the old receipt's Summary text plus ` (rerun after merging <base>)`, with no nested Command or stream sections; a CLI test failed on the nesting before the fix and passes after it.

### Deviations
None.

### Decisions
EvidenceRecord.summary holds the whole body by design, so the parser is unchanged and the sync reads the Summary section through the new atelier_records::evidence_summary_text (decision note on atelier-3xq4).

### Surprises
None.

### Open items
None. Repeated syncs still append another rerun suffix to a summary that already has one (proposed as a follow-up in the handoff, not filed).
