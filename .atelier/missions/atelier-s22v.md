---
created_at: "2026-06-14T16:39:17.032525489+00:00"
id: "atelier-s22v"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-jxs8"
    type: "advances"
  - kind: "issue"
    id: "atelier-xq7i"
    type: "advances"
  - kind: "issue"
    id: "atelier-xzsm"
    type: "advances"
  - kind: "issue"
    id: "atelier-ybit"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Simplify closeout and tracker workflow after postmortem"
updated_at: "2026-06-14T16:45:29.180711919+00:00"
---

## Intent

Turn the postmortem decisions into a simpler Atelier workflow: missions act as coordination shells, proof approval belongs to workflow validation, tracker commands do not self-block closeout, and command-surface drift checks understand hidden command visibility.

## Constraints

- Do not add compatibility aliases, staged deprecations, or fallback command shims.
- Keep mission closeout aggregate-first unless a separate validation issue explicitly owns mission-level judgment.
- Preserve useful proof and docs/help safeguards without exposing implementation heuristics as product behavior.

## Risks

- Removing heuristic proof matching can weaken closeout if workflow validation is not explicit enough.
- Relaxing clean-worktree checks can hide meaningful uncommitted tracker state unless status and handoff guidance remain clear.

## Validation

- Mission-linked epics cover closeout proof model, tracker clean-tree/read-only behavior, and docs/help hidden-command visibility; linked validation work owns the final parent-claim mapping.
- Focused tests or transcripts prove mission close no longer depends on token matching, transition options are read-only, tracker files do not self-block closeout, and docs/help drift handles hidden commands.
- Product, workflow, validation, and Agent Factory docs describe the simplified model without red-tape heuristics.
- atelier lint, git diff --check, and relevant focused workflow and command-surface checks pass.
