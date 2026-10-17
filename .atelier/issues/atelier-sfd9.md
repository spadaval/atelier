---
created_at: "2026-10-17T01:10:57.971403328+00:00"
id: "atelier-sfd9"
issue_type: "feature"
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
    id: "atelier-o3lm"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give a repair route for links to records that never existed"
updated_at: "2026-10-17T01:10:57.971403328+00:00"
---

## Goal

When a link's target was deleted, check prints a good `git restore --source=<commit>^ -- ':/.atelier/issues/<id>.md'` hint. When the target never existed (`atelier-gfi8 has advances reference to missing issue atelier-nope`), there is no advice. The obvious repair, `atelier issue unlink gfi8 atelier-nope --role advances`, fails with the same error because the graph will not load. Every command adds `Next: run atelier check, fix every named record file, and rerun`, but no file is named. The deleted-record advice is a 60-word block repeated for each finding, and says "delete it from the linking issue's relationships" without naming the file or field.

## Done when

- The finding names the file and field, e.g. `.atelier/issues/atelier-gfi8.md relationships.relates[type: advances] → atelier-nope`, and the exact edit to make.
- Either `issue unlink` removes a dangling edge while the graph is otherwise invalid, or the message says to edit the file by hand.
- Advice shared by several findings prints once after the list.
- Tests cover a never-existed target and the unlink path.
