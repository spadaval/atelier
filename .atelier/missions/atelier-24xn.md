---
created_at: "2026-06-19T19:38:36.424983456+00:00"
id: "atelier-24xn"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-t3h3"
    type: "advances"
  - kind: "issue"
    id: "atelier-txf6"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Prune stale Atelier artifacts and branches"
updated_at: "2026-06-19T19:39:08.969007540+00:00"
---

## Intent

Define and implement safe pruning so Atelier repositories do not accumulate unbounded evidence files, activity sidecars, closed issues, missions, branches, worktrees, local diagnostics, and cache artifacts over long-running agent work.

## Constraints

- Separate canonical tracked record retention from ignored local runtime/cache cleanup and Git branch/worktree cleanup.
- Protect active work, open blockers, attached proof for non-archived work, dirty worktrees, and unmerged branches by default.
- Destructive cleanup must be explainable, dry-run first, and explicitly confirmed or forced.

## Risks

- A prune command could delete proof needed to validate closed work or audit historical decisions.
- Branch or worktree cleanup could discard unmerged changes if eligibility rules are too broad.
- A single retention policy could blur tracked project state, local runtime state, and external artifact references.

## Validation

- Committed product and architecture docs define retention classes, prune eligibility, safety defaults, and operator command shape.
- Dry-run command output lists prune candidates with record kind, age or lifecycle reason, protection reason, and apply command.
- Focused CLI tests reject active/open records, attached required proof, dirty worktrees, and unmerged branches, while allowing eligible stale local artifacts.
- Final validation records command transcripts for focused prune tests, atelier lint, git diff --check, and documentation/help parity.
