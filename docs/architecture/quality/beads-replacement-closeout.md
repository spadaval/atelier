# Beads Replacement Closeout

Closeout item: `atelier-z1p.7`

Parent mission: `atelier-z1p`

Date: 2026-06-08

## Evidence

- Repository cutover: commit `f64201d` (`Dogfood Atelier as live tracker`).
- Agent Factory tracker binding update: commit `db372b0` and external skill
  edits under `/root/.agents/skills/agent-factory`.
- Workflow validation: commit `32fedc6` and
  `docs/architecture/quality/agent-factory-atelier-validation.md`.
- Archive policy: `.beads/ARCHIVED.md` and `.beads/README.md`.

## Mission Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Atelier is the configured tracker in `AGENTFACTORY.md` for `/root/atelier`. | pass | `AGENTFACTORY.md` names Atelier, `.atelier-state/`, `.atelier/state.db`, and Atelier normal commands. |
| Current Beads issues, dependencies, statuses, labels, and notes are migrated or explicitly waived. | pass | `.atelier-state/` contains imported records with Beads aliases and source sections; `f64201d` added the canonical state projection from `.beads/issues.manual.jsonl`. |
| Agent Factory skill docs no longer assume Beads as the only tracker. | pass | `SKILL.md` routes through the repository-bound tracker, `standards/tracker.md` contains Atelier examples, and Beads command mechanics are isolated in `standards/beads.md` for legacy/archive use. |
| A real planning, update, and closeout workflow is executed through Atelier. | pass | `agent-factory-atelier-validation.md` records issue create, ready, show, update, dependency add/remove, notes, close, lint, export, rebuild, and sync proof. |
| Beads data is retained only as an archived fallback. | pass | `.beads/ARCHIVED.md` and `.beads/README.md` mark `.beads/` read-only recovery data; normal docs route tracker work through Atelier. |
| `bd doctor` is no longer required for normal work in this repo. | pass | Required health checks are `atelier export --check`, `atelier lint`, and `atelier doctor`; residue search found no normal-path `bd doctor` instruction outside historical/imported text or legacy-only Beads references. |

## Closeout Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Classify every mission success criterion. | pass | See `Mission Criteria` above. |
| Beads files and Dolt state are archived or removed according to documented policy. | pass | `.beads/` is retained read-only for recovery; no `.beads/.dolt` directory is present; retained Dolt metadata is inside the archived fallback. |
| Docs no longer instruct normal agents to use `bd` for `/root/atelier`. | pass | `AGENTS.md` says not to use `bd` for normal planning, execution, or handoff; `AGENTFACTORY.md` names Atelier commands. |
| Agent Factory skill changes are verified. | pass | Residue search after the tracker-neutral correction found no hard-coded Beads defaults outside `standards/beads.md`, the legacy-only reference. |
| Final validation evidence is linked from closeout notes. | pass | Closeout tracker notes link this document and `agent-factory-atelier-validation.md`. |

## Deferred Items

- `#45` (`Clarify unblock command success output`): deferred polish. Validation
  showed `atelier issue unblock` removes the dependency edge but prints wording
  that looks like the add path.

## Risks

- `atelier` is not installed on `PATH` in this shell; validation used
  `target/debug/atelier` after `cargo build`.
- Imported historical tracker text still contains Beads-era instructions by
  design. Normal-path docs and Agent Factory routing no longer depend on them.
