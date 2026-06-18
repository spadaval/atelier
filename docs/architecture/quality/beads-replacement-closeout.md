# Beads Replacement Closeout

Closeout item: `atelier-z1p.7`

Parent mission: `atelier-z1p`

Date: 2026-06-08

## Evidence

- Repository cutover: commit `f64201d` (`Dogfood Atelier as live tracker`).
- Agent Factory tracker binding update: commit `db372b0` and external skill
  edits that now live in the repository-owned copy at
  `.agents/skills/agent-factory`.
- Workflow validation: commit `32fedc6` and
  `docs/architecture/quality/agent-factory-atelier-validation.md`.
- Purge policy: Beads archive data was deleted after successful import,
  validation, and rebuild proof.

## Mission Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Atelier is the configured tracker in `AGENTS.md` for `/root/atelier`. | pass | `AGENTS.md` names Atelier, `.atelier/`, `.atelier/runtime/state.db`, and Atelier normal commands. |
| Current Beads issues, dependencies, statuses, labels, and notes are migrated or explicitly waived. | pass | `.atelier/` contains the imported records as neutral Atelier issues; Beads source aliases and source sections were removed after validation. |
| Agent Factory skill docs no longer assume Beads as the only tracker. | pass | `SKILL.md` routes through the repository tracker and `AGENTS.md` names only Atelier commands for normal work. |
| A real planning, update, and closeout workflow is executed through Atelier. | pass | `agent-factory-atelier-validation.md` records issue create, ready, show, update, dependency add/remove, notes, close, lint, historical export/rebuild diagnostics, and sync proof. |
| Beads data is kept after import. | superseded | The retention policy was retired. The old archive was purged after the canonical Atelier state proved self-sufficient. |
| Predecessor tracker health checks are no longer required for normal work in this repo. | pass | Current normal health checks are `atelier lint` and `atelier doctor`; storage-rendering diagnostics such as `atelier export --check` are no longer routine workflow proof. |

## Closeout Criteria

| Criterion | Result | Evidence |
| --- | --- | --- |
| Classify every mission success criterion. | pass | See `Mission Criteria` above. |
| Beads files and Dolt state are archived or removed according to documented policy. | pass | The policy is now purge-after-validation: the old archive and retained Dolt metadata are deleted from the repository. |
| Docs no longer instruct normal agents to use the predecessor CLI for `/root/atelier`. | pass | `AGENTS.md` prohibits the predecessor CLI for planning, execution, handoff, and recovery; `AGENTS.md` names Atelier commands. |
| Agent Factory skill changes are verified. | pass | Residue search after the tracker-neutral correction found no hard-coded Beads defaults in repository workflow bindings. |
| Final validation evidence is linked from closeout notes. | pass | Closeout tracker notes link this document and `agent-factory-atelier-validation.md`. |

## Deferred Items

- None. `#45` (`Clarify unblock command success output`) was closed by giving
  dependency add/remove distinct text and JSON action/state fields.

## Risks

- Imported historical tracker text still contains Beads-era instructions where
  the issue itself describes the replacement mission. Normal-path docs and
  Agent Factory routing do not depend on them.
