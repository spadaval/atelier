---
created_at: "2026-10-17T01:10:57.973237161+00:00"
id: "atelier-rp1c"
issue_type: "feature"
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
    id: "atelier-nn9y"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Summarise prune by eligible vs kept, and name the root keeping each cluster"
updated_at: "2026-10-17T01:10:57.973237161+00:00"
---

## Goal

On the real repository `atelier prune` shows `Candidates: 74` with no eligible or protected split, then prints 74 lines. 46 end in `referenced by retained issue atelier-1ioo` (15 cite atelier-8ud1, 8 cite atelier-xz11). atelier-1ioo is not itself a candidate (closed 2026-10-15, inside the 7-day window), so the reader cannot learn why it is kept or when the cluster becomes prunable without another command. The output also always ends with a hard-coded section (`print_deferred_classes`, crates/atelier-cli/src/commands/prune.rs:1338-1342):

```
Deferred Cleanup Classes
------------------------
  none
```

## Done when

- The header reads `Canonical records: 0 to remove, 74 kept (cutoff 2026-10-10, 7 days)`.
- Kept records are grouped by root: `atelier-1ioo  Complete session workflow… · closed 2026-10-15, within 7 days (prunable after 2026-10-22) · keeps 46: 3 issues, 31 reviews, 12 evidence`. Per-record lines appear only with `--full`.
- The Deferred Cleanup Classes section is removed, or printed only when a class is actually deferred.
- A test covers grouping by a recent root.

## Context

docs/product/retention-and-prune.md (protection follows links in both directions; atelier-r7cd).
