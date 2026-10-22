---
created_at: "2026-10-17T01:10:57.973536770+00:00"
id: "atelier-rmvb"
issue_type: "bug"
labels:
- "cli-output"
- "prune"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Keep prune rows to one line with a one-line title"
updated_at: "2026-10-17T01:10:57.973536770+00:00"
---

## Goal

Prune uses an evidence or review record's summary or body as its `title` (crates/atelier-cli/src/commands/prune.rs:1333-1335). One evidence row (atelier-kf2p) printed four paragraphs, including `DISCARD: no reliable wall improvement…`, and many review rows are 1,000+ character lines. Each eligible record takes three lines:

```
  eligible issue atelier-0349 (done, latest 2026-10-16, path .atelier/issues/atelier-0349.md)
    recover: git log --all -- .atelier/issues/atelier-0349.md; git show <commit>:.atelier/issues/atelier-0349.md
    title: Make atelier land fast enough for the Stop hook
```

The path repeats what the ID implies. Protected rows leave out the title, which is what helps decide whether a record matters. `(recorded, …)` status is noise on evidence and reviews.

## Done when

- Every candidate is one line, `eligible issue atelier-0349  done 2026-10-16  Make atelier land fast enough…`, with titles cut to the first line and truncated.
- Protected rows include the title.
- The recovery recipe prints once in a footer.
- A test with a multi-paragraph evidence summary asserts one line per row.

## Context

Applies the one-line rule from ADR 0022 and atelier-deyy to prune.
