# Agent Factory Atelier Validation

Validation item: `atelier-z1p.6`

Date: 2026-06-08

Scenario: prove Agent Factory can operate through Atelier after the repository
and skill bindings were updated.

## Historical Evidence Summary (Non-Normative)

This section preserves the command transcript from the 2026-06-08 replacement
exercise. Its numeric IDs and removed start, direct-close, lint, doctor,
export, and rebuild paths are historical evidence only; they are not current
operator guidance or compatibility aliases.

- Start gate: `atelier issue show atelier-z1p.6` and `atelier issue show
  atelier-z1p` showed the assigned validation item and parent mission.
- Planning/docs workflow: `atelier issue create` created issue `#43` under
  `atelier-z1p.6`; `atelier issue note 43 ...` annotated it;
  `atelier start 43` established active local work; `atelier issue close 43
  --reason ...` closed it after this note was added.
- Ready discovery: `atelier work ready` listed `atelier-z1p.6` as ready after
  its blockers were closed.
- Dependency workflow: `atelier issue link 43 44` added a blocker; `atelier
  issue show 43` displayed `Blocked by: #44`; `atelier issue unlink 43 44`
  removed the edge; a follow-up `show` displayed no blockers.
- Close workflow: `atelier issue close 44 --reason ...` closed the dependency
  fixture task; `atelier issue close 43 --reason ...` closed the docs task.
- Historical health checks: `atelier lint atelier-z1p.6`,
  `atelier export --check`, and `atelier doctor` passed during the original
  validation. These names do not define current normal proof.
- Historical storage-rendering check: `/tmp/atelier-rebuild-check` was populated with
  `.atelier` and an empty `.atelier`; from that directory, `atelier
  rebuild --input .atelier` rebuilt `/tmp/atelier-rebuild-check/.atelier/runtime/state.db`,
  and `atelier export --check` confirmed freshness.
- Historical final freshness recovery: after closing `atelier-z1p.6`, `atelier export
  --check` reported stale `ISS-0041.md` and `manifest.json`; `atelier export`
  refreshed the canonical projection.

## Current Equivalent Workflow

Current Agent Factory validation uses human command output and explicit
drill-down commands:

- inspect work with `atelier issue show <id>` and inspect available lifecycle
  actions with `atelier issue transition <id>`;
- enter active work and close it by executing the configured transitions shown
  by that command, such as `atelier issue transition <id> start` and
  `atelier issue transition <id> close --reason "..."` in this repository;
- manage blockers with
  `atelier issue link <blocked-id> <blocker-id> --role blocked_by` and
  `atelier issue unlink <blocked-id> <blocker-id> --role blocked_by`;
- validate canonical records and workflow configuration with `atelier check`,
  using `atelier check --fix` only when ignored local state requires repair;
- capture claim-specific proof with `atelier evidence record` and use quiet
  acknowledgements only where a command naturally returns one result.

Do not use command-result `--json` as workflow proof. Low-level export and
rebuild diagnostics remain implementation or migration probes, not normal
handoff, health, or recovery guidance.

## Historical Failure Classifications

- Fixed in validation scope at the time: stale canonical projection after
  closing `atelier-z1p.6`; refreshed with the then-current export diagnostic and
  rechecked. Current normal local repair is `atelier check --fix`.
- Deferred with owner: `atelier` was not on PATH in the shell; validation used
  `/root/atelier/target/debug` on PATH. Owner: repository operator or install
  story follow-up.
- Deferred with owner: `atelier issue unlink` removed the dependency edge but
  printed the same wording as `block` (`#43 is blocked by #44 (changed)`).
  Owner: `#45` under `atelier-z1p.7` (`supadava@cisco.com`).
- Not applicable: direct Beads validation; this scenario intentionally used
  Atelier only.
