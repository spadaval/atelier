---
created_at: "2026-10-23T14:48:54.336219134+00:00"
id: "atelier-jiz0"
issue_type: "bug"
labels: []
priority: "P3"
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
title: "Say an event is not a finding when disposition is given its ID"
updated_at: "2026-10-23T15:19:35.824157468+00:00"
---

## Goal

`atelier review disposition <id>` given an approval (or any non-finding event) refuses with `disposition must target the finding's owner`, which names the wrong cause. Say that the event is not a finding, name its kind, and point to `atelier review list <owner>` to find the finding IDs.

## Done when

A CLI test: disposition of an approval ID prints `<id> is an approval, not a finding` (or equivalent naming the kind) with the `review list` recovery command for its owner, and exits non-zero.

## Context

Hit on 2026-10-23 while dispositioning atelier-dg3h review: the reviewer reported the finding and approval IDs swapped, and the error sent us looking at owners. Code: crates/atelier-cli/src/commands/review.rs disposition path.
