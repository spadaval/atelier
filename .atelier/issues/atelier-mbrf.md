---
created_at: "2026-10-17T19:00:36.352316978+00:00"
id: "atelier-mbrf"
issue_type: "feature"
labels:
- "land"
- "merge"
- "retro-2026-10-17"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-k8h8"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Let planning text reach the base without a full work-branch cycle"
updated_at: "2026-10-22T19:58:17.943290365+00:00"
---

## Goal

Records may land on master directly, but ADRs, mission strategies, the vendored skill and `workflow.yaml` need an issue, branch, proof, review and merge. A one-line `workflow.yaml` change (qymu) took a full cycle; ADR 0022 (bw0b) took 3 review rounds and 21 mostly-wording findings; the skill vendoring (92mz, kr30) hit merge_diverged and two re-approvals.

## Done when

The policy can name planning paths that land on the base like records, or accept a human-approval review event ('Sumeet approved this text') in place of an independent review for them. `land` enforces whichever rule is configured, and `docs/product/workflow-configuration.md` documents it. Tests cover an allowed planning path and a code path that still requires review.

## Context

Found in the 2026-10-17 retrospective of the 10-15..10-17 atelier runs (sessions b32c39e6, 9a04974d, d483b55c, bf433168).

## Unknowns

Which paths qualify, and whether policy files themselves should ever skip review.
