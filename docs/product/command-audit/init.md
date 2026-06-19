# `atelier init`

Primary role: Admin.

Primary question: "How do I create or reconcile the Atelier project state in
this repository?"

## Assessment

- Name: Correct. `init` is the expected setup verb.
- Documentation: Mostly correct. It should stay in setup/admin guidance and
  should not be presented as ordinary workflow after a repository is initialized.
- Design: Correct if it writes only tracker scaffolding, workflow config, local
  runtime storage, and ignore rules. `--import-beads` is acceptable as explicit
  migration input, but the command must not silently import predecessor state.
- Output hierarchy: Created/reused paths first, migration detection second,
  verification commands last. It should point to `atelier lint`,
  `atelier man admin`, and `atelier status` before issue creation.

## Operator Assessment

| Form | Persona | Likely use cases | Information wanted | Likely next action | Guidance/orientation |
| --- | --- | --- | --- | --- | --- |
| `atelier init` | Admin, bootstrap agent | Initialize first-class Atelier state; create canonical tracker scaffolding; prepare a fresh checkout for agent work. | Created vs reused paths, tracked vs ignored state, config/workflow locations, and first verification command. | Run `atelier lint`, `atelier man admin`, then `atelier status`. | Good setup verb. Current implementation should orient before nudging issue creation. |
| `atelier init --force` | Admin | Reconcile a partial setup; refresh core tracker scaffolding; recover from interrupted setup. | Exactly what was reused, overwritten, repaired, or preserved. | Run `atelier lint` and `atelier doctor`. | Good if output keeps mutation scope explicit. |
| `atelier init --import-beads` | Admin/migration operator | Import repo-local predecessor data; perform explicit migration during setup. | Input path, imported counts, skipped/conflicting records, resulting canonical paths. | Run `atelier lint`, inspect imported work, then `atelier status`. | Correctly explicit; predecessor import must not be silent default behavior. |

## Guidance Finding

The current setup guidance should prefer orientation before creation. The command
implementation prints `atelier issue create "Task"` before `atelier man admin`
and does not name `atelier status`; that can push a fresh operator toward
creating work before checking tracker health and role guidance.
