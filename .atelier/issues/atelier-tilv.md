---
created_at: "2026-06-18T22:42:06.615655083+00:00"
id: "atelier-tilv"
issue_type: "task"
labels:
- "cli"
- "forgejo"
- "pr"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Align PR comment and review discussion surfaces"
updated_at: "2026-06-18T22:42:06.615655083+00:00"
---

## Description

Reviewers need a reliable CLI view of PR feedback. Today the command surface can
be confusing because top-level PR comments and review comments are handled by
different Forgejo endpoints while the user-facing names do not make that
boundary obvious. Align the command behavior or wording so review discussion is
inspectable without knowing Forgejo's internal API split.

## Outcome

- `atelier pr comments --issue <id>` either lists both top-level PR comments and
  review comments, or the CLI exposes distinct commands/help text that clearly
  names which discussion stream each command reads.
- `atelier pr comment` output points reviewers to the command that will show
  the comment they just created.
- Reviewer and validator role guidance uses the final command vocabulary.

## Evidence

- Mocked Forgejo tests cover comment creation plus the corresponding list/show
  command output.
- Help or role-guide transcript proves the command names no longer imply that
  review feedback will disappear.
- `cargo test -p atelier-app --lib forgejo::tests` and focused CLI tests pass.
