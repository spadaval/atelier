---
created_at: "2026-06-30T16:08:07.533291886+00:00"
id: "atelier-sszj"
issue_type: "mission"
labels:
- "git"
- "mission"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fs79"
    type: "advances"
  - kind: "issue"
    id: "atelier-fs79"
    type: "validates"
  - kind: "issue"
    id: "atelier-j8ot"
    type: "advances"
  - kind: "issue"
    id: "atelier-li5h"
    type: "advances"
  - kind: "issue"
    id: "atelier-ms7i"
    type: "advances"
  - kind: "issue"
    id: "atelier-otxv"
    type: "advances"
  - kind: "issue"
    id: "atelier-qu06"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "in_progress"
title: "Mission: Add workflow-configured mission integration branches"
updated_at: "2026-06-30T18:15:15.391510506+00:00"
---

## Description

Introduce workflow-configured mission integration branches so repositories can
choose to route epic review branches through a mission branch before the
mission integrates to the configured base branch.

Constraints:

- No hidden mission-branch mode. Behavior is visible in workflow policy.
- No custom branch-template mini-language. Branch names are `<issue_type>/<issue_id>` unless a future product decision introduces field-derived naming for a real use case.
- Keep Git-related workflow names readable under the `git.*` namespace.
- Mission scope is based on direct `advances` links plus descendants, not hierarchy.
- Workflow output must show resolved source and target branches plainly.

Non-scope:

- Do not require all repositories to use mission integration branches.
- Do not add a generic branch selector language or symbolic codebook.
- Do not preserve obsolete action or validator aliases unless a human explicitly asks for a compatibility window.

Risks:

- Provider-backed review code may assume review targets are always the configured base branch.
- Branch base state must be durable and reviewable, not hidden runtime state.
- Syncing target branches without checkout must respect Git worktree safety.

## Outcome

Atelier supports mission integration branches as an explicit workflow policy choice. Repositories can opt into mission-scoped integration by configuring Git validators and actions in `.atelier/workflow.yaml`; repositories that omit those validators and actions keep ordinary base-branch behavior.

The target model removes configurable branch-name templates and uses canonical work branch names derived from issue type and issue id. Mission start can create `mission/<mission-id>` from the configured base branch. Epic start can require the operator to be on the mission branch and create `epic/<epic-id>` from that branch, recording the branch base for later review, merge, and sync. Epic close targets the recorded mission branch; mission close targets the configured base branch.

## Evidence

- Manual check: linked validation issue `atelier-fs79` owns mission completion proof for the implemented workflow behavior.
