---
created_at: "2026-06-14T04:03:21.700088169+00:00"
id: "atelier-ywqa"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-14T08:11:41.426005641+00:00"
status: "done"
title: "Remove hidden legacy issue command surfaces"
updated_at: "2026-06-14T08:11:41.426005641+00:00"
---

## Description

Hidden issue subcommands remain in the command enum for predecessor workflows:
quick, subissue, search, block, unblock, blocked, relate, unrelate, related,
impact, next, tree, tested, comment, label, unlabel, delete, and close-all
style behavior. These hidden aliases are no longer needed for the normal
Atelier command model. Remove them rather than inventorying them indefinitely,
except for any helper that a worker proves is still required by a current core
workflow test and documents as internal-only.

## Outcome

- Hidden legacy issue command variants are deleted from dispatch or reduced to
  explicitly justified internal-only helpers.
- Replacement public commands cover create, parented create, active work,
  search, issue blockers, graph/tree/impact, record-specific notes, labels,
  destructive deletion, and proof workflows without hidden aliases.
- Public docs and Agent Factory guidance do not depend on hidden legacy issue
  commands.

## Evidence

- Source diff removes each hidden legacy issue command or documents the narrow
  internal-only reason for any remaining helper.
- Search transcript shows public docs and guidance do not recommend removed
  hidden commands.
- Focused tests or command transcripts cover any changed rejection or help
  behavior.
- `git diff --check` and `atelier lint` pass.
