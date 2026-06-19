---
created_at: "2026-06-19T01:43:11.589867283+00:00"
id: "atelier-bilp"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-19T05:14:43.260519819+00:00"
status: "done"
title: "Slim Agent Factory guidance to orchestration-only role"
updated_at: "2026-06-19T05:14:43.260519819+00:00"
---

## Description

Agent Factory guidance has grown into a second command manual for Atelier.
That duplicates `atelier man`, product command docs, validation policy, and
workflow-owned proof behavior. Slim the repo-local skill so it only owns
portable agent coordination: subskill routing, one-role delegation, model and
delegation judgment, assignment context, and role posture.

## Outcome

- `.agents/skills/agent-factory/SKILL.md` routes agents to compact subskill
  references and to Atelier-owned command/help surfaces for tactical workflow.
- Obsolete skill standards that duplicate Atelier command manuals, work item
  authoring policy, generic repo workflow, or subskill tables are removed or
  replaced by direct references to durable product/quality docs.
- Agent Factory philosophy is folded into `docs/product/zen.md` so the product
  doctrine has one durable home.
- Procedure references no longer teach `atelier export --check` or other
  maintenance diagnostics as normal handoff or validation commands.
- `AGENTS.md` no longer lists `atelier export --check` as a common command or
  normal validation criterion.
- The installed `atelier` binary exposes the current `atelier man validator`
  role guide.

## Evidence

- `atelier man` and `atelier man validator` output show the validator role from
  the installed binary.
- `rg -n 'export --check|standards/(tracker|work-item-authoring|repo-workflow|subskills|ready-work)' AGENTS.md .agents/skills/agent-factory`
  shows no normal Agent Factory or repository guidance still depending on the
  removed duplicate standards.
- `docs/product/zen.md` includes the retained philosophy around bounded roles,
  delegation, proof, verification methods, and durable failure signals.
- `atelier lint atelier-bilp`, `atelier lint`, and `git diff --check` pass.
