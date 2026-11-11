---
created_at: "2026-10-17T01:10:57.971709219+00:00"
id: "atelier-kset"
issue_type: "bug"
labels:
- "check"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sfd9"
    type: "related"
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Print each check finding once, on one line, against its record, and say what check checked"
updated_at: "2026-11-07T21:03:25.446176563+00:00"
---

## Goal

Two of `atelier check`'s output faults remain in the new tool (checked 2026-11-11 with the new binary in a converted scratch clone):
- YAML line numbers count from the start of the front matter, so they are one less than the file line: an unknown `status` on file line 6 of `.atelier/items/atelier-1a5k/item.md` is reported `at line 5 column 9`.
- A failure's last line is `Fix each in the file named, then run atelier check again.`, so `| tail -1` keeps neither the count nor the problem. Agents piped `atelier check` through `tail` 166 of 323 times in the old tool's transcripts.

## Done when

- [ ] YAML line numbers in `check`'s findings are file-relative; a test covers a bad front-matter value.
- [ ] A failure's last line names the first problem and the next step, not only a count or a general instruction; a test asserts it.

## Context

The diagnostic block also appears in status and transition output, so fix it at the shared formatter. The `Lint found N issue(s)` header the audit reported is now `Check found N findings:`. atelier-sfd9 adds the file and field to a missing-reference finding; share the format.

Rewritten for the new tool in the cutover sort (atelier-5zre): the new `check` already prints each fault once, on one line, against its file, and a pass names the counts (`Every record is valid: 173 items, 8 missions and config.toml.`), so those parts left the Goal and Done when; the two that remain are stated against the new binary and the new record path.
