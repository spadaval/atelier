---
created_at: "2026-10-17T08:21:38.945007213+00:00"
id: "atelier-467y"
issue_type: "bug"
labels:
- "cli-output"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-rc8j"
    type: "advances"
  - kind: "review"
    id: "atelier-268e"
    type: "discovered_from"
  - kind: "review"
    id: "atelier-owat"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T17:32:59.889470263+00:00"
status: "done"
title: "Give land's Stop-hook refusal one code and one recovery step"
updated_at: "2026-10-23T17:32:59.889470263+00:00"
---

## Goal

Found in review of atelier-91w1:
- The land hook's block reason (`crates/atelier-cli/src/commands/session.rs`, about line 460) appends "Fix the named check or hand off…" after the error's own `Next:` step, so the agent gets two recovery steps (finding atelier-268e).
- Any snapshot failure that is not an uncommitted-record error, Git failures included, is labelled `land_records_uncommitted` with a "commit records" step (about lines 51-57), contradicting the rule that Git failures carry no code (finding atelier-owat).

## Done when

- A hook block reason carries exactly one recovery step.
- Only an uncommitted-record failure is coded `land_records_uncommitted`; other snapshot failures keep their own message.
- Tests cover a hook refusal and a Git failure during land.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed (session.rs:636 trailer); records_uncommitted (session.rs:51-57) labels any error, Git failures included, as land_records_uncommitted.
- 2026-10-23 [decision] The one-recovery-step rule is met by making the hook reason land's refusal verbatim; the land_records_uncommitted mislabel of other snapshot failures was already gone after atelier-0uaj (records_uncommitted passes other errors through), so this item adds a test (corrupt .git/index) rather than a code change for it.
  Reason: Re-verified against current code: both read_record_files and the final fence downcast RecordsUncommitted and return any other error unchanged.

## Close Reason

The Stop hook's block reason is exactly land's refusal, each failure carrying its one `Next:` step and the verdict last; a Git failure while land reads the checkout keeps Git's own message and is not coded `land_records_uncommitted`.

### Deviations
The snapshot-mislabel half needed no code change: after atelier-0uaj, both the read and the final fence pass any non-RecordsUncommitted error through unchanged. This item adds the Git-failure test (corrupt `.git/index`) that pins it.

### Decisions
See the decision note.

### Surprises
None.

### Open items
None.
