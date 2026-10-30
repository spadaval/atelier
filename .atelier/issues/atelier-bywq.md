---
created_at: "2026-10-30T06:40:57.500738838+00:00"
id: "atelier-bywq"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-9pa6"
    type: "related"
  - kind: "issue"
    id: "atelier-uf97"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Warn when a command on the base edits a record an open work branch owns"
updated_at: "2026-10-30T06:40:57.500738838+00:00"
---

## Goal

On the base branch, `atelier issue update <id>` and `atelier issue note <id>` succeed silently on an item that an open work branch owns, and `land` passes. Checked on master 2f1eea77 in a scratch repository: owner started on `task/<id>`, then on `main` `issue update --priority high` and `issue note` both exited 0 with no warning. The branch's copy of the record then conflicts at merge, and resolving that conflict is a change after review that needs fresh review. Warn at the command, when the edit is still cheap to undo.

## Done when

An integration test starts an owner on its work branch, commits the start there, switches to the base and runs `issue update <id> --priority high`, `issue note <id> "x"` and a transition: each still applies, exits 0, and prints one warning naming the owning branch and saying the edit belongs on that branch. The same commands on an item no open branch owns print no warning.

## Scope

Issue mutation commands run on the base branch. Work branches and records-only planning commits that touch unowned items are unaffected.

## Context

- Sibling atelier-9pa6 warns when a done transition runs on the base; share the warning path.
- Open branches and their owners are already listed by `status` (Branches) and computed as in-flight marks (atelier-3i7x).
