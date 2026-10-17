---
created_at: "2026-10-17T01:10:57.962045771+00:00"
id: "atelier-8bve"
issue_type: "feature"
labels:
- "cli-output"
- "issue-commands"
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
title: "Make issue create confirm the record and suggest the next step"
updated_at: "2026-10-17T01:10:57.962045771+00:00"
---

## Goal

The `issue create` confirmation (`crates/atelier-cli/src/commands/issue.rs:667-699`) prints `Created issue atelier-xmtr - Title`, Type, Priority and the file path, and never shows status, labels, size or mission. With `--parent` it prints only `Created subissue X under Y`, without Type, Priority or the parent's title. The footer is four generic actions ("Edit issue Markdown", "Validate this issue", "Inspect this issue", "Inspect tracked work transitions"). It never suggests the actual next step: `atelier start <id>`, or linking the item to a mission. A failed `--parent <mission>` error names an ID that was allocated but never written: `issue atelier-mzcf cannot be child of mission atelier-1w8c`.

## Done when

- The confirmation is one dense block: `Created atelier-xxxx task todo medium "Title"`, then labels/size when set, `Parent: <id> "<title>"` when set, and the file path.
- Next steps come from state. Ready work suggests `atelier start <id>`. Non-mission work with no mission suggests `atelier issue link <id> <mission-id> --role advances` (with the real mission ID when exactly one mission is open). The generic four-item menu is gone.
- Validation errors raised before the write don't name the unpersisted ID.
- CLI tests cover plain, `--parent` and `--status backlog` output.
