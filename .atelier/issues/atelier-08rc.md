---
created_at: "2026-10-17T01:10:57.972626841+00:00"
id: "atelier-08rc"
issue_type: "task"
labels:
- "cli-output"
- "prune"
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
title: "End prune --apply with the commit step and an exact recovery"
updated_at: "2026-10-17T01:10:57.972626841+00:00"
---

## Goal

After removing 651 records in a scratch clone (`atelier prune --apply --retention-days 0`), apply printed the full three-lines-per-record list and stopped. There was no `651 removed` total and no commit step, even though docs/product/retention-and-prune.md says the user must commit the deletions. The `recover:` lines use a `<commit>` placeholder, though the exact recovery is known: `git restore` before committing, `git restore --source=HEAD~1` after. The Git section says `protected branch task/atelier-bw0b - owner atelier-bw0b has active workflow state` for a todo issue, which conflicts with the `active` status category.

## Done when

- Apply ends with `Removed 651 (512 issues, 90 evidence, 49 reviews). Next: git commit -am "Prune …"; undo before committing: git restore .atelier`, using the real counts.
- The owner reason reads `owner atelier-bw0b is open (todo)`.
- A test asserts the summary and footer.
