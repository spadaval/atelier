---
created_at: "2026-06-14T02:48:02.762747950+00:00"
id: "atelier-hy2i"
labels:
- "mission"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-a625"
    type: "advances"
  - kind: "issue"
    id: "atelier-n5ar"
    type: "advances"
  - kind: "issue"
    id: "atelier-oknl"
    type: "advances"
  - kind: "issue"
    id: "atelier-omlz"
    type: "advances"
  - kind: "issue"
    id: "atelier-zah8"
    type: "advances"
  - kind: "issue"
    id: "atelier-4p7q"
    type: "advances"
schema: "atelier.mission"
schema_version: 1
status: "ready"
title: "Turn Codex mission log insights into agent operability improvements"
updated_at: "2026-06-14T02:53:36.538227701+00:00"
---

## Intent

Turn the 2026-06-14 Codex mission-log findings into durable improvements across repository documentation, Agent Factory guidance, and the Atelier CLI. Future long-running mission agents should orient from documented command and source maps, recover from stale tracker state through clear CLI guidance, use reliable worktree/projection flows, and close work with proof matched to the claim being closed.

## Constraints

- Preserve human-first command output; do not add compatibility aliases or old-command shims unless explicitly approved.
- Use docs/architecture/quality/codex-mission-log-insights-2026-06-14.md as the source insight report.
- Treat existing open issues that already match a finding as mission work rather than duplicating them.

## Risks

- Some findings overlap recently completed stabilization missions; duplicate work should be merged or closed during planning rather than reintroducing old command models.
- CLI recovery improvements can accidentally become compatibility shims if corrective messages preserve removed behavior instead of pointing at the new path.

## Validation

- The insight report is mapped to linked epics/issues covering repository docs, Agent Factory guidance, and Atelier CLI improvements.
- Command/help docs match current CLI behavior for mission, issue, worktree, evidence, workflow, and health flows.
- Agent Factory guidance includes stale-state, command-surface, shell, checkout, and proof safeguards.
- Focused CLI tests prove corrective errors, stale-state recovery, worktree behavior, and closeout proof matching where implemented.
- Mission closeout records atelier lint, atelier doctor, relevant focused tests, and git diff --check proof; low-level export/rebuild diagnostics are recorded only when a failure specifically requires them.
