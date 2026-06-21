---
created_at: '2026-06-14T16:39:17.032525489+00:00'
id: atelier-s22v
issue_type: mission
labels:
- mission
priority: P2
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: issue
    id: atelier-jxs8
    type: advances
  - kind: issue
    id: atelier-xq7i
    type: advances
  - kind: issue
    id: atelier-xzsm
    type: advances
  - kind: issue
    id: atelier-ybit
    type: advances
schema: atelier.issue
schema_version: 1
closed_at: '2026-06-14T17:31:00.652106266+00:00'
status: closed
title: Simplify closeout and tracker workflow after postmortem
updated_at: '2026-06-14T17:31:00.652106266+00:00'
---

## Description

Turn the postmortem decisions into a simpler Atelier workflow: missions act as coordination shells, proof approval belongs to workflow validation, tracker commands do not self-block closeout, and command-surface drift checks understand hidden command visibility.

## Outcome

### Constraints

- Do not add compatibility aliases, staged deprecations, or fallback command shims.
- Keep mission closeout aggregate-first unless a separate validation issue explicitly owns mission-level judgment.
- Preserve useful proof and docs/help safeguards without exposing implementation heuristics as product behavior.

### Risks

- Removing heuristic proof matching can weaken closeout if workflow validation is not explicit enough.
- Relaxing clean-worktree checks can hide meaningful uncommitted tracker state unless status and handoff guidance remain clear.

## Evidence

- Manual check: Mission-linked epics cover closeout proof model, tracker clean-tree/read-only behavior, and docs/help hidden-command visibility; linked validation work owns the final parent-claim mapping.
- Manual check: Focused tests or transcripts prove mission close no longer depends on token matching, transition options are read-only, tracker files do not self-block closeout, and docs/help drift handles hidden commands.
- Manual check: Product, workflow, validation, and Agent Factory docs describe the simplified model without red-tape heuristics.
- Manual check: atelier lint, git diff --check, and relevant focused workflow and command-surface checks pass.

## Notes

### Terminal Notes

- Close reason: All linked work, parent epics, and independent validation issue atelier-jxs8 are closed with passing evidence; mission closeout gates are green.

Migrated from `.atelier/missions/atelier-s22v.md` as a declared mission objective issue.
