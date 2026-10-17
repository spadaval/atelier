---
created_at: "2026-10-17T01:36:21.044375930+00:00"
id: "atelier-rz2l"
issue_type: "feature"
labels:
- "issue-commands"
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
title: "Import a drafted record file into the tracker after validating it"
updated_at: "2026-10-17T01:36:21.044375930+00:00"
---

## Goal

Agents that draft records outside the tracker, for bulk grooming or long bodies, have no way to validate a draft and bring it in. In astromech on 2026-10-16, a grooming script passed Markdown bodies as shell arguments. It died on an apostrophe after creating part of its records. That left 16 untracked records, including two "Ships page with live build stats" duplicates, which were cleaned up with `rm` and a hand edit of a relationships list. Writing each record as a file and importing it avoids shell quoting entirely.

## Done when

- `atelier issue add <file>...` (name to be decided) does the following for each draft:
  - validates it as `atelier check` would: front matter, typed body sections, labels, and links to existing records;
  - assigns an ID when the draft has none, and refuses an ID that already exists;
  - moves it into `.atelier/issues/`;
  - prints the new ID.
- A batch is all-or-nothing, with one error per failing file that names the file and field. `-q` prints only the IDs.
- A starting draft for a type can be printed, for example with `--template <type>`.
- Tests cover a valid draft, a draft with an invalid section, a duplicate ID, a dangling link, and a mixed batch.

## Context

This may answer the open question on atelier-o3lm about a Markdown input format for bundles: importing record files gives Markdown input for single records. Bundles stay the tool for creating linked graphs. From the 2026-10-16 transcript retrospective.
