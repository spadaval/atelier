---
created_at: "2026-06-15T21:30:54.557186845+00:00"
id: "atelier-ooyw"
issue_type: "task"
labels:
- "docs"
- "product"
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-3gr9"
  - kind: "issue"
    id: "atelier-3yoa"
  - kind: "issue"
    id: "atelier-a44d"
  - kind: "issue"
    id: "atelier-jeyr"
  - kind: "issue"
    id: "atelier-rdyl"
  - kind: "issue"
    id: "atelier-tpuc"
  - kind: "issue"
    id: "atelier-yn3u"
  - kind: "issue"
    id: "atelier-z0yu"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T19:09:38.582041956+00:00"
status: "done"
title: "Define mission completion model without closeout"
updated_at: "2026-06-16T19:09:38.582041956+00:00"
---

## Description

Update the target-state product contract for removing closeout as a product concept.

## Outcome

- `docs/product/zen.md`, `docs/product/work-model.md`, `docs/product/cli-surface.md`, `docs/product/validation.md`, `docs/architecture/quality/validation.md`, `PRODUCT_INTENT.md`, `CONTEXT.md`, and `AGENTFACTORY.md` agree on the new model.
- The model says issues and epics use normal issue workflow transitions for terminal work.
- The model says missions keep a simple built-in lifecycle and use shared terminal checks rather than a configurable mission workflow graph.
- The docs do not introduce readiness as a vague replacement term.

## Evidence

- `git diff` shows the terminology and model changes in the named docs.
- Command transcript from `rg -n "closeout|Closeout|--closeout" docs/product docs/architecture/quality PRODUCT_INTENT.md CONTEXT.md AGENTFACTORY.md` shows no live closeout guidance except intentional historical references.
- `atelier lint` and `git diff --check` pass.
