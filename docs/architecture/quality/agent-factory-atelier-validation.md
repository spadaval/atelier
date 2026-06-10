# Agent Factory Atelier Validation

Validation item: `atelier-z1p.6`

Date: 2026-06-08

Scenario: prove Agent Factory can operate through Atelier after the repository
and skill bindings were updated.

## Evidence Summary

- Start gate: `atelier issue show atelier-z1p.6` and `atelier issue show
  atelier-z1p` showed the assigned validation item and parent mission.
- Planning/docs workflow: `atelier issue create` created issue `#43` under
  `atelier-z1p.6`; `atelier issue update 43 --claim --status in_progress
  --append-notes ...` claimed and updated it; `atelier issue close 43
  --reason ...` closed it after this note was added.
- Ready discovery: `atelier issue ready` listed `atelier-z1p.6` as ready after
  its blockers were closed.
- Dependency workflow: `atelier issue block 43 44` added a blocker; `atelier
  issue show 43` displayed `Blocked by: #44`; `atelier issue unblock 43 44`
  removed the edge; a follow-up `show` displayed no blockers.
- Close workflow: `atelier issue close 44 --reason ...` closed the dependency
  fixture task; `atelier issue close 43 --reason ...` closed the docs task.
- Health checks: `atelier lint atelier-z1p.6`, `atelier export --check`, and
  `atelier doctor` passed. The historical `atelier sync` proof is superseded by
  Git plus canonical export/rebuild checks.
- Clean rebuild: `/tmp/atelier-rebuild-check` was populated with
  `.atelier-state` and an empty `.atelier`; from that directory, `atelier
  rebuild --input .atelier-state` rebuilt `/tmp/atelier-rebuild-check/.atelier/state.db`,
  and `atelier export --check` confirmed freshness.
- Final freshness recovery: after closing `atelier-z1p.6`, `atelier export
  --check` reported stale `ISS-0041.md` and `manifest.json`; `atelier export`
  refreshed the canonical projection.

## Failure Classifications

- Fixed in validation scope: stale canonical projection after closing
  `atelier-z1p.6`; refreshed with `atelier export` and rechecked.
- Deferred with owner: `atelier` was not on PATH in the shell; validation used
  `/root/atelier/target/debug` on PATH. Owner: repository operator or install
  story follow-up.
- Deferred with owner: `atelier issue unblock` removed the dependency edge but
  printed the same wording as `block` (`#43 is blocked by #44 (changed)`).
  Owner: `#45` under `atelier-z1p.7` (`supadava@cisco.com`).
- Not applicable: direct Beads validation; this scenario intentionally used
  Atelier only.
