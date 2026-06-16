# Agent Factory Atelier Validation

Validation item: `atelier-z1p.6`

Date: 2026-06-08

Scenario: prove Agent Factory can operate through Atelier after the repository
and skill bindings were updated.

## Evidence Summary

- Start gate: `atelier issue show atelier-z1p.6` and `atelier issue show
  atelier-z1p` showed the assigned validation item and parent mission.
- Planning/docs workflow: `atelier issue create` created issue `#43` under
  `atelier-z1p.6`; `atelier issue note 43 ...` annotated it;
  `atelier start 43` established active local work; `atelier issue close 43
  --reason ...` closed it after this note was added.
- Ready discovery: `atelier issue list --ready` listed `atelier-z1p.6` as ready after
  its blockers were closed.
- Dependency workflow: `atelier issue block 43 44` added a blocker; `atelier
  issue show 43` displayed `Blocked by: #44`; `atelier issue unblock 43 44`
  removed the edge; a follow-up `show` displayed no blockers.
- Close workflow: `atelier issue close 44 --reason ...` closed the dependency
  fixture task; `atelier issue close 43 --reason ...` closed the docs task.
- Historical health checks: `atelier lint atelier-z1p.6`,
  `atelier export --check`, and `atelier doctor` passed during the original
  validation. Current normal proof uses `atelier lint`, `atelier doctor`, and
  claim-specific tests; export/rebuild diagnostics are storage-rendering or
  migration/debug proof only.
- Current Agent Factory validation should inspect and close work through human
  command output, quiet acknowledgements where a command naturally returns a
  single result, and explicit drill-down commands such as `atelier issue show
  <id>`, `atelier mission show <id>`, `atelier mission status <id>`,
  `atelier lint` and `atelier doctor`. Do not use command-result `--json` as
  the workflow proof.
- Historical storage-rendering check: `/tmp/atelier-rebuild-check` was populated with
  `.atelier` and an empty `.atelier`; from that directory, `atelier
  rebuild --input .atelier` rebuilt `/tmp/atelier-rebuild-check/.atelier/runtime/state.db`,
  and `atelier export --check` confirmed freshness.
- Historical final freshness recovery: after closing `atelier-z1p.6`, `atelier export
  --check` reported stale `ISS-0041.md` and `manifest.json`; `atelier export`
  refreshed the canonical projection.

## Failure Classifications

- Fixed in validation scope at the time: stale canonical projection after
  closing `atelier-z1p.6`; refreshed with the then-current export diagnostic and
  rechecked. Current normal local repair is `atelier doctor --fix`.
- Deferred with owner: `atelier` was not on PATH in the shell; validation used
  `/root/atelier/target/debug` on PATH. Owner: repository operator or install
  story follow-up.
- Deferred with owner: `atelier issue unblock` removed the dependency edge but
  printed the same wording as `block` (`#43 is blocked by #44 (changed)`).
  Owner: `#45` under `atelier-z1p.7` (`supadava@cisco.com`).
- Not applicable: direct Beads validation; this scenario intentionally used
  Atelier only.
