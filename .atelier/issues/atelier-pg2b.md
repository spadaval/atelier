---
created_at: "2026-10-28T02:06:44.271780469+00:00"
id: "atelier-pg2b"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-5tl2"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Tell Agent Factory's install procedure how to get the atelier CLI"
updated_at: "2026-10-28T02:06:44.271780469+00:00"
---

## Goal

`.agents/skills/agent-factory/procedures/install.md` step 1 says to confirm `atelier` is on `PATH` but not how to get it. An agent adopting Atelier in another repository is who the release installer is for.

## Done when

The step says to install it with the release installer from Atelier's README when it is missing, and the skill's evaluation (`.agents/skills/agent-factory/maintainers/skill-evaluation.md`) is followed for the wording change.

## Context

Found in review of atelier-5tl2 (R2). Skill wording changes behavior, so it needs independent review.
