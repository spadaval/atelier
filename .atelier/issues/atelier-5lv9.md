---
created_at: "2026-11-09T22:10:10.298438603+00:00"
id: "atelier-5lv9"
issue_type: "task"
labels:
- "agent-factory"
- "review"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fnlu"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give the light-path review brief the moving-part rule and Verdict/Sizes Report line atelier-fnlu added to the item brief"
updated_at: "2026-11-09T22:10:10.298438603+00:00"
---

## Goal

The brief `atelier review request` prints for a change with no issue (`crates/atelier-cli/src/commands/light_path.rs`) asks reviewers the same questions as the item brief. atelier-fnlu added a working rule (size the change, ask which requirement needs each moving part it adds) and a Report line asking for Verdict and Sizes to the item brief only, since its scope named `review_request.rs`. The light-path brief still asks only for Correctness and Fitness.

## Done when

- [ ] The light-path brief carries the moving-part working rule, with the commit messages standing in for Done when as the requirements, and its Report line asks for Verdict and Sizes; a light-path brief test covers both lines.
