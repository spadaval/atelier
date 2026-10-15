---
created_at: "2026-10-15T21:00:45.352378891+00:00"
id: "atelier-40i6"
issue_type: "bug"
labels:
- "adoption"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fxbg"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Define how the commit that installs Atelier lands"
updated_at: "2026-10-15T21:00:45.352378891+00:00"
---

## Goal

The commit that runs `atelier init` and imports the first records happens before any item can own a branch. On that branch, `atelier land` fails with `land_work_untraceable`, and `atelier merge` has no owner to integrate, so the install has to land by plain Git or a PR, outside the gates.

## Done when

The install procedure in the docs names a supported path for landing the bootstrap commit, and `land` on that branch gives guidance instead of an error; a test covers it.

## Context

astromech branch `chore/atelier-tracker` (four commits: init, bundle import, review fixes).

Found 2026-10-15 while moving astromech's BACKLOG.md (about 135 items with research, references and decisions) into Atelier records. Sumeet's principle: Atelier records should hold the whole backlog; needing a side document means Atelier needs work.
