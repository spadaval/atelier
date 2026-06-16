---
created_at: "2026-06-16T15:59:16.078833726+00:00"
id: "atelier-k1ga"
issue_type: "epic"
labels:
- "cli"
- "dependencies"
- "workflow"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-3s9y"
  - kind: "issue"
    id: "atelier-d226"
  - kind: "issue"
    id: "atelier-em15"
  - kind: "issue"
    id: "atelier-f7vd"
  - kind: "issue"
    id: "atelier-kswx"
  - kind: "issue"
    id: "atelier-kzfl"
  - kind: "issue"
    id: "atelier-nqjc"
  - kind: "issue"
    id: "atelier-qh52"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-16T19:03:14.608736104+00:00"
status: "done"
title: "Epic: Sort work views by blocker dependency order"
updated_at: "2026-06-16T19:03:14.608736104+00:00"
---

## Description

Atelier's work views currently sort many issue rows by ID, status, or local priority. That makes blocker chains harder to execute: a validation item, dependent feature, or later task can appear before the work it is waiting on.

Make the normal work-selection surfaces blocker-aware. When a view displays a set of issue-like work, visible blockers should appear before the work they unblock, and each row should communicate an action state such as ready, blocked, active, review, validation, or done. Commands should stop leaking duplicate workflow internals like `todo/todo` and stop using long inline blocker-ID suffixes as the primary state signal. This is guidance at the friction point, not another process gate.

## Outcome

- A shared ordering contract exists for visible issue sets: open blockers in the same visible set appear before blocked items, closed blockers no longer act as prerequisites, and blockers outside the visible set remain visible as blocker cues rather than phantom rows.
- `atelier issue list`, `atelier issue show`, `atelier graph tree`, `atelier mission status`, `atelier mission list`, and root `atelier status` use the shared blocker-aware ordering where they display issue rows or derive next-work suggestions.
- Work rows use a single readable state label instead of duplicate `category/status` tokens, and blocked rows show blocked state with a count or drill-down cue rather than relying on an inline `blocked by <id>` suffix.
- Detail and drill-down surfaces still make exact blocker IDs available when an operator needs to inspect or resolve blockers.
- Tie breakers remain deterministic and useful after dependency order is applied, using workflow category/status, priority, update time, ID, or title only where they do not violate visible blocker order.
- Invalid or cyclic dependency data cannot panic or hide work; the affected commands report or degrade deterministically while preserving enough IDs for repair.
- Product docs and tests make the ordering contract clear enough for future command surfaces to reuse it instead of inventing local sort rules.

## Evidence

- Child issue proof maps each affected command surface to a focused transcript or test showing blockers before blocked work.
- Integrated validation evidence exercises one representative dependency graph across issue queues, issue detail, graph tree, mission status, mission list, and root status.
- Integrated validation evidence shows default work rows expose readable action state and do not print duplicate category/status tokens.
- Contract or documentation file diff describes the shared ordering rule and its non-goals.
- `atelier lint`, `atelier export --check`, `cargo fmt -- --check`, focused cargo tests, and `git diff --check` pass.

## Notes

This work should make the next action more obvious without making every command a full scheduler. Mission-level rows remain mission-level rows; issue-level order only appears where the surface already displays or summarizes issue work.
