---
created_at: "2026-10-17T08:28:57.961331205+00:00"
id: "atelier-vsh8"
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
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "List every record bundle apply writes in its quiet output"
updated_at: "2026-10-17T08:28:57.961331205+00:00"
---

## Goal

`bundle apply -q` prints only the IDs in `summary.records` (`crates/atelier-cli/src/commands/bundle.rs`, about line 49), but apply also writes existing records it never lists: a `parent` pointing at an existing ID writes that parent, and `blocks`/`advances` declared by an existing target write the target. The Quiet Tokens row says "the IDs of the records it created or updated", so a script misses the modified parent. Found in review of atelier-pr4k (finding atelier-ayr9).

## Done when

- `bundle apply -q` lists every record it writes, or the Quiet Tokens row says exactly which it lists.
- A test applies a bundle that adds a child under an existing parent and checks the quiet output.
