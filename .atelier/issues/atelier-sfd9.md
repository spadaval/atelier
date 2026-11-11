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
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Give a repair route for links to missing records that arrive by merge or hand edit"
updated_at: "2026-11-07T21:03:24.979790291+00:00"
---

## Goal

Since atelier-ilba a command refuses to write a link to a record that does not exist, naming the branch that holds it when one does. Such links still arrive by a merge or a hand edit, and their repair route is missing (checked 2026-11-07): `atelier issue unlink <id> <missing> --role advances` fails with the same load error, because the graph will not load, and the finding names the file but not the field to edit. A deleted target gets a `git restore --source=<commit>^ …` hint; a target that never existed gets none.

## Done when

- [ ] The finding names the file and field, e.g. `.atelier/issues/atelier-gfi8.md relationships.relates[type: advances] → atelier-nope`, and the edit that removes it.
- [ ] `issue unlink` removes a dangling link while the graph is otherwise invalid, or the finding says to edit the file by hand.
- [ ] Advice shared by several findings prints once after the list.
- [ ] Tests cover a never-existed target written by hand and the unlink path.

## Context

Narrowed in decision atelier-mzpn to links that reach the records without a command. atelier-kset owns the one-line finding format; share it.
