---
created_at: "2026-06-16T17:49:07.246642402+00:00"
id: "atelier-n4gp"
evidence_type: "validation"
captured_at: "2026-06-16T17:49:06.918739206+00:00"
command: "bash -lc 'rg -n \"workflow check|diagnostics slow|import-beads|atelier rebuild|atelier export|maintenance delete|branch status|worktree repair|worktree for-mission\" docs AGENTFACTORY.md /root/.agents/skills/agent-factory .atelier/issues/atelier-1xmi.md'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-1xmi"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 21795
    summary: ".atelier/issues/atelier-1xmi.md:30:- `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, `maintenance`, `branch`, and `worktree` are each classified as keep, hide/admin-frame, rename, fold into another command, remove, or defer.\n.atelier/issues/atelier-1xmi.md:46:Initial likely candidates: `rebuild` should follow `doctor --fix`; `workflow check` and `diagnostics slow` should stay hidden debug tools; `import-beads` should remain migration-only; `maintenance` should stay explicit danger-zone; `branch` should be demoted by the branch-lifecycle epic; `worktree` likely remains visible because it is real workspace management.\ndocs/adr/0004-work-lock-sync-policy.md:31:`atelier worktree for-mission` own the default ergonomic path:\ndocs/spec/storage/export/rebuild/canonical-layout.md:259:4. Run `atelier rebuild` if the local projection is stale or was rebuilt from\ndocs/spec/storage/export/rebuild/canonical-layout.md:628:Hidden/admin `atelier export` remains the deterministic check surface for\ndocs/spec/storage/export/rebuild/canonical-layout.md:632:Hidden/admin `atelier rebuild` recreates `.atelier/runtime/state.db` from\ndocs/spec/storage/export/rebuild/canonical-layout.md:636:surfaces; predecessor imports use `atelier import-beads`.\ndocs/adr/0002-markdown-first-record-store.md:49:Hidden/admin `atelier export` and `atelier export --check` remain compatibility\n/root/.agents/skills/agent-factory/procedures/install.md:119:  - `atelier export`\n/root/.agents/skills/agent-factory/procedures/install.md:120:  - `atelier export --check`\ndocs/spec/agent-factory/tracker-replacement-mvp.md:50:- rebuildable local projections refreshed by `atelier rebuild`;\ndocs/spec/agent-factory/tracker-replacement-mvp.md:51:- freshness and health commands such as `atelier export --check`,\ndocs/spec/agent-factory/tracker-replacement-mvp.md:136:| Export durable state | `bd export -o .beads/issues.manual.jsonl` | `atelier export` and `atelier export --output .atelier` | Print records written, output path, and whether derived files were regenerated. | Canonical `.atelier/` records and derived projections; `export --check` verifies freshness. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |\ndocs/spec/agent-factory/tracker-replacement-mvp.md:137:| Check export freshness before handoff | No exact Beads equivalent; Agent Factory uses backup plus Dolt status | `atelier export --check` | Print whether `.atelier/` is current. Stale files name changed, missing, or unexpected paths. | Supported freshness gate. Exit status, stale path names, and canonical records are the automation boundary. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |\ndocs/spec/agent-factory/tracker-replacement-mvp.md:138:| Rebuild runtime state after checkout | `bd bootstrap` / Dolt sync | `atelier rebuild` | Print source state dir, runtime DB path, record counts, and schema validation result. | Rebuildable ProjectionIndex and RuntimeState generated from `.atelier/`; the rebuilt SQLite database is local runtime state. | Yes | `atelier-fq9y` closed; cutover proof in `atelier-z1p.4` |\ndocs/spec/agent-factory/tracker-replacement-mvp.md:139:| Import current Beads data | `bd export` then `bd import` | `atelier import-beads .beads/issues.manual.jsonl` or `atelier import --format beads-jsonl ...` | Print imported, skipped, lossy, and failed counts. Preserve or report every source ID. | One-way import input and canonical `.atelier/` output; import reports are migration evidence, not an ongoing command-result API. | Yes | `atelier-z1p.2` |\ndocs/spec/agent-factory/tracker-replacement-mvp.md:140:| Push/pull tracker state | `bd dolt pull`, `bd dolt push`, `bd dolt status` | Git plus `.atelier/`: `git status`, `git pull`, `atelier rebuild`, `atelier export --check`, `git push` | Agent docs must state that committed `.atelier/` is the durable sync surface and SQLite is local runtime state. | Git moves committed canonical records; `atelier rebuild` recreates local projections; health/check commands detect dirty projection state. | Yes | `atelier-z1p.4` |\ndocs/spec/ag"
    truncated: true
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-1xmi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "hidden and admin commands are not taught as routine workflow paths"
updated_at: "2026-06-16T17:49:11.002912396+00:00"
---

