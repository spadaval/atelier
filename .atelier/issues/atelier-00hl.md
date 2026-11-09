---
created_at: "2026-11-09T00:45:59.153090199+00:00"
id: "atelier-00hl"
issue_type: "feature"
labels:
- "agent-attribution"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-bnek"
    type: "advances"
  - kind: "issue"
    id: "atelier-tu8c"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Name the reviewer's provider and model in structured fields on review records"
updated_at: "2026-11-09T01:04:26.826682230+00:00"
---

## Goal

Name the reviewer's provider, model, effort and harness in structured fields on every review record, filled by the CLI from the environment, so review outcomes can be compared by model. Today the identity is free text the agent types, such as `claude-code-reviewer (opus-5.5)`.

## Done when

- [ ] Review events written by the CLI carry `provider`, `model`, `effort` and `harness` where the environment or an explicit flag gives them, beside the existing free-text `identity`. A flag (for example `--model`) sets the model when the environment cannot.
- [ ] Records without the new fields keep reading, checking and merging under the new binary, and the old installed binary still reads records with them (one binary serves every repository). Tests cover both directions.
- [ ] `scripts/smoke-repos.sh` passes against every local Atelier repository.
- [ ] Agent Factory's review procedure tells reviewers to pass the model when the environment cannot give it.

## Context

The harness variables are in the attribution item and the strategy. Independence checks compare identities; do not change what they compare.

## Unknowns

- Whether the model can be learned in-process for Claude and Codex at all; if not, the flag and the site's T3 lookup carry it.
