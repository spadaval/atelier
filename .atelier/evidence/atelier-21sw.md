---
created_at: "2026-10-17T05:28:32.801354347+00:00"
id: "atelier-21sw"
evidence_type: "validation"
commit: "565c956e79e8313d95334db92ed4ed6119b2ee0f"
result: "pass"
captured_at: "2026-10-17T05:28:32.801258859+00:00"
path: ".agents/skills/agent-factory/procedures/orchestrate.md"
agent_identity: "orchestrator (Claude Opus 5.5)"
target:
  kind: "issue"
  id: "atelier-w56o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-w56o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Micro-test answers kept here for hand reading (supplements atelier-63ub; subagents were Sonnet at default reasoning effort). Control 1: order c,b,d; merge master into each in turn; regenerate Cargo.lock with cargo build; checks app/src-tauri only if shared crates changed; no cargo metadata --locked. Control 2: order c,b,d; same per-branch merge, regenerate with cargo metadata or build, app/src-tauri build in proof; no --locked check. Treatment 1: 'I don't touch any branch until it is next to land'; b and d back to back; cargo metadata --locked in root and app/src-tauri per branch. Treatment 2: checks readiness with git merge-tree first, leaves waiting branches alone, b and d back to back, cargo metadata --locked in both workspaces. After fixes 07c8d1c the back-to-back rule is removed (R2) and regenerated merges need review (R1); those two edits were not re-run in a micro-test. Vendored copy equals spadaval/.agents 07c8d1c (diff -rq clean)."
updated_at: "2026-10-17T05:28:32.801855682+00:00"
---

Micro-test answers kept here for hand reading (supplements atelier-63ub; subagents were Sonnet at default reasoning effort). Control 1: order c,b,d; merge master into each in turn; regenerate Cargo.lock with cargo build; checks app/src-tauri only if shared crates changed; no cargo metadata --locked. Control 2: order c,b,d; same per-branch merge, regenerate with cargo metadata or build, app/src-tauri build in proof; no --locked check. Treatment 1: 'I don't touch any branch until it is next to land'; b and d back to back; cargo metadata --locked in root and app/src-tauri per branch. Treatment 2: checks readiness with git merge-tree first, leaves waiting branches alone, b and d back to back, cargo metadata --locked in both workspaces. After fixes 07c8d1c the back-to-back rule is removed (R2) and regenerated merges need review (R1); those two edits were not re-run in a micro-test. Vendored copy equals spadaval/.agents 07c8d1c (diff -rq clean).
