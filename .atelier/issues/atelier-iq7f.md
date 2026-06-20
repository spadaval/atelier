---
created_at: "2026-06-19T19:39:57.005628760+00:00"
id: "atelier-iq7f"
issue_type: "validation"
labels:
- "prune"
- "validation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Validate pruning safeguards end to end"
updated_at: "2026-06-19T19:39:57.005628760+00:00"
---

## Description

Validate the completed pruning workflow against the retention contract, command
surface, and safety expectations.

## Outcome

- The documented retention classes match implemented command behavior.
- Dry-run output is clear enough for an operator to decide what would be
  removed or protected.
- Apply paths refuse protected records, active/open work, required proof, dirty
  worktrees, current/protected branches, and unmerged branches.
- Eligible local, Git, and canonical-record candidates are pruned according to
  contract.

## Evidence

- Independent validation evidence maps each mission validation criterion to
  command transcripts or focused test output.
- Required closeout commands include focused prune tests, `atelier lint`,
  `git diff --check`, and documentation/help parity checks for the prune
  command surface.
