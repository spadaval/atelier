---
created_at: "2026-06-15T21:30:03.510385707+00:00"
id: "atelier-rrmq"
issue_type: "epic"
labels:
- "product"
- "validation"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3gr9"
  - kind: "issue"
    id: "atelier-3yoa"
  - kind: "issue"
    id: "atelier-9p3t"
  - kind: "issue"
    id: "atelier-a44d"
  - kind: "issue"
    id: "atelier-jeyr"
  - kind: "issue"
    id: "atelier-ooyw"
  - kind: "issue"
    id: "atelier-rdyl"
  - kind: "issue"
    id: "atelier-tpuc"
  - kind: "issue"
    id: "atelier-yn3u"
  - kind: "issue"
    id: "atelier-z0yu"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T20:52:46.079860356+00:00"
status: "done"
title: "Epic: Remove closeout as a separate product concept"
updated_at: "2026-06-16T20:52:46.079860356+00:00"
---

## Description

Replace closeout with workflow-owned completion semantics without adding a configurable mission workflow graph.

## Outcome

- No closeout issue type or live command vocabulary remains in product code, config, help, or target-state docs.
- Issues and epics use normal issue workflow transitions for terminal work.
- Missions keep the simple built-in lifecycle draft, ready, active, closed while terminal mission checks use shared validator policy instead of a separate closeout subsystem.
- `AGENTFACTORY.md` and the Agent Factory skill docs route validation and evidence work to the new model: no separate closeout issue concept, no evidence `pass/fail` result as proof semantics, and validation agents classify claims while attaching evidence that validates the accountable issue.
- Historical tracker records remain readable or are migrated explicitly according to the implementation plan.

## Evidence

- `rg` over crates, docs, specs, `AGENTFACTORY.md`, `/root/.agents/skills/agent-factory`, `PRODUCT_INTENT.md`, `CONTEXT.md`, and `.atelier/workflow.yaml` shows no live closeout vocabulary or obsolete evidence-result examples except explicitly retained historical references.
- `atelier help`, `atelier mission --help`, and `atelier mission status --help` show no `--closeout` flag or `Closeout` headings.
- Focused tests cover issue type removal, mission terminal checks, status next actions, and blocked terminal transitions.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, and relevant `cargo nextest` or `cargo test` runs pass.
