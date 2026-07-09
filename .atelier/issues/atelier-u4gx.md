---
created_at: "2026-07-06T19:26:09.538256093+00:00"
id: "atelier-u4gx"
issue_type: "feature"
labels:
- "agent-factory"
- "cli"
- "docs"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Add Agent Factory references and an Atelier work-model man page"
updated_at: "2026-07-06T19:26:09.538256093+00:00"
---

## Description

Restore a concise progressive-disclosure reference layer to the repository-owned Agent Factory skill. Portable submodel-selection and repository-shape guidance belongs in skill references; Atelier-specific mission, epic, and issue semantics belong in an executable Atelier man topic. Preserve the accepted Agent Factory/Atelier ownership boundary and do not restore tracker command cookbooks or duplicated workflow policy.

## Outcome

- `.agents/skills/agent-factory/references/` contains concise portable references for submodel selection and agent-ready repository shape, and `SKILL.md` tells agents exactly when to load each reference.
- The orchestrate, install, and readiness procedures route detailed portable guidance to those references without duplicating it or importing Atelier-specific command policy.
- `atelier man work-model` explains the mission, epic, and issue split, hierarchy versus `advances` scope, proof ownership, and the commands used to inspect each boundary; bare `atelier man` discovers both role guides and the work-model topic.
- Product command documentation and focused tests match the new man topic.

## Evidence

- Focused CLI tests cover `atelier man` discovery, `atelier man work-model`, and unknown-page guidance.
- Skill validation or an explicit validator compatibility finding confirms the updated skill structure and direct reference links.
- `cargo fmt --all -- --check`, focused `cargo nextest`, `atelier check`, and `git diff --check` pass.
