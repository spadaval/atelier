---
created_at: "2026-10-17T19:00:36.343067865+00:00"
id: "atelier-57wj"
issue_type: "feature"
labels:
- "evidence"
- "retro-2026-10-17"
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
title: "Store a summary of passing command output in evidence, not the full log"
updated_at: "2026-10-23T14:14:37.648780001+00:00"
---

## Goal

Evidence records hold 1.6 MB of the window's 3.1 MB of records; 68% of that is pasted cargo output ('Compiling ...' lines and 7,703 `PASS [` lines), and 276 of 449 files are truncated anyway. A passing receipt needs the command, exit code, commit, tree, counts and a digest, not the log.

## Done when

A passing `evidence record` stores the command, exit status, commit, a test-count summary line and a digest of the full output, plus the output tail only on failure. Failing receipts keep the failure tail as atelier-bphy specifies. Existing records still parse. A test pins the new shape for pass and fail.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168). Must stay compatible with atelier-bphy (keep the tail of failing output).

## Notes

- 2026-10-23 [note] 2026-10-23 regroom against master ff7cc896: still needed. Passes store the same 4 KiB streams as fails (evidence.rs:336, lib.rs:2030); 562 of 590 receipts are passes (2.1 MB). Do not rewrite existing receipts (atelier-dg3h scope). Lands with or after bphy.
