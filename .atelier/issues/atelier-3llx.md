---
created_at: "2026-06-24T20:36:37.481824471+00:00"
id: "atelier-3llx"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-8c91"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-z0ll"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Wire terminal-aware color policy into command rendering"
updated_at: "2026-06-24T20:36:37.481824471+00:00"
---

## Description

Fix the color-policy proof gap from the command-surface/output audit. The product docs require automatic interactive color behavior, and `human_output::StylePolicy::from_context` exists, but production renderers still use plain style directly. Evidence cited by the audit: `docs/product/human-cli-output.md:320`, `crates/atelier-cli/src/human_output.rs:24`, and `crates/atelier-cli/src/commands/workflow.rs` render paths using `StylePolicy::plain()`.

Constraints:
- Domain/app services must not know about color or terminal detection; this is CLI/render boundary behavior.
- Color must reinforce text only; colorless output remains complete.

Risks:
- Leaving `StylePolicy::from_context` test-only lets missions claim output behavior that production never uses.

## Outcome

Production command rendering either uses a shared terminal-aware `StylePolicy` derived from terminal context and `NO_COLOR`, or the color behavior is explicitly deferred and product/mission validation claims stop saying it is implemented.

## Evidence

- Focused tests cover interactive/color-enabled, `NO_COLOR`, and non-interactive/plain behavior without making color the only signal.
- Search proof shows workflow renderers no longer hard-code `StylePolicy::plain()` where command context should choose style.
- `git diff --check` passes for the output and test changes.