hidden and admin commands are not taught as routine workflow paths

Command: bash -lc 'rg -n "workflow check|diagnostics slow|import-beads|atelier rebuild|atelier export|maintenance delete|branch status|worktree repair|worktree for-mission" docs AGENTFACTORY.md /root/.agents/skills/agent-factory .atelier/issues/atelier-1xmi.md'
Exit status: 0

Stdout summary (truncated):
.atelier/issues/atelier-1xmi.md:30:- `rebuild`, `workflow check`, `diagnostics slow`, `import-beads`, `maintenance`, `branch`, and `worktree` are each classified as keep, hide/admin-frame, rename, fold into another command, remove, or defer.
.atelier/issues/atelier-1xmi.md:46:Initial likely candidates: `rebuild` should follow `doctor --fix`; `workflow check` and `diagnostics slow` should stay hidden debug tools; `import-beads` should remain migration-only; `maintenance` should stay explicit danger-zone; `branch` should be demoted by the branch-lifecycle epic; `worktree` likely remains visible because it is real workspace management.
docs/adr/0004-work-lock-sync-policy.md:31:`atelier worktree for-mission` own the default ergonomic path:
docs/spec/storage/export/rebuild/canonical-layout.md:259:4. Run `atelier rebuild` if the local projection is stale or was rebuilt from
docs/spec/storage/export/rebuild/canonical-layout.md:628:Hidden/admin `atelier export` remains the deterministic check surface for
docs/spec/storage/export/rebuild/canonical-layout.md:632:Hidden/admin `atelier rebuild` recreates `.atelier/runtime/state.db` from
docs/spec/storage/export/rebuild/canonical-layout.md:636:surfaces; predecessor imports use `atelier import-beads`.
docs/adr/0002-markdown-first-record-store.md:49:Hidden/admin `atelier export` and `atelier export --check` remain compatibility
/root/.agents/skills/agent-factory/procedures/install.md:119:  - `atelier export`
/root/.agents/skills/agent-factory/procedures/install.md:120:  - `atelier export --check`
docs/spec/agent-factory/tracker-replacement-mvp.md:50:- rebuildable local projections refreshed by `atelier rebuild`;
docs/spec/agent-factory/tracker-replacement-mvp.md:51:- freshness and health commands such as `atelier export --check`,
docs/spec/agent-factory/tracker-replacement-mvp.md:136:| Export durable state | `bd export -o .beads/issues.manual.jsonl` | `atelier export` and `atelier export --output .atelier` | Print records written, output path, and whether derived files were regenerated. | Canonical `.atelier/` records and derived projections; `export --check` verifies freshness. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
docs/spec/agent-factory/tracker-replacement-mvp.md:137:| Check export freshness before handoff | No exact Beads equivalent; Agent Factory uses backup plus Dolt status | `atelier export --check` | Print whether `.atelier/` is current. Stale files name changed, missing, or unexpected paths. | Supported freshness gate. Exit status, stale path names, and canonical records are the automation boundary. | Yes | `atelier-ywow` closed; parity polish in `atelier-z1p.3` |
docs/spec/agent-factory/tracker-replacement-mvp.md:138:| Rebuild runtime state after checkout | `bd bootstrap` / Dolt sync | `atelier rebuild` | Print source state dir, runtime DB path, record counts, and schema validation result. | Rebuildable ProjectionIndex and RuntimeState generated from `.atelier/`; the rebuilt SQLite database is local runtime state. | Yes | `atelier-fq9y` closed; cutover proof in `atelier-z1p.4` |
docs/spec/agent-factory/tracker-replacement-mvp.md:139:| Import current Beads data | `bd export` then `bd import` | `atelier import-beads .beads/issues.manual.jsonl` or `atelier import --format beads-jsonl ...` | Print imported, skipped, lossy, and failed counts. Preserve or report every source ID. | One-way import input and canonical `.atelier/` output; import reports are migration evidence, not an ongoing command-result API. | Yes | `atelier-z1p.2` |
docs/spec/agent-factory/tracker-replacement-mvp.md:140:| Push/pull tracker state | `bd dolt pull`, `bd dolt push`, `bd dolt status` | Git plus `.atelier/`: `git status`, `git pull`, `atelier rebuild`, `atelier export --check`, `git push` | Agent docs must state that committed `.atelier/` is the durable sync surface and SQLite is local runtime state. | Git moves committed canonical records; `atelier rebuild` recreates local projections; health/check commands detect dirty projection state. | Yes | `atelier-z1p.4` |
docs/spec/ag

Stderr summary:
(none)

