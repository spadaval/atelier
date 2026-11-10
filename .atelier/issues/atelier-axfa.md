---
created_at: "2026-11-10T00:56:08.066927831+00:00"
id: "atelier-axfa"
issue_type: "task"
labels:
- "redesign"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-fcb7"
    type: "advances"
  - kind: "issue"
    id: "atelier-yptz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Add `escalate` and `decide`, rebuttals in the brief, and settling deferred findings"
updated_at: "2026-11-10T03:58:04.771654927+00:00"
---

## Goal

Give a disagreement between implementer and reviewer its three exits in the tool.

## Done when

- [ ] Provisional: scenario tests cover a rebuttal the next reviewer withdraws; one it keeps; an escalation tied to a finding that a decider settles with `--defer`, after which `submit` passes, the next review shows the finding as decided and `verdict approve` is allowed; and `decide` refused for the implementer and for the reviewer who raised the finding. `list --deferred` shows the deferred findings not yet settled, across done items; `new --from <finding>` makes an item from one and writes `Taken:` into it, `drop <finding> --reason` writes `Dropped:`, and neither then appears in the list.

## Context

Preliminary. This step is planned in full at its boundary, with one planning review, as `docs/strategies/redesign-2-core.md` says. Target design: `docs/strategies/redesign-product.md` and `docs/strategies/redesign-architecture.md`.

## Unknowns

- How `status` shows an unanswered question to the lead.
