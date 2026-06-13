---
created_at: "2026-06-13T20:44:42.287682511+00:00"
id: "atelier-cve1"
issue_type: "epic"
labels: []
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-1krs"
  - kind: "issue"
    id: "atelier-1vdl"
  - kind: "issue"
    id: "atelier-1xuf"
  - kind: "issue"
    id: "atelier-50tm"
  - kind: "issue"
    id: "atelier-a0fd"
  - kind: "issue"
    id: "atelier-c9ej"
  - kind: "issue"
    id: "atelier-e723"
  - kind: "issue"
    id: "atelier-gzel"
  - kind: "issue"
    id: "atelier-pa33"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Improve agent readiness and mechanical guardrails"
updated_at: "2026-06-13T23:12:00.253045347+00:00"
---

## Description

Apply an agent-readiness lens to the stabilization mission. The goal is not generic process expansion; it is to make fresh agents able to set up, choose work, run the right checks, detect quality hazards, and hand off without private context.
- Fresh-agent entry, setup, validation, and tracker workflows are verifiable from committed repository files.
- Mechanical guardrails exist for the recurring quality hazards identified by the readiness audit: command-doc drift, complexity, dead code, dependency hygiene, and task ownership.
- Optional collaboration scaffolding such as GitHub templates or CODEOWNERS is either added or explicitly classified as not applicable for this repo.
- Agent-readiness audit evidence record scores the repository against Agent Factory and Factory-style criteria.
- File change review shows any added setup, docs validation, quality tooling, or collaboration templates.
- `atelier lint`, `atelier export --check`, `atelier doctor`, and focused setup/quality command transcripts pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
