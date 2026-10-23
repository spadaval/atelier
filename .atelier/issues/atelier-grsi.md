---
created_at: "2026-10-17T01:10:57.980882902+00:00"
id: "atelier-grsi"
issue_type: "bug"
labels:
- "cli-output"
- "land"
priority: "P1"
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
title: "Name every uncommitted record and the git command in land_records_uncommitted"
updated_at: "2026-10-23T15:47:09.279479452+00:00"
---

## Goal

With `.atelier/issues/atelier-43m1.md` modified and `.atelier/issues/atelier-vhkt.md` untracked, `atelier land` prints:
```
Error: land_records_uncommitted: canonical_snapshot_uncommitted: .atelier/issues/atelier-43m1.md path, bytes or mode differs from pinned HEAD; commit canonical records before continuing
Commit canonical records and policy with the work they describe, then retry.
```
- Only the first path is named, so an agent commits one record and fails again on the next.
- It stacks two error codes, uses jargon ("pinned HEAD", "path, bytes or mode"), and says "commit" twice without giving a command.
- This is the most frequent real Stop hook block in the transcripts, in this repository and in astromech (`s-n70u`).

## Done when

- The message reads `land_records_uncommitted: 2 tracker records differ from HEAD: .atelier/issues/atelier-43m1.md (modified), .atelier/issues/atelier-vhkt.md (new). Commit them with the work: git add .atelier && git commit`.
- It has one error code, and paths are listed first 5, then `and N more`.
- A test covers several dirty and untracked records.

## Context

The message is built in crates/atelier-cli/src/commands/session.rs:40-43 and 306-308, from `CommittedSnapshot::verified_working_paths` and `read_record_files` (crates/atelier-app/src/committed_snapshot.rs). atelier-mp6v changes whether the hook blocks on master. This item changes the message everywhere.

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: partly done: one code now (session.rs:51-57). Still names only the first path, with 'path, bytes or mode differs from pinned HEAD' (committed_snapshot.rs:244/288).
- 2026-10-23 [decision] The records fix is git add .atelier && git commit -m "…", or git add .atelier alone when every difference is staged-only (working bytes already match HEAD).
  Reason: git commit would fail with nothing to commit in the staged-only case; both run verbatim.
