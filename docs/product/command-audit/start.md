# `atelier start`

Primary role: Worker.

Primary question: "How do I make this issue part of the current work in this
checkout?"

## Assessment

- Name: Removed. Starting work is an issue lifecycle transition, not a separate
  root command.
- Documentation: This page is retained only as a retired-surface note. Current
  help and role guides should teach `atelier issue transition <id> start`.
- Design: Remove. Duplicate lifecycle paths obscure the workflow-backed
  transition surface and make branch/worktree side effects harder to reason
  about.
- Output hierarchy: Owned by `issue transition`: issue ID, resulting workflow
  status, branch/workspace side effects, proof expectations, and next commands.

## Role Use

| Form | Primary role | Operator purpose | Fit |
| --- | --- | --- | --- |
| `atelier start <issue-id>` | Worker | Legacy duplicate of issue lifecycle start. | Removed; use `atelier issue transition <id> start`. |
| `atelier issue transition <id> start` | Worker | Begin implementation or validation work through the canonical workflow transition. | Good. |
