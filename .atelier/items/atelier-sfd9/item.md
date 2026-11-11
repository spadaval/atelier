---
schema: atelier.item
schema_version: 3
id: atelier-sfd9
title: Give a repair route for links to missing records that arrive by merge or hand edit
status: dropped
mission: atelier-uf97
labels:
- check
- cli-output
- feature
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T01:10:57.971403328Z
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

## Report

Dropped: Fixed by the 0.5 rewrite: `check` names the file and field of a `blocked_by` that names no item, and the record still loads so it can be edited.
